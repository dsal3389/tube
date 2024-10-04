use std::fmt;

use crate::ffi;

#[derive(Debug)]
pub enum Errno {
    ENONET,
    EAGAIN,
    EINVAL,
    Other(i32),
}

impl Errno {
    pub fn new() -> Self {
        let errno = unsafe { *ffi::__errno_location() };
        Self::from(errno)
    }
}

impl fmt::Display for Errno {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ENONET => write!(
                f,
                "A directory component in pathname does not exists or is a dangling symbolic link."
            ),
            Self::EAGAIN => write!(f, "Resouce temporarily unavailable."),
            Self::EINVAL => write!(f, "Invalid argument."),
            Self::Other(o) => write!(f, "unexpected errno `{}`.", o),
        }
    }
}

impl std::error::Error for Errno {}

impl From<i32> for Errno {
    fn from(value: i32) -> Self {
        match value {
            2 => Self::ENONET,
            11 => Self::EAGAIN,
            22 => Self::EINVAL,
            o => Self::Other(o),
        }
    }
}

impl From<&Errno> for i32 {
    fn from(value: &Errno) -> Self {
        todo!()
    }
}
