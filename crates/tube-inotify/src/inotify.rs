use futures::stream::Stream;
use std::collections::HashMap;
use std::fmt;
use std::os::fd::{AsRawFd, RawFd};
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::errno::Errno;
use crate::ffi;

pub const SYSCALL_ERROR: i32 = -1;

/// a opaque struct that defines consts that can be used
/// as flags with bitwise operations
pub struct Mask;

impl Mask {
    pub const OPEN: u32 = ffi::IN_OPEN;
    pub const CLOSE: u32 = ffi::IN_CLOSE;
}

pub struct Flag;

impl Flag {
    pub const NONBLOCKING: i32 = ffi::IN_NONBLOCK;
}

#[derive(Debug)]
pub struct InotifyEvent {
    wd: RawFd,
    mask: u32,
    cookie: u32,
    name: String,
}

impl InotifyEvent {
    fn new(wd: RawFd, mask: u32, cookie: u32, name: String) -> Self {
        Self {
            wd,
            mask,
            cookie,
            name,
        }
    }

    /// returns `InotifyEvent` from given silice, because the `name` field can be dynamic
    /// function also returns the size in bytes of the event, in case the original buffer
    /// contains multiple events and the caller to `from_buffer` need to know the size in buffer
    /// of the returned event
    fn from_buffer(buffer: &[u8]) -> (usize, Self) {
        let event_size = std::mem::size_of::<ffi::inotify_event>();
        let ptr = buffer.as_ptr() as *const ffi::inotify_event;
        assert!(buffer.len() >= event_size);

        let ffi_event = unsafe { ptr.read() };

        // index to the last byte in the buffer, the `ffi_event.len` defines
        // the length of `name` field, which is dynamic size and part of the event
        let event_end = event_size + ffi_event.len as usize;

        // the name is an optional field that is defined at the end of the event buffer,
        // the `ffi_event.len` defines the length of the name string, so we
        // take a slice from the end of the event until the event_size + len
        // which should be the end of name string
        let name_bytes = &buffer[event_size..event_end];

        // convert the string to a higher level `String`
        let name = String::from_utf8(name_bytes.into()).unwrap();
        let event = Self::new(ffi_event.wd, ffi_event.mask, ffi_event.cookie, name);
        (event_end, event)
    }
}

/// a type that holds buffer returned by `read` that should contain
/// multiple `InotifyEvent`s
#[derive(Debug)]
pub struct InotifyEventBatch<const N: usize> {
    buffer: [u8; N],
    num_bytes: usize,
    pos: usize,
}

impl<const N: usize> InotifyEventBatch<N> {
    fn new(buffer: [u8; N], num_bytes: usize) -> Self {
        Self {
            buffer,
            num_bytes,
            pos: 0,
        }
    }
}

/// iterates over the events found in the given buffer
impl<const N: usize> Iterator for InotifyEventBatch<N> {
    type Item = InotifyEvent;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.num_bytes {
            return None;
        }

        let (size, event) = InotifyEvent::from_buffer(&self.buffer[self.pos..]);
        self.pos += size;
        Some(event)
    }
}

/// Inotify struct contians the information about
/// the invoked InotifyError,
/// this method types is builder pattern
pub struct Inotify {
    fd: RawFd,
    watchers: HashMap<RawFd, PathBuf>,
}

impl Inotify {
    /// returns new `Inotify` with `inotify_init1` syscall and passing
    /// the `flags` to the syscall
    pub fn with_flags(flags: i32) -> Result<Self, Errno> {
        match unsafe { ffi::inotify_init1(flags) } {
            SYSCALL_ERROR => Err(Errno::last()),
            fd => Ok(Self {
                fd,
                watchers: HashMap::new(),
            }),
        }
    }

    /// addes a path to the inotify watch event via `inotify_add_watch`
    pub fn watch(mut self, pathname: PathBuf, mask: u32) -> Result<Self, Errno> {
        let wd = unsafe {
            ffi::inotify_add_watch(
                self.fd,
                pathname.to_str().unwrap().as_ptr() as *const i8,
                mask,
            )
        };
        match wd {
            SYSCALL_ERROR => Err(Errno::last()),
            _ => {
                self.watchers.insert(wd, pathname);
                Ok(self)
            }
        }
    }

    /// this function is for directories, since I notify is not recursive
    /// listening for events on directories won't trigger events for sub directories
    /// the depth arguments defines how deep the recurse the given directory, if None is given
    /// then there is no limit
    pub fn watch_recursive(mut self, pathname: PathBuf, depth: Option<u32>) -> Result<Self, Errno> {
        if !pathname.is_dir() {
            return Err(Errno::from(0));
        }
        for entry in pathname.read_dir().expect("couldn't read directory") {
            match entry {
                Ok(entry) if entry.path().is_dir() => {
                    // add the directory to the watch list and decrease the depths by one
                    self = self.watch_recursive(entry.path(), depth.and_then(|n| Some(n - 1)))?;
                }
                _ => todo!(),
            }
        }
        Ok(self)
    }

    /// returns the defined path for given watch descriptor
    pub fn path_for_watch(&self, wd: RawFd) -> Option<&Path> {
        self.watchers.get(&wd).and_then(|p| Some(p.as_path()))
    }

    /// checks if event is ready on the inotify descriptor by using the
    /// `poll` syscall
    fn events_ready(&self) -> Result<bool, Errno> {
        let mut fds = [ffi::pollfd {
            fd: self.fd,
            events: ffi::POLLIN,
            revents: 0,
        }; 1];
        match unsafe { ffi::poll(fds.as_mut_ptr(), 1, -1) } {
            SYSCALL_ERROR => Err(Errno::last()),
            ret if ret < 0 => {
                panic!(
                    "poll file descriptor returned unexpected status code `{}`",
                    ret
                )
            }
            ret => Ok(ret != 0 && fds[0].revents & (ffi::POLLIN as i16) != 0),
        }
    }
}

impl Stream for Inotify {
    type Item = Result<InotifyEventBatch<4096>, Errno>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let events_ready = self.events_ready();

        if events_ready.is_err() {
            return Poll::Ready(Some(Err(unsafe { events_ready.unwrap_err_unchecked() })));
        }

        if !unsafe { events_ready.unwrap_unchecked() } {
            return Poll::Pending;
        }

        let mut buffer = [0u8; 4096];
        let bytes_read = unsafe { ffi::read(self.fd, buffer.as_mut_ptr(), buffer.len()) };

        cx.waker().wake_by_ref();
        Poll::Ready(Some(Ok(InotifyEventBatch::new(
            buffer,
            bytes_read as usize,
        ))))
    }
}

impl AsRawFd for Inotify {
    fn as_raw_fd(&self) -> RawFd {
        self.fd
    }
}

/// octal formatting for Inotify returns the Inotify file descriptor
impl fmt::Octal for Inotify {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.fd)
    }
}

/// clean all resources when inotify is dropped, also drop the watchers
impl Drop for Inotify {
    fn drop(&mut self) {
        self.watchers.drain();
        unsafe {
            ffi::close(self.fd);
        }
    }
}
