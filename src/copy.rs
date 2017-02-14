extern crate libc;

use std;
use std::fs::File;
use std::os::unix::io::AsRawFd;

use errno;

fn try_sendfile(src: &File, dest: &File, len: u64) -> Result<(), String> {
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
                        // TODO: return TRY_ANOTHER
                }

                return Err(format!("sendfile failed: errno({})", error));
            }
            remaining -= sent as u64;
        }
    }
    return Ok(());
}

pub fn copy_file(src: &File, dest: &File, len: u64) -> Result<(), String> {
    // TODO: copy_file_range

    try_sendfile(src, dest, len).unwrap();
    return Ok(());
}
