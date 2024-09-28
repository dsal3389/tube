use std::os::raw::{c_char, c_int};

extern "C" {
    pub(crate) fn inotify_init() -> c_int;
    pub(crate) fn inotify_init1(flags: c_int) -> c_int;
    pub(crate) fn inotify_add_watch(fd: c_int, pathname: *const c_char, mask: u32) -> c_int;
    pub(crate) fn inotify_rm_watch(fd: c_int, wd: c_int) -> c_int;
    pub(crate) fn close(fd: c_int) -> c_int;
}
