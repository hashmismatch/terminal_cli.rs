use prelude::v1::*;
use utils::*;

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
	additional_part_range: Range<usize>,
	/// String that should be displayed on the autocomplete list
	display_range: Range<usize>
}

impl AutocompleteLine {
	pub fn get_display(&self) -> &str {
		&self.full_new_line.index(self.display_range.clone())
	}

	pub fn get_additional_part(&self) -> &str {
		&self.full_new_line.index(self.additional_part_range.clone())
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineBufferResult {	
	MoreInputRequired { prefix_matches: Vec<String> },
	NoMatchFound,
	Match { args: String },
	Autocomplete { result: AutocompleteResult }
}

/// Match commands against the given input line
pub struct CliLineMatcher<'a> {
	line: &'a str,
	line_trimmed: &'a str,
	line_prefix: Option<String>,
	mode: LineMatcherMode,	
	path_separator: Option<char>,
    state: LineBufferResult
}

/// State of the line matcher
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LineMatcherProgress {
	MatchFound,
	Processing,
	Skipped
}

/// Should we stop processing commands when we find a match
/// or should we just collect autocomplete suggestions?
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LineMatcherMode {
	Execute,
	AutocompleteOnly
}

impl<'a> CliLineMatcher<'a> {
	/// Create a new line matcher
	pub fn new(line: &'a str, mode: LineMatcherMode) -> CliLineMatcher<'a> {
		CliLineMatcher {
			line: line,
			line_trimmed: line.trim(),
			mode: mode,
			line_prefix: None,
			state: LineBufferResult::MoreInputRequired { prefix_matches: Vec::new() },
			path_separator: Some('/')
		}
	}


	pub fn set_line_prefix(&mut self, prefix: String) {
		self.line_prefix = Some(prefix);
	}

    pub fn get_state(&self) -> &LineBufferResult {
        &self.state
    }

	/// Match the command, mutates the internal state of the matching
	pub fn match_cmd<'b>(&mut self, cmd: &'b CliCommand<'b>) -> LineMatcherProgress {
		let prefixed = if let Some(ref prefix) = self.line_prefix {
			let cmd = format!("{}{}", prefix, &cmd.command);
			Some((cmd, prefix.to_string()))
		} else {
			None
		};

		if let Some((cmd, prefix)) = prefixed {
			let r = self.match_cmd_str(&cmd, Some(&prefix));

			if r != LineMatcherProgress::Processing {
				return r;
			}
		}

		self.match_cmd_str(&cmd.command, None)
	}

    /// Match the string, mutates the internal state
	pub fn match_cmd_str<'b>(&mut self, cmd: &'b str, prefix: Option<&'b str>) -> LineMatcherProgress {
		match self.state {
            LineBufferResult::MoreInputRequired { .. } => (),
            _ => { return LineMatcherProgress::Skipped; }
        };

		let c = cmd.len();
		if c == 0 { return LineMatcherProgress::Processing; }		

		let cmd_ends_with_sep = {
			let l = cmd.chars().last();
			l == Some(' ')
		};
		let at_sep = self.line.chars().skip(cmd.len()).next();

		if self.mode == LineMatcherMode::Execute &&
		   self.line.len() >= c &&
		   self.line.starts_with(&*cmd) &&
		   (cmd_ends_with_sep || at_sep == None || at_sep == Some(' '))
		{
			let args = self.line.chars().skip(cmd.len() + 1).collect();
			self.state = LineBufferResult::Match { args: args };
			return LineMatcherProgress::MatchFound;
		} else if self.mode == LineMatcherMode::AutocompleteOnly && cmd.starts_with(self.line) {

			if let Some(sep) = self.path_separator {
				// show and auto-complete only the first part of the separated command string				
				let common = [self.line, cmd];
				
				// skip the common part that's already in the buffer
				let (c, prefix) = {
					let p = longest_common_prefix(&common);

					match p {
						Some(p) => {
							(&cmd[p.len()..], p)
						},
						None => {
							(cmd, "")
						}
					}
				};

				match c.find(sep) {
					None => {
						self.push_prefix_match(cmd.to_string());
					},
					Some(l) => {
						self.push_prefix_match(format!("{}{}{}", prefix, &c[..l], sep));
					}
				}

			} else {
				self.push_prefix_match(cmd.to_string());
			}			
		}

