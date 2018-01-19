//! # Terminal CLI
//! 
//! Need to build an interactive command prompt, with commands, properties and with full autocomplete? This is for you.
//!
//! [![Build Status](https://travis-ci.org/hashmismatch/terminal_cli.rs.svg?branch=master)](https://travis-ci.org/hashmismatch/terminal_cli.rs)
//! 
//! [![Documentation](https://docs.rs/terminal_cli/badge.svg)](https://docs.rs/terminal_cli)
//! 
//!
//! # Example, output only (Rust's ```stdout```)
//!
//! ```
//! # use terminal_cli::*;
//! # use std::io;
//! # use std::io::Write;
//!
//! // Simple ranged integer property
//! let mut num1 = 1;
//! 
//! // Rust stdout terminal
//! let mut terminal = StdoutTerminal;
//!	
//! let options = PromptBufferOptions { echo: true, ..Default::default() };
//! let mut prompt = PromptBuffer::new(options);
//!
//! let input_keys = [Key::Character('h' as u8), Key::Character('e' as u8), Key::Character('l' as u8),
//!                   Key::Character('p' as u8), Key::Newline];
//! 
//! for key in &input_keys {
//!     let p = prompt.handle_key(*key, &mut terminal, |mut m| {
//!         if let Some(mut ctx) = m.command("help") {
//!             ctx.get_terminal().print_line("Help!");
//!         }
//!
//!         // Provides "num1/get" and "num1/set", with input validation
//!         if let Some(mut ctx) = m.property("num1", validate_property_min_max(1, 100)) {
//!             ctx.apply(&mut num1);
//!         }
//!     });
//! 
//!     if let PromptEvent::Break = p {
//!         break;
//!     }
//! }
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

#![cfg_attr(not(feature="std"), feature(alloc))]

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


pub mod i18n;

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
