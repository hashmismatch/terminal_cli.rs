#[cfg(any(target_os="none"))]
#[path = "no_std.rs"]
pub mod v1;

#[cfg(any(feature="debug_std", test, not(target_os="none")))]
#[path = "std.rs"]
pub mod v1;