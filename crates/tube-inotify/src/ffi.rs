use std::os::raw::{c_char, c_int, c_short, c_ulong};

pub const POLLIN: c_short = 0x001;

pub const IN_NONBLOCK: c_int = 2048;
pub const IN_CLOSE_WRITE: u32 = 0x00000008;
pub const IN_CLOSE_NOWRITE: u32 = 0x00000010;
pub const IN_OPEN: u32 = 0x00000020;
pub const IN_CLOSE: u32 = IN_CLOSE_WRITE | IN_CLOSE_NOWRITE;

pub type nfds_t = c_ulong;

#[repr(C)]
pub struct inotify_event {
    pub wd: c_int,
    pub mask: u32,
    pub cookie: u32,
    pub len: u32,
}

#[repr(C)]
pub struct pollfd {
    pub fd: c_int,
    pub events: c_short,
    pub revents: c_short,
}

extern "C" {
    pub(crate) fn inotify_init() -> c_int;
    pub(crate) fn inotify_init1(flags: c_int) -> c_int;
    pub(crate) fn inotify_add_watch(fd: c_int, pathname: *const c_char, mask: u32) -> c_int;
    pub(crate) fn inotify_rm_watch(fd: c_int, wd: c_int) -> c_int;
    pub(crate) fn read(fd: c_int, buf: *mut u8, count: usize) -> isize;
    pub(crate) fn close(fd: c_int) -> c_int;
    pub(crate) fn poll(fds: *mut pollfd, nfds: nfds_t, timeout: c_int) -> c_int;
    pub(crate) fn __errno_location() -> *mut c_int;
}
