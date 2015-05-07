use core::prelude::*;
use alloc::boxed::Box;

use collections::Vec;
use collections::String;
use collections::string::ToString;
use collections::string::FromUtf8Error;
use core::mem;
use core::array::FixedSizeArray;

use cli::*;
use utils::*;

enum AutocompleteRequest {	
	None,
	HaveMultipleOptions { matches: AutocompleteResult }
}

/// Holds the current line buffer for a terminal and its possible autocomplete state.
pub struct CliPromptAutocompleteBuffer {
	line_buffer: Vec<u8>,
	prompt: String,
	newline: String,
	autocomplete: AutocompleteRequest,
	max_buffer_length: usize
}

pub trait CliPromptTerminal {
	fn print_bytes(&self, bytes: &[u8]);
	fn print(&self, str: &str) {
		self.print_bytes(str.bytes().collect::<Vec<u8>>().as_slice());
	}
}

impl CliPromptAutocompleteBuffer {
	pub fn new(prompt: String) -> CliPromptAutocompleteBuffer {
		CliPromptAutocompleteBuffer {
			line_buffer: Vec::new(),
			prompt: prompt, 
			newline: "\r\n".to_string(),
			autocomplete: AutocompleteRequest::None,
			max_buffer_length: 512
		}
	}

	pub fn print_prompt<T>(&self, output: &T) where T: CliPromptTerminal {
		if !self.prompt.len() == 0 { return; }

		output.print(self.prompt.as_str());
	}

	pub fn handle_received_byte<T>(&mut self, byte: u8, output: &T, cmds: &mut [Box<CliCommand + 'static>], cli_terminal: &mut CliTerminal)
		where T: CliPromptTerminal
	{
		let mut handled_autocomplete = false;

		match byte {
			// tab \t
			0x09 => {
				match self.autocomplete {					
					AutocompleteRequest::None => {
						// try to resolve the options
						let str = String::from_utf8(self.line_buffer.clone());
						if !str.is_err() {
							let autocomplete = cli_try_autocomplete(str.unwrap().as_str(), cmds);

							match autocomplete {
								AutocompleteResult::None => {},
								AutocompleteResult::SingleMatch { line: ref line } => {
									// immediately send the new stuff
									let ref additional_part = line.additional_part;
									output.print_bytes(additional_part.bytes().collect::<Vec<u8>>().as_slice());

									// replace our line buffer with the stuff from autocomplete, to be consistent with future invokations
									self.line_buffer.clear();
									for c in line.full_new_line.bytes() {
										self.line_buffer.push(c);
									}

									// we're done with this one
									self.autocomplete = AutocompleteRequest::None;
								},
								AutocompleteResult::MultipleMatches { lines: ref lines } => {
									// this was the first time tab was pressed, and there are multiple options. store them,
									// when the user presses tab again, print them out
									// we could also bleep at this point...

									self.autocomplete = AutocompleteRequest::HaveMultipleOptions {
										matches: autocomplete.clone()
									};
								}
							}
						}

						handled_autocomplete = true;
					},
					AutocompleteRequest::HaveMultipleOptions { matches: ref matches } => {
						if let &AutocompleteResult::MultipleMatches { lines: ref lines } = matches {
							// print the available autocomplete options
							let suggestions = lines.iter().map(|l| { l.full_new_line.as_str() }).collect::<Vec<&str>>();							
							output.print(format_in_columns(suggestions.as_slice(), 80, 4, "\r\n").as_str());

							self.print_prompt(output);

							// restore the current buffer
							output.print_bytes(self.line_buffer.as_slice());

							handled_autocomplete = false;
						};
					}
				}
			}
			
			// carriage return, \r
			0x0d  => {
				output.print(self.newline.as_str());

				// new line
				let str = String::from_utf8(self.line_buffer.clone());
				if str.is_err() {
					output.print("String parse error.");
				} else {
					cli_execute(str.unwrap().as_str(), cmds, cli_terminal);
				}

				self.line_buffer.clear();
				self.print_prompt(output);
			}

			// backspace
			0x7f => {
				// reply to it only if our line buffer contains something
				// otherwise it would overwrite the prompt

				if let Some(..) = self.line_buffer.pop() {					
					output.print_bytes(&[0x7f].as_slice());
				}
			}

			// anything else
			_ => {
				self.line_buffer.push(byte);
				if self.line_buffer.len() >= self.max_buffer_length {
					// system message?
					output.print(self.newline.as_str());
					output.print("Line buffer overflow.");
					output.print(self.newline.as_str());

					self.line_buffer.clear();
					self.print_prompt(output);
				}
				output.print_bytes(&[byte].as_slice());
			}
		}

		if (handled_autocomplete == false) {
			// reset autocomplete state
			self.autocomplete = AutocompleteRequest::None;
		}
	}
}
