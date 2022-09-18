//! A simple cross-platform solution for deciding if a given user is privileged.
//! Works on Unix (Linux and MacOS) and Windows, please make an issue if it doesn't work for you!
//!
//! Usage:
//! ```
//! use is_superuser::is_superuser;
//! fn main() {
//!     if is_superuser() {
//!         println!("I am running with sudo/admin privileges!");
//!     } else {
//!         println!("I am a normal user!");
//!     }
//! }
//! ```
//!
#[cfg(test)]
mod tests {
    use crate::checker;
    #[test]
    fn no_superuser() {
        assert_eq!(checker::is_superuser(),false);
    }
}
#[cfg_attr(windows, path = "checker/windows.rs")]
#[cfg_attr(unix, path = "checker/unix.rs")]
mod checker;

pub use crate::checker::*;
