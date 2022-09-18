use libc;
/// Returns true if the current user has root privileges, or false otherwise.
pub fn is_superuser() -> bool {
    unsafe {
        if libc::geteuid() != 0 {
            return false
        } else {
            return true
        }
    }
}
