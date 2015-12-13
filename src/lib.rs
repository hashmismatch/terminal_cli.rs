//! A helper library for implementing low-level terminal command line interfaces,
//! like those on embedded bare-bones environments with UART ports.
//!
//! The library doesn't use Rust's ```std``` library, but requires the ```alloc```
//! and ```collections``` libraries for heap memory allocation and vector manipulation.
//!
//! Custom commands can be easily added by implementing the ```CliCommand``` trait.
//!
//! # Example
//!
//! ```
//! #![feature(convert)]
//! # use terminal_cli::*;
//! # use std::io;
//! # use std::io::Write;
//! let help = CliCommandKeyword {
//! 	keyword: "help".to_string(),
//! 	action: |line, cli| {
//! 		cli.output_line("Help here!");
//! 	}
//! };
//!
//! // Adds "set time <HH:mm>" and "get time" commands. No parsing here.
//! let time = CliPropertyVar {
//! 	var_name: "time".to_string(),
//! 	var_value: "11:15".to_string(),
//! 	val_hint: "HH:mm".to_string(),
//! 
//! 	var_output: |v| { v.to_string() },
//! 	var_input: |v| {
//! 		if v.len() > 0 { Some(v.to_string()) }
//! 		else { None }
//! 	}
//! };
//! 
//! // For this test, a ```stdout``` terminal. Isn't included in the 
//! // library as it's meant for bare-bones usage - no std library.
//! struct StdoutTerminal;
//! impl CliTerminal for StdoutTerminal {
//! 	fn output_line(&mut self, line: &str) {
//! 		println!("{}", line);
//! 	}
//! }
//!
//! let mut commands = vec![
//! 	Box::new(help) as Box<CliCommand + Send>,
//! 	Box::new(time) as Box<CliCommand + Send>
//! ];
//! 
//! let mut term = StdoutTerminal;
//! 
//! // Execute a line buffer
//! cli_execute("help", commands.as_mut_slice(), &mut term);
//! 
//! // Try to autocomplete the active buffer. Will return a summary of all commands.
//! let autocomplete = cli_try_autocomplete("", commands.as_mut_slice());
//!
//!
//! // Serial terminal input handling
//! struct StdoutPromptOutput;
//! impl CliPromptTerminal for StdoutPromptOutput {
//!		fn print_bytes(&self, bytes: &[u8]) {
//!			io::stdout().write(bytes);
//!		}
//! }
//!
//! let mut prompt = CliPromptAutocompleteBuffer::new("#".to_string());
//! // help + newline
//! let keyboard_input = vec!['h' as u8, 'e' as u8, 'l' as u8, 
//! 	0x7f /* backspace */, 'l' as u8, 'p' as u8, 0x0d /* \r */ ];
//! 
//! for byte in keyboard_input {
//!		prompt.handle_received_byte(byte, &StdoutPromptOutput,
//!									commands.as_mut_slice(), &mut term);
//! }
//! ```

#![no_std]
#![feature(core)]
#![feature(alloc)]
#![feature(no_std)]
#![feature(macro_reexport)]
#![feature(unboxed_closures)]
#![feature(collections)]
#![feature(convert)]
#![feature(slice_concat_ext)]
#![feature(core_float)]
#![feature(fixed_size_array)]
#![feature(iter_cmp)]
#![feature(core_intrinsics)]
#![feature(fn_traits)]

extern crate alloc;

#[macro_use(vec, format)]
extern crate collections;

// for tests
#[cfg(test)]
#[macro_use(println, assert_eq, print, panic)]
extern crate std;


mod cli;
mod commands;
mod utils;
mod prompt_buffer;


pub use cli::*;
pub use utils::*;
pub use commands::*;
pub use prompt_buffer::*;

#[cfg(test)]
mod tests;
