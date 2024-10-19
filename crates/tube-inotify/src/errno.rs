use std::fmt;

#[derive(Debug)]
pub struct Errno(i32);

/// contains all errno values that can be found
/// in C, represent them as rust enum
pub enum ErrnoKind {
    EPERM,
    ENOENT,
    ESRCH,
    EINTER,
    EIO,
    ENXIO,
    E2BIG,
    ENOEXEC,
    EBADF,
    ECHILD,
    EAGAIN,
    ENOMEM,
    EACCES,
    EFAULT,
    ENOTBLK,
    EBUSY,
    EEXIST,
    EXDEV,
    ENODEV,
    ENOTDIR,
    EISDIR,
    EINVAL,
    ENFILE,
    EMFILE,
    ENOTTY,
    ETXTBSY,
    EFBIG,
    ENOSPC,
    ESPIPE,
    EROFS,
    EMLINK,
    EPIPE,
    EDOM,
    ERANGE,
    Unknown,
}

impl Errno {
    /// creates Errno instance from the last os error
    pub fn last() -> Self {
        Self::from(std::io::Error::last_os_error().raw_os_error().unwrap())
    }

    /// returns value of `ErrnoKind` from the current
    /// errno code
    pub fn kind(&self) -> ErrnoKind {
        ErrnoKind::from(self)
    }
}

impl fmt::Display for Errno {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind(), "")
    }
}

impl fmt::Display for ErrnoKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EPERM => write!(f, "EPERM"),
            Self::ENOENT => write!(f, "ENOENT"),
            Self::ESRCH => write!(f, "ESRCH"),
            Self::EINTER => write!(f, "EINTER"),
            Self::EIO => write!(f, "EIO"),
            Self::ENXIO => write!(f, "ENXIO"),
            Self::E2BIG => write!(f, "E2BIG"),
            Self::ENOEXEC => write!(f, "ENOEXEC"),
            Self::EBADF => write!(f, "EBADF"),
            Self::ECHILD => write!(f, "ECHILD"),
            Self::EAGAIN => write!(f, "EAGAIN"),
            Self::ENOMEM => write!(f, "ENOMEM"),
            Self::EACCES => write!(f, "EACCESS"),
            Self::EFAULT => write!(f, "EFAULT"),
            Self::ENOTBLK => write!(f, "ENOTBLK"),
            Self::EBUSY => write!(f, "EBUSY"),
            Self::EEXIST => write!(f, "EEXIST"),
            Self::EXDEV => write!(f, "EXDEV"),
            Self::ENODEV => write!(f, "ENODEV"),
            Self::ENOTDIR => write!(f, "ENOTDIR"),
            Self::EISDIR => write!(f, "EISDIR"),
            Self::EINVAL => write!(f, "EINVAL"),
            Self::ENFILE => write!(f, "ENFILE"),
            Self::EMFILE => write!(f, "EMFILE"),
            Self::ENOTTY => write!(f, "ENOTTY"),
            Self::ETXTBSY => write!(f, "ETXTBSY"),
            Self::EFBIG => write!(f, "EFBIG"),
            Self::ENOSPC => write!(f, "ENOSPC"),
            Self::ESPIPE => write!(f, "ESPIPE"),
            Self::EROFS => write!(f, "EROFS"),
            Self::EMLINK => write!(f, "EMLINK"),
            Self::EPIPE => write!(f, "EPIPE"),
            Self::EDOM => write!(f, "EDOM"),
            Self::ERANGE => write!(f, "ERANGE"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

impl std::error::Error for Errno {}

impl From<i32> for Errno {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<Errno> for ErrnoKind {
    fn from(value: Errno) -> Self {
        Self::from(value.0)
    }
}

impl From<&Errno> for ErrnoKind {
    fn from(value: &Errno) -> Self {
        Self::from(value.0)
    }
}

impl From<i32> for ErrnoKind {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::EPERM,
            2 => Self::ENOENT,
            3 => Self::ESRCH,
            4 => Self::EINTER,
            5 => Self::EIO,
            6 => Self::ENXIO,
            7 => Self::E2BIG,
            8 => Self::ENOEXEC,
            9 => Self::EBADF,
            10 => Self::ECHILD,
            11 => Self::EAGAIN,
            12 => Self::ENOMEM,
            13 => Self::EACCES,
            14 => Self::EFAULT,
            15 => Self::ENOTBLK,
            16 => Self::EBUSY,
            17 => Self::EEXIST,
            18 => Self::EXDEV,
            19 => Self::ENODEV,
            20 => Self::ENOTDIR,
            21 => Self::EISDIR,
            22 => Self::EINVAL,
            23 => Self::ENFILE,
            24 => Self::EMFILE,
            25 => Self::ENOTTY,
            26 => Self::ETXTBSY,
            27 => Self::EFBIG,
            28 => Self::ENOSPC,
            29 => Self::ESPIPE,
            30 => Self::EROFS,
            31 => Self::EMLINK,
            32 => Self::EPIPE,
            33 => Self::EDOM,
            34 => Self::ERANGE,
            _ => Self::Unknown,
        }
    }
}
