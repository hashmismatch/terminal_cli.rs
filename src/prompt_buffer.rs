use prelude::v1::*;
use cli::*;
use keys::*;
use terminal::*;
use utils::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PromptEvent {
	Break
}

enum AutocompleteRequest {	
	None,
	HaveMultipleOptions { lines: Vec<AutocompleteLine> }
}

/// Holds the current line buffer for a terminal and its possible autocomplete state.
pub struct PromptBuffer {
	line_buffer: Vec<u8>,
	echo: bool,
	prompt: String,
	newline: String,
	autocomplete: AutocompleteRequest,
	max_buffer_length: usize
}

impl PromptBuffer {
	pub fn new(echo: bool) -> PromptBuffer {
		PromptBuffer {
			line_buffer: Vec::new(),
			prompt: "# ".into(),
			echo: echo,
			newline: "\r\n".to_string(),
			autocomplete: AutocompleteRequest::None,
			max_buffer_length: 512
		}
	}

	pub fn print_prompt<T: CharacterTerminalWriter>(&self, output: &mut T) {
		if !self.prompt.len() == 0 { return; }

		output.print_str(&self.prompt);
	}

	pub fn handle_key<T, F: FnOnce(CliLineMatcher, &mut T) -> LineBufferResult>(&mut self, key: Key, terminal: &mut T, call_commands: F) -> Option<PromptEvent>
		where T: CharacterTerminalWriter + LineTerminalWriter + Write
	{
		let mut handled_autocomplete = false;

		match key {
			Key::Tab => {

				match self.autocomplete {					
					AutocompleteRequest::None => {
						if let Some(line) = self.buffer_as_str() {
							let mut matcher = CliLineMatcher::new(&line, LineMatcherMode::AutocompleteOnly);
							let result = call_commands(matcher, terminal);

							match result {
								LineBufferResult::Autocomplete { ref result } => {
									match *result {
										AutocompleteResult::None => (),
										AutocompleteResult::SingleMatch { ref line } => {
											// immediately send the new stuff
											let ref additional_part = line.additional_part;
											terminal.print_str(additional_part);

											// replace our line buffer with the stuff from autocomplete, to be consistent with future invokations
											self.line_buffer.clear();
											for c in line.full_new_line.bytes() {
												self.line_buffer.push(c);
											}
										},
										AutocompleteResult::MultipleMatches { ref lines } => {
											// this was the first time tab was pressed, and there are multiple options. store them,
											// when the user presses tab again, print them out
											// we could also bleep at this point...

											self.autocomplete = AutocompleteRequest::HaveMultipleOptions {
												lines: lines.clone()
											};
										}
									}
								},
								_ => ()
							}
						}

						handled_autocomplete = true;
					},					
					AutocompleteRequest::HaveMultipleOptions { ref lines } => {
						// print the available autocomplete options
						
						let suggestions = lines.iter().map(|l| { l.full_new_line.as_str() }).collect::<Vec<&str>>();
						format_in_columns(suggestions.as_slice(), 80, 4, &self.newline, terminal);

						self.print_prompt(terminal);

						// restore the current buffer
						terminal.print(&self.line_buffer);

						handled_autocomplete = false;
					}
				}

				
			},
			Key::Newline => {
				
				terminal.print_str_line("");

				if let Some(line) = self.buffer_as_str() {
					let mut matcher = CliLineMatcher::new(&line, LineMatcherMode::Execute);
					let result = call_commands(matcher, terminal);
				}

				self.line_buffer.clear();
				self.print_prompt(terminal);
			},
			Key::Backspace => {
				if let Some(..) = self.line_buffer.pop() {
					if self.echo {
						terminal.print(&[0x08, 0x20, 0x08]);
					}
				}
			},
			Key::Break => {
				if self.line_buffer.len() == 0 {
					return Some(PromptEvent::Break);
				}

				// clear the line
				self.line_buffer.clear();
				terminal.print_str_line("");
				self.print_prompt(terminal);
			},
			Key::Eot => {
				return Some(PromptEvent::Break);
			},
			Key::Arrow(_) => {
				// todo: line history?
			},
			Key::Character(c) => {
				if c != '\r' as u8 {
					self.line_buffer.push(c);

					if self.echo {
						terminal.print(&[c]);
					}
				}
			}
		}

		if handled_autocomplete == false {
			// reset autocomplete state
			self.autocomplete = AutocompleteRequest::None;
		}		

		None
	}

	fn buffer_as_str(&self) -> Option<String> {
		if let Ok(s) = String::from_utf8(self.line_buffer.clone()) {
			Some(s)
		} else {
			None
		}
	}
}