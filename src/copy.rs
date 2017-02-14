extern crate libc;

use std;
use std::fs::File;
use std::os::unix::io::AsRawFd;

use errno;

enum CopyFailure {
    Unsupported,
    Errno(i32),
}

fn try_sendfile(src: &File, dest: &File, len: u64) -> Result<(), CopyFailure> {
    let mut remaining = len;
    while remaining > 0 {
        unsafe {
            let offset: *mut i64 = std::ptr::null_mut();
            let to_send: usize = std::cmp::min(std::u32::MAX as u64, remaining) as usize;
            let sent = libc::sendfile(dest.as_raw_fd(), src.as_raw_fd(), offset, remaining as usize);
            let error = errno();
            if -1 == sent {
                if libc::EAGAIN == error {
                    continue;
                }

                if len == remaining &&
                    (libc::EINVAL == error || libc::ENOSYS == error) {
                        return Err(CopyFailure::Unsupported)
                }

                return Err(CopyFailure::Errno(error));
            }
            remaining -= sent as u64;
        }
    }
    return Ok(());
}

fn try_streams(src: &mut File, dest: &mut File, len: u64) -> Result<(), ()> {
    std::io::copy(src, dest).unwrap();
    return Ok(());
}

pub fn copy_file(src: &mut File, dest: &mut File, len: u64) -> Result<(), String> {
    // TODO: copy_file_range

    match try_sendfile(src, dest, len) {
        Ok(_) => return Ok(()),
        Err(fail) => match fail {
            CopyFailure::Errno(x) => return Err(format!("sendfile failed: errno({})", x)),
            CopyFailure::Unsupported => ()
        }
    };

    try_streams(src, dest, len).unwrap();

    return Err("unsupported".to_string());
}
