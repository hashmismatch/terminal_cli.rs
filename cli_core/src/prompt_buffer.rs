use prelude::v1::*;
use autocomplete::*;
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
	change_path_enabled: bool,
	current_path: Vec<String>,
	path_separator: char,
	autocomplete: AutocompleteRequest,
	options: PromptBufferOptions
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NewlineSequence {
	Newline,
	CarriageReturn,
	NewlineOrCarriageReturn
}

/// Options for the prompt buffer
pub struct PromptBufferOptions {
	/// Prompt sequence to be printed after every newline
	pub prompt: Cow<'static, str>,
	/// Newline sequence to be used while writing
	pub newline: Cow<'static, str>,
	/// Maximum size of the line buffer
	pub max_line_length: usize,
	/// Echo the typed characters?
	pub echo: bool,
	/// Input newline key sequence
	pub newline_key_sequence: NewlineSequence

}

impl Default for PromptBufferOptions {
	fn default() -> PromptBufferOptions {
		PromptBufferOptions {
			prompt: "# ".into(),
			echo: true,
			newline: "\r\n".into(),
			max_line_length: 512,
			newline_key_sequence: NewlineSequence::NewlineOrCarriageReturn
		}
	}
}

impl PromptBuffer {
	/// Create a new prompt buffer
	pub fn new(options: PromptBufferOptions) -> PromptBuffer {
		PromptBuffer {
			line_buffer: Vec::new(),
			change_path_enabled: false,
			current_path: vec![],
			path_separator: '/',
			autocomplete: AutocompleteRequest::None,
			options: options
		}
	}
	
	/// Print the prompt
	pub fn print_prompt<T: CharacterTerminalWriter>(&self, output: &mut T) {
		if !self.options.prompt.len() == 0 { return; }

		if self.change_path_enabled {
			let sep = self.path_separator.to_string();

			let path: String = {
				if self.current_path.len() == 0 {
					sep
				} else {
					format!("{}{}{}", &sep, self.current_path.join(&sep), &sep)
				}
			};
			
			let prompt = self.options.prompt.replace("\\W", &path);

			output.print_str(&prompt);
		} else {
			output.print_str(&self.options.prompt);
		}
	}

	/// Handle the incoming key press. Pass the lambda that will match the commands for
	/// autocomplete or execution.
	pub fn handle_key<T, F: FnOnce(&mut CliExecutor) -> ()>(&mut self, key: Key, terminal: &mut T, call_commands: F) -> Option<PromptEvent>
		where T: CharacterTerminalWriter + FmtWrite
	{
		let mut handled_autocomplete = false;

		let is_line_finished = {
			match self.options.newline_key_sequence {
				NewlineSequence::Newline => key == Key::Newline,
				NewlineSequence::CarriageReturn => key == Key::CarriageReturn,
				NewlineSequence::NewlineOrCarriageReturn => {
					key == Key::Newline || key == Key::CarriageReturn
				}
			}
		};

		if is_line_finished {
			
			terminal.print_line("");

			if let Ok(line) = str::from_utf8(self.line_buffer.as_slice()) {
				
				let result = {
					let matcher = CliLineMatcher::new(&line, LineMatcherMode::Execute);
					let mut executor = CliExecutor::new(matcher, terminal);
					call_commands(&mut executor);
					executor.close().finish()
				};

				match result {
					LineBufferResult::NoMatchFound => {
						if line.trim().len() > 0 {
							// command not recognized
							terminal.print_line("Command not recognized.");
						}
					},
					_ => ()
				}

			}

			self.line_buffer.clear();
			self.print_prompt(terminal);

		} else {
			match key {
				Key::Tab => {

					match self.autocomplete {
						AutocompleteRequest::None => {
							
							let mut single_match_additional_chars = None;

							if let Ok(line) = str::from_utf8(self.line_buffer.as_slice()) {

								let result = {
									let matcher = CliLineMatcher::new(&line, LineMatcherMode::AutocompleteOnly);
									let mut executor = CliExecutor::new(matcher, terminal);
									call_commands(&mut executor);
									executor.close().finish()
								};

								match result {
									LineBufferResult::Autocomplete { result } => {
										match result {
											AutocompleteResult::None => (),
											AutocompleteResult::SingleMatch { line } => {
												// immediately send the new stuff
												terminal.print_str(line.get_additional_part());

												// clear the line outside the borrowed content
												single_match_additional_chars = Some(line.full_new_line);
												
											},
											AutocompleteResult::MultipleMatches { lines } => {
												// this was the first time tab was pressed, and there are multiple options. store them,
												// when the user presses tab again, print them out
												// we could also bleep at this point...

												self.autocomplete = AutocompleteRequest::HaveMultipleOptions {
													lines
												};
											}
										}
									},
									_ => ()
								}
							}

							if let Some(single_match_additional_chars) = single_match_additional_chars.take() {
								// replace our line buffer with the stuff from autocomplete, to be consistent with future invokations
								self.line_buffer.clear();
								for c in single_match_additional_chars.bytes() {
									self.line_buffer.push(c);
								}
							}

							handled_autocomplete = true;
						},					
						AutocompleteRequest::HaveMultipleOptions { ref lines } => {
							// print the available autocomplete options

							terminal.print_line("");
							
							let suggestions = lines.iter().map(|l| { l.get_display() }).collect::<Vec<&str>>();
							match format_in_columns(suggestions.as_slice(), 80, 4, &self.options.newline, terminal) {
								Err(_) => terminal.print_line("Error formating the columns."),
								_ => ()
							}

							self.print_prompt(terminal);

							// restore the current buffer
							terminal.print(&self.line_buffer);

							handled_autocomplete = false;
						}
					}

					
				},
				Key::Newline | Key::CarriageReturn => {
					// newline keys				
				},
				Key::Backspace => {
					if let Some(..) = self.line_buffer.pop() {
						if self.options.echo {
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
					terminal.print_line("");
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

						if self.options.echo {
							terminal.print(&[c]);
						}
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

	/*
	fn buffer_as_str(&self) -> Option<&str> {
		str::from_utf8(self.line_buffer.as_slice()).ok()
	}

	fn buffer_as_string(&self) -> Option<String> {
		String::from_utf8(self.line_buffer.clone()).ok()
	}
	*/
}