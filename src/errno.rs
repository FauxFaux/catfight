extern crate libc;

// TODO: replace with a module, or ..?
pub fn errno() -> i32 {
    unsafe {
        *libc::__errno_location() as i32
    }
}

