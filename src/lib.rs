#![no_std]


#![cfg_attr(target_os="none", feature(alloc))]
#![cfg_attr(target_os="none", feature(collections))]
#![cfg_attr(target_os="none", feature(core_intrinsics))]
#![cfg_attr(target_os="none", feature(slice_concat_ext))]

#[cfg(any(target_os="none"))]
#[macro_use]
extern crate alloc;

#[cfg(any(target_os="none"))]
#[macro_use]
extern crate collections;

#[cfg(any(feature="debug_std", test, not(target_os="none")))]
#[macro_use]
extern crate std;


mod property;
mod utils;
mod prelude;
mod cli;
mod keys;
mod keys_terminal;
mod terminal;
mod terminal_telnet;
mod prompt_buffer;


pub use utils::*;
pub use cli::*;
pub use keys::*;
pub use keys_terminal::*;
pub use property::*;
pub use terminal::*;
pub use terminal_telnet::*;
pub use prompt_buffer::*;

#[cfg(feature = "termios_support")]
pub mod terminal_termios;

#[cfg(test)]
mod tests;