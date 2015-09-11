use alloc::boxed::Box;

use collections::string::*;
use collections::Vec;
use collections::slice::SliceConcatExt;

use utils::*;


/// A command's hints to the autocompletition
#[derive(Clone)]
pub enum AutocompleteOption {
	/// Hint for the missing argument for the end user
	Hint { hint: String },
	/// The full line buffer of the suggested command
	FullCommand { line: String}
}

/// Terminal trait.
pub trait CliTerminal {
	/// Output a string with the newline characters at the end. The implementation
	/// adds the newline control characters.
	fn output_line(&mut self, line: &str);
}

/// A command that can be executed by the execution function.
pub trait CliCommand {
	/// Execute the command with the given line buffer
	fn execute(&mut self, cli: &mut CliTerminal, line: &str);
	/// Check if the line buffer is valid for this command
	fn is_match(&self, line: &str) -> bool;
	/// Give auto-complete hints
	fn autocomplete(&self, line_start: &str) -> Option<Vec<AutocompleteOption>>;

	fn get_property(&self) -> Option<&CliStringProperty>;
	fn get_property_mut(&mut self) -> Option<&mut CliStringProperty>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum CliStringPropertyError {
	InvalidValue
}

/// Command that alters the state of a simple string property
pub trait CliStringProperty {
	fn get_id(&self) -> &str;
	fn get_val(&self) -> String;
	fn set_val(&mut self, new_val: &str) -> Result<(), CliStringPropertyError>;
}

/// Execute the given line buffer with the set of commands.
pub fn cli_execute(line: &str, cmds: &mut [Box<CliCommand + Send + 'static>], cli: &mut CliTerminal) {
	let mut line_start = line.trim();
	if line_start.len() == 0 { return; }

	for ref mut cmd in cmds.iter_mut() {
		if !cmd.is_match(line_start) {
			continue;
		}

		cmd.execute(cli, line_start);
		return;
	}

	if line_start.ends_with("?") {
		line_start = line_start.trim_right_matches("?").trim();
	} else {
		cli.output_line("Unrecognized command.");
	}	

	let fl = collect_options(line_start, cmds);

	if fl.len() > 0 {
		let mut hints = fl.iter().filter_map(|c| {
			match c {
				&AutocompleteOption::Hint {hint: ref hint} => { Some(hint.clone()) }
				_ => { None }
			}
		}).collect::<Vec<String>>();

		if hints.len() > 0 {
			// sort the lines
			hints.sort_by(|a, b| { a.cmp(&b) });

			cli.output_line(format!("Related commands: {}", hints.connect(", ")).as_str());
		}
	}
}

/// Result of the autocomplete request on a given set of commands
#[derive(Debug, Clone)]
pub enum AutocompleteResult {
	/// No suggestions available
	None,
	/// A single match has been found, the line buffer can be immediately expanded with the new command
	SingleMatch { line: AutocompleteLine },	
	/// Multiple matches, usually they can be presented to the end user in a column format.
	MultipleMatches { lines: Vec<AutocompleteLine> }
}

/// One autocomplete suggestion
#[derive(Debug, Clone)]
pub struct AutocompleteLine {
	/// The entire new suggested line buffer
	pub full_new_line: String,
	/// The additional suggested part of the buffer, can be sent to the terminal device
	pub additional_part: String
}

fn collect_options(line: &str, cmds: &mut [Box<CliCommand + Send + 'static>]) -> Vec<AutocompleteOption> {
	let mut ret = Vec::new();
	for cmd in cmds.iter() {
		let options = cmd.autocomplete(line);
		if let Some(options) = options {
			for option in options.iter() {
				ret.push(option.clone());
			}
		}
	}
	ret
}

/// Collect autocomplete suggestions for this line buffer
pub fn cli_try_autocomplete(line: &str, cmds: &mut [Box<CliCommand + Send + 'static>]) -> AutocompleteResult {
	// check if any command matches, ignore autocomplete in that case - for now
	for ref mut cmd in cmds.iter_mut() {
		if !cmd.is_match(line) {
			continue;
		}

		return AutocompleteResult::None;
	}


	let fl = collect_options(line, cmds);

	let mut matches = Vec::new();
	for opt in fl.iter() {
		match opt {
			&AutocompleteOption::FullCommand { line: ref line } => {
				matches.push(line.clone());
			}
			_ => {}
		}
	}

	match matches.len() {
		0 => AutocompleteResult::None,
		1 => {
			let ref m = matches[0];
			let c = m.chars().skip(line.len()).collect();
			let l = AutocompleteLine { full_new_line: m.clone(), additional_part: c };
			AutocompleteResult::SingleMatch { line: l }
		}
		_ => {			
			let mut lines = Vec::new();
			for m in matches.iter() {
				let c = m.chars().skip(line.len()).collect();
				let l = AutocompleteLine { full_new_line: m.clone(), additional_part: c };
				lines.push(l);
			}

			// sort the lines
			lines.sort_by(|a, b| { a.full_new_line.cmp(&b.full_new_line) });

			let lcp = {
				let mut strings = Vec::new();
				for m in lines.iter() {
					strings.push(m.full_new_line.as_str());
				}

				let lcp = longest_common_prefix(strings.as_slice());
				if let Some(lcp) = lcp {
					if lcp.len() == line.len() {
						None
					} else {
						Some(lcp)
					}
				} else {
					None
				}
			};



			if let Some(lcp) = lcp {
				//println!("lcp: {}", lcp);
				AutocompleteResult::SingleMatch { 
					line: AutocompleteLine {
						full_new_line: lcp.clone(),
						additional_part: lcp.chars().skip(line.len()).collect()
					}
				}
			} else {
				AutocompleteResult::MultipleMatches { lines: lines }
			}
		}
	}
}

pub struct CliTerminalNull;
impl CliTerminal for CliTerminalNull {
	fn output_line(&mut self, line: &str) {		
	}
}