		LineMatcherProgress::Processing
	}

	/// Maintain a list of unique prefixes!
	fn push_prefix_match(&mut self, s: String) {
		if let LineBufferResult::MoreInputRequired { ref mut prefix_matches } = self.state {
			// check the last entry first, usually they match
			if let Some(l) = prefix_matches.last() {
				if l == &s {
					return;
				}
			}

			if !prefix_matches.contains(&s) {
				prefix_matches.push(s);
			}
		}
	}


	/// Finish with the line matching, consume the matcher
	pub fn finish(self) -> LineBufferResult {
        match self.state {
			LineBufferResult::MoreInputRequired { .. } => (),
            LineBufferResult::NoMatchFound => { return LineBufferResult::NoMatchFound; },
            p @ LineBufferResult::Match { .. } => { return p; },
            p @ LineBufferResult::Autocomplete { .. } => { return p; }
        };

		match (self.mode, self.state) {
			(LineMatcherMode::AutocompleteOnly, LineBufferResult::MoreInputRequired { prefix_matches }) => {

				let autocomplete = match prefix_matches.len() {
					0 => AutocompleteResult::None,
					1 => {
						let full_new_line = prefix_matches[0].to_string();
						let full_new_line_length = full_new_line.len();

						let l = AutocompleteLine { 
							full_new_line: full_new_line,
							additional_part_range: self.line.len()..full_new_line_length,
							display_range: 0..full_new_line_length
						};
						AutocompleteResult::SingleMatch { line: l }
					}
					_ => {			
						let mut lines = Vec::new();
						for prefix_match in prefix_matches.into_iter() {
							
							let full_new_line = prefix_match;
							let full_new_line_length = full_new_line.len();

							let display_range = {
								if let Some(sep) = self.path_separator {
									let l = self.line.rfind(sep);
									let r = full_new_line.rfind(sep);
									match (l, r) {
										(Some(l), Some(r)) => {
											let p = min(l, r) + 1;
											if full_new_line.len() > p {
												p..full_new_line_length
											} else {
												0..full_new_line_length
											}
										},
										_ => {
											0..full_new_line_length
										}
									}
								} else {
									0..full_new_line_length
								}
							};

							let l = AutocompleteLine {
								full_new_line: full_new_line,
								additional_part_range: self.line.len()..full_new_line_length,
								display_range: display_range
							};
							lines.push(l);
						}

						// sort the lines
						lines.sort_by(|a, b| { a.full_new_line.cmp(&b.full_new_line) });

						// todo: separator and non-separator modes!
						// lcp is already properly computed!

						let lcp = {
							let line_strings: Vec<&str> = lines.iter().map(|x| x.full_new_line.as_str()).collect();

							let lcp = longest_common_prefix(&line_strings);
							if let Some(lcp) = lcp {							
								if lcp.len() == self.line.len() {
									None
								} else {
									Some(lcp.to_string())
								}
							} else {
								None
							}
						};

						if let Some(lcp) = lcp {
							AutocompleteResult::SingleMatch { 
								line: AutocompleteLine {
									additional_part_range: self.line.len()..lcp.len(),
									display_range: 0..lcp.len(),
									full_new_line: lcp								
								}
							}
						} else {
							AutocompleteResult::MultipleMatches { lines: lines }
						}
					}
				};

				LineBufferResult::Autocomplete { result: autocomplete }
			},
			(_, _) => LineBufferResult::NoMatchFound
		}
	}
}
