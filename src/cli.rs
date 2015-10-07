use prelude::v1::*;
use utils::*;
use property::*;
use terminal::*;

/// A command that can be matched by the command line prompt
#[derive(Debug, Clone, PartialEq)]
pub struct CliCommand<'a> {
	/// Prefix with which the line should start.
	pub command: Cow<'a, str>,
	
	/// Help for this command. Will be shown in case user requests it with 'help [command]'
	/// or this command is the only one left when autocompleting	
	pub help: Option<Cow<'a, str>>
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CliError {
	InvalidInput
}

/// Result of the autocomplete request on a given set of commands
#[derive(Debug, Clone, PartialEq)]
pub enum AutocompleteResult {
	/// No suggestions available
	None,
	/// A single match has been found, the line buffer can be immediately expanded with the new command
	SingleMatch { line: AutocompleteLine },	
	/// Multiple matches, usually they can be presented to the end user in a column format.
	MultipleMatches { lines: Vec<AutocompleteLine> }
}

/// One autocomplete suggestion
#[derive(Debug, Clone, PartialEq)]
pub struct AutocompleteLine {
	/// The entire new suggested line buffer
	pub full_new_line: String,
	/// The additional suggested part of the buffer, can be sent to the terminal device
	pub additional_part: String
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBufferResult {
	MoreInputRequired,
	Match { args: String },
	Autocomplete { result: AutocompleteResult }
}

pub struct CliLineMatcher<'a> {
	prefix_matches: Vec<String>,
	line: &'a str,
	line_trimmed: &'a str,
	mode: LineMatcherMode,
	result: Option<LineBufferResult>
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LineMatcherProgress {
	MatchFound,
	Processing,
	Skipped
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LineMatcherMode {
	Execute,
	AutocompleteOnly
}

impl<'a> CliLineMatcher<'a> {
	pub fn new(line: &'a str, mode: LineMatcherMode) -> CliLineMatcher<'a> {
		CliLineMatcher {
			prefix_matches: Vec::new(),
			line: line,
			line_trimmed: line.trim(),
			mode: mode,
			result: None
		}
	}

	pub fn process<'b>(&mut self, cmd: &'b CliCommand<'b>) -> LineMatcherProgress {
		let c = cmd.command.len();
		if c == 0 { return LineMatcherProgress::Processing; }

		if self.result.is_some() { return LineMatcherProgress::Skipped; }

		if self.mode == LineMatcherMode::Execute && self.line.len() >= c && self.line.starts_with(&*cmd.command) {
			let args = self.line.chars().skip(cmd.command.len()).collect();
			self.result = Some(LineBufferResult::Match { args: args });
			return LineMatcherProgress::MatchFound;
		} else if cmd.command.starts_with(self.line) {
			self.prefix_matches.push(cmd.command.to_string());
		}

		LineMatcherProgress::Processing
	}

	pub fn process_property<'b, P: CliProperty<'b>, T: LineTerminalWriter>(&mut self, property: &mut P, terminal: &mut T) -> LineMatcherProgress {

		if self.process(property.command_get()) == LineMatcherProgress::MatchFound {
			if let Some(LineBufferResult::Match { ref args, .. }) = self.result {
				property.get(terminal);

				return LineMatcherProgress::MatchFound;
			}
		}

		if self.process(property.command_set()) == LineMatcherProgress::MatchFound {
			if let Some(LineBufferResult::Match { ref args, .. }) = self.result {
				property.set(args, terminal);

				return LineMatcherProgress::MatchFound;
			}
		}

		LineMatcherProgress::Processing
	}





	pub fn finish(mut self) -> LineBufferResult {
		if let Some(r) = self.result {
			return r;
		}

		if self.mode == LineMatcherMode::AutocompleteOnly {
			let autocomplete = match self.prefix_matches.len() {
				0 => AutocompleteResult::None,
				1 => {
					let ref cmd = self.prefix_matches[0];
					let ref m = cmd;
					let c = m.chars().skip(self.line.len()).collect();
					let l = AutocompleteLine { full_new_line: m.to_string(), additional_part: c };
					AutocompleteResult::SingleMatch { line: l }
				}
				_ => {			
					let mut lines = Vec::new();
					for cmd in &self.prefix_matches {
						let ref m = cmd;
						let c = m.chars().skip(self.line.len()).collect();
						let l = AutocompleteLine { full_new_line: m.to_string(), additional_part: c };
						lines.push(l);
					}

					// sort the lines
					lines.sort_by(|a, b| { a.full_new_line.cmp(&b.full_new_line) });

					let lcp = {
						let line_strings: Vec<&str> = lines.iter().map(|x| x.full_new_line.as_str()).collect();

						let lcp = longest_common_prefix(&line_strings);
						if let Some(lcp) = lcp {
							if lcp.len() == self.line.len() {
								None
							} else {
								Some(lcp)
							}
						} else {
							None
						}
					};

					if let Some(lcp) = lcp {
						AutocompleteResult::SingleMatch { 
							line: AutocompleteLine {
								full_new_line: lcp.clone(),
								additional_part: lcp.chars().skip(self.line.len()).collect()
							}
						}
					} else {
						AutocompleteResult::MultipleMatches { lines: lines }
					}
				}
			};

			return LineBufferResult::Autocomplete { result: autocomplete };
		}

		LineBufferResult::MoreInputRequired
	}
}
