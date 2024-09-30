use futures::stream::Stream;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::os::raw::c_int;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::task::{Context, Poll};

mod ffi;

use ffi::inotify_rm_watch;

/// the `InotifyError` enum contains
/// all error variants possible in the Inotify API
#[derive(Debug, PartialEq)]
pub enum InotifyError {
    InitError,
    NotDirectory { path: PathBuf },
    ENVAL,
    EMFILE,
    ENOMEM,
}

impl fmt::Display for InotifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InitError => write!(f, "inotify init error"),
            Self::NotDirectory { path } => write!(f, "given path `{:?}` is not a directory", path),
            Self::ENVAL => write!(f, "An invalid value was specified in flags"),
            Self::EMFILE => write!(
                f,
                "the user limit on the total number of inotify instances has been reached"
            ),
            Self::ENOMEM => write!(f, "Insufficient kernel memory is available"),
        }
    }
}

impl Error for InotifyError {}

/// Inotify struct contians the information about
/// the invoked InotifyError,
/// this method types is builder pattern
#[derive(Debug)]
pub struct Inotify {
    fd: c_int,
    watchers: HashMap<i32, InotifyWatch>,
}

/// InotifyWatch, a high level class that holds the inotify watcher
/// information like the inotify file descriptor, watcher descriptor
/// and the path that watcher is watching
#[derive(Debug)]
pub struct InotifyWatch {
    fd: c_int,
    wd: c_int,
    pathname: PathBuf,
}

pub struct InotifyEvent {
    wd: i32,
    mask: u32,
    cookie: u32,
    len: u32,
}

impl Inotify {
    pub fn new() -> Result<Self, InotifyError> {
        let fd = unsafe { ffi::inotify_init() };
        if fd == -1 {
            return Err(InotifyError::InitError);
        }
        Ok(Self {
            fd,
            watchers: HashMap::new(),
        })
    }

    /// addes a path to the inotify watch event via `inotify_add_watch`
    pub fn watch(mut self, pathname: PathBuf) -> Result<Self, InotifyError> {
        let wd = unsafe {
            ffi::inotify_add_watch(self.fd, pathname.to_str().unwrap().as_ptr() as *const i8, 0)
        };
        let watcher = InotifyWatch {
            fd: self.fd,
            wd,
            pathname,
        };
        self.watchers.insert(wd, watcher);
        Ok(self)
    }

    /// this function is for directories, since I notify is not recursive
    /// listening for events on directories won't trigger events for sub directories
    /// the depth arguments defines how deep the recurse the given directory, if None is given
    /// then there is no limit
    pub fn watch_recursive(
        mut self,
        pathname: PathBuf,
        depth: Option<u32>,
    ) -> Result<Self, InotifyError> {
        if !pathname.is_dir() {
            return Err(InotifyError::NotDirectory { path: pathname });
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

    /// returns reference to the `watch` instance by the given
    /// watch descriptor
    pub fn get_watch_by_descriptor(&self, wd: i32) -> Option<&InotifyWatch> {
        self.watchers.get(&wd)
    }
}

impl InotifyWatch {
    /// returns the defined path for the watcher
    pub fn pathname(&self) -> &Path {
        &self.pathname
    }
}

impl Stream for Inotify {
    type Item = InotifyEvent;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        unsafe {
            let mut event_ptr = [0u8; 1];
            let read_len = ffi::read(self.fd, event_ptr.as_mut_ptr(), 1);
        }
        todo!()
    }
}

/// octal formatting for Inotify returns the Inotify file descriptor
impl fmt::Octal for Inotify {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.fd)
    }
}

/// octal formatting for InotifyWatch returns the watch descriptor
impl fmt::Octal for InotifyWatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.wd)
    }
}

/// default formatting for InotifyWatch returns the watch path
impl fmt::Display for InotifyWatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.pathname)
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

impl Drop for InotifyWatch {
    fn drop(&mut self) {
        unsafe {
            inotify_rm_watch(self.fd, self.wd);
        }
    }
}
