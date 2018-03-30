//! A helper library for implementing a low-level terminal command line interface,
//! like those on embedded bare-bones environments with a UART port.
//!
//! The library doesn't use Rust's ```std``` library, but requires the ```alloc```
//! and ```collections``` libraries for heap memory allocation and vector manipulation.
//!
//! # Example
//!
//! ```
//! # use terminal_cli::*;
//! # use std::io;
//! # use std::io::Write;
//!
//! // Simple ranged integer property
//!	//let mut num1 = new_property_min_max("num1".into(), 1 as u8, 1, 100);
//! let mut num1 = 1;
//! 
//! // Rust stdout terminal
//!	let mut terminal = StdoutTerminal;
//!	
//! let options = PromptBufferOptions { echo: true, ..Default::default() };
//! let mut prompt = PromptBuffer::new(options);
//!
//!	let input_keys = [Key::Character('h' as u8), Key::Character('e' as u8), Key::Character('l' as u8), Key::Character('p' as u8),
//! 				  Key::Newline];
//! 
//! for key in &input_keys {
//!		let p = prompt.handle_key(*key, &mut terminal, |mut m| {
//!         if let Some(mut ctx) = m.command("help") {
//!             ctx.get_terminal().print_line("Help!");
//!         }
//!
//!         if let Some(mut ctx) = m.property("num1", validate_property_min_max(1, 100)) {
//!             ctx.apply(&mut num1);
//!         }
//!		});
//!		if let Some(PromptEvent::Break) = p {
//!			break;
//! 	}
//! }
//!	
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

#![cfg_attr(not(feature="std"), feature(alloc))]
#![cfg_attr(not(feature="std"), feature(core_intrinsics))]
#![cfg_attr(not(feature="std"), feature(slice_concat_ext))] 

#[cfg(not(feature="std"))]
#[macro_use]
extern crate alloc;

mod autocomplete;
mod property;
mod utils;
mod prelude;
mod cli;
mod cli_command;
mod cli_property;
mod keys;
mod keys_terminal;
mod terminal;
mod prompt_buffer;


pub use autocomplete::*;
pub use utils::*;
pub use cli::*;
pub use cli_command::*;
pub use cli_property::*;
pub use keys::*;
pub use keys_terminal::*;
pub use property::*;
pub use terminal::*;
pub use prompt_buffer::*;

#[cfg(test)]
mod tests;
