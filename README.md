# is_superuser

A simple cross-platform solution for deciding if a given user is privileged.
Works on Unix (Linux and MacOS) and Windows, please make an issue if it doesn't work for you!

Usage:
```rust
use is_superuser::is_superuser;
fn main() {
    if is_superuser() {
        println!("I am running with sudo/admin privileges!");
    } else {
        println!("I am a normal user!");
    }
}
```
