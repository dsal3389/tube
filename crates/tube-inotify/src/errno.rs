use std::fmt;

#[derive(Debug)]
pub enum Errno {
    ENONET,
    EINTER,
    EAGAIN,
    EINVAL,
    Other(i32),
}

impl Errno {
    /// returns the last os error
    pub fn last() -> Self {
        Self::from(std::io::Error::last_os_error().raw_os_error().unwrap())
    }
}

impl fmt::Display for Errno {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ENONET => write!(f, ""),
            Self::EINTER => write!(f, ""),
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
            4 => Self::EINTER,
            11 => Self::EAGAIN,
            22 => Self::EINVAL,
            o => Self::Other(o),
        }
    }
}
