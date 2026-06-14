// https://www.reddit.com/r/rust/comments/6hoayo/how_do_i_write_to_stdout_without_line_buffering/

#[cfg(unix)]
pub fn stdout() -> std::fs::File {
    use std::os::unix::io::FromRawFd;
    unsafe {
        std::fs::File::from_raw_fd(1)
    }
}

#[cfg(windows)]
pub fn stdout() -> std::fs::File {

    use std::os::windows::io::FromRawHandle;

    extern crate winapi;

    unsafe  {
        let h = winapi::um::processenv::GetStdHandle(winapi::um::winbase::STD_OUTPUT_HANDLE);
        std::fs::File::from_raw_handle(h as *mut std::ffi::c_void)
    }

}