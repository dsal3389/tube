use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::os::raw::c_int;
use std::path::PathBuf;

mod ffi;

use ffi::inotify_rm_watch;

/// the `InotifyError` enum contains
/// all error variants possible in the Inotify API
#[derive(Debug, PartialEq)]
pub enum InotifyError {
    InitError,
    ENVAL,
    EMFILE,
    ENOMEM,
}

impl fmt::Display for InotifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InitError => write!(f, "inotify init error"),
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
/// the invoked InotifyError
///
/// this method types is builder pattern
#[derive(Debug)]
pub struct Inotify {
    fd: c_int,
    watchers: HashMap<c_int, InotifyWatch>,
}

/// InotifyWatch cannot be created directly, it can only
/// be created with the `Inotify` methods.
///
///
#[derive(Debug)]
pub struct InotifyWatch {
    fd: c_int,
    wd: c_int,
    pathname: PathBuf,
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

    /// watch the specified pathname, method takes ownershipt of `self`
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
}

/// octal formatting returns the Inotify file descriptor
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

/// clean all resources when inotify is dropped, also drop
/// the watchers
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
