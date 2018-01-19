use prelude::v1::*;
use property::*;
use terminal::*;
use autocomplete::*;
use cli_property::*;
use cli_command::*;
use super::i18n::Strings;

pub trait CliContext<'a> {
	/// Creates a new prefixed execution context, but only if the current line matches. Reduces the
	/// processing overhead for large tree command environments.
	fn with_prefix<'b>(&'b mut self, prefix: &str) -> Option<PrefixedExecutor<'a, 'b>>;

	/// Announces a command to be executed. Returns an execution context in case the command is invoked.
	fn command<'b>(&'b mut self, cmd: &str) -> Option<CommandContext<'b>>;

	/// Announces a property that can be manipulated. Returns an execution context in case the property
	/// is to be either retrieved or updated.
	fn property<'b, V, P, Id: Into<Cow<'b, str>>>(&'b mut self, property_id: Id, input_parser: P) -> Option<PropertyContext<'b, V>> where P: ValueInput<V>, V: Display;
}

/// Helper for matching commands and properties against an input line.
pub struct CliExecutor<'a> {
	matcher: CliLineMatcher<'a>,
	strings: &'a Strings,
	terminal: &'a mut CharacterTerminalWriter
}

impl<'a> CliContext<'a> for CliExecutor<'a> {	
	fn with_prefix<'b>(&'b mut self, prefix: &str) -> Option<PrefixedExecutor<'a, 'b>> {
		if self.matcher.starts_with(&prefix) {
			let p = PrefixedExecutor {
				prefix: prefix.to_string().into(),
				executor: self
			};

			return Some(p);
		} else {
			self.matcher.add_unmatched_prefix(&prefix);
		}
		
		None
	}
	
	fn command<'b>(&'b mut self, cmd: &str) -> Option<CommandContext<'b>> {

		if self.matcher.match_cmd_str(cmd, None) == LineMatcherProgress::MatchFound {
			let args = if let &LineBufferResult::Match { ref args, .. } = self.matcher.get_state() {
				Some(args.clone())
			} else {
				None
			};

			if let Some(args) = args {
				let ctx = CommandContext {
					args: args.into(),
					terminal: self.terminal,
					current_path: ""
				};
				
				return Some(ctx);
			}
		}

		None
	}
		
	fn property<'b, V, P, Id: Into<Cow<'b, str>>>(&'b mut self, property_id: Id, input_parser: P) -> Option<PropertyContext<'b, V>> where P: ValueInput<V>, V: Display {
		let property_id: Cow<str> = property_id.into();

		if self.matcher.match_cmd_str(&format!("{}/get", property_id), None) == LineMatcherProgress::MatchFound {
			let args = if let &LineBufferResult::Match { ref args, .. } = self.matcher.get_state() {
				args.clone()
			} else {
				"".into()
			};

			return Some(PropertyContext::Get(PropertyContextGet {
				common: PropertyContextCommon {
					args: args.into(),
					terminal: self.terminal,
					current_path: "",
					id: property_id,
					style: PropertyCommandStyle::DelimitedGetSet,
					strings: &*self.strings
				}
			}));
		}

		if self.matcher.match_cmd_str(&format!("{}/set", property_id), None) == LineMatcherProgress::MatchFound {
			let args = if let &LineBufferResult::Match { ref args, .. } = self.matcher.get_state() {
				args.trim()
			} else {
				"".into()
			};

			match input_parser.input(&args) {
				Ok(val) => {
					return Some(PropertyContext::Set(PropertyContextSet {
						common: PropertyContextCommon {
							args: args.into(),
							terminal: self.terminal,
							current_path: "",
							id: property_id,
							style: PropertyCommandStyle::DelimitedGetSet,
							strings: &*self.strings
						},
						value: val
					}));
				},
				Err(e) => {

					match e {
						PropertyValidationError::InvalidInput => {
							self.strings.property_invalid_value(self.terminal, &property_id, &args);
						},
						PropertyValidationError::ValueTooSmall { min, val } => {
							self.strings.property_value_too_small(self.terminal, &property_id, &val, &min);
						},
						PropertyValidationError::ValueTooBig { max, val } => {
							self.strings.property_value_too_big(self.terminal, &property_id, &val, &max);
						}
					}

					self.terminal.newline();
				}
			}
		}

		None
	}
}

impl<'a> CliExecutor<'a> {
	pub fn new<T: CharacterTerminalWriter>(matcher: CliLineMatcher<'a>, strings: &'a Strings, terminal: &'a mut T) -> Self {
		CliExecutor {
			matcher: matcher,
			strings: strings,
			terminal: terminal
		}
	}

	/// Finish the execution of this line invocation.
	pub fn close(self) -> CliLineMatcher<'a> {
		self.matcher
	}


	

	/// Get the associated terminal.
	pub fn get_terminal(&mut self) -> &mut CharacterTerminalWriter {
		self.terminal
	}
}

impl<'a> Deref for CliExecutor<'a> {
	type Target = &'a mut CharacterTerminalWriter;

    fn deref<'b>(&'b self) -> &'b &'a mut CharacterTerminalWriter {
        &self.terminal
    }
}

pub struct PrefixedExecutor<'a: 'p, 'p> {
	prefix: Cow<'p, str>,
	executor: &'p mut CliExecutor<'a>
}

impl<'a, 'p> PrefixedExecutor<'a, 'p> {
	fn add_prefix(&self, cmd: &str) -> String {
		format!("{}{}", self.prefix, cmd)
	}
}


impl<'a, 'p> CliContext<'a> for PrefixedExecutor<'a, 'p> {
	fn with_prefix<'b>(&'b mut self, prefix: &str) -> Option<PrefixedExecutor<'a, 'b>> {
		let prefix = self.add_prefix(prefix);
		self.executor.with_prefix(&prefix)
	}

	fn command<'b>(&'b mut self, cmd: &str) -> Option<CommandContext<'b>> {
		let cmd = self.add_prefix(cmd);

		self.executor.command(&cmd)
	}
	
	fn property<'b, V, P, Id: Into<Cow<'b, str>>>(&'b mut self, property_id: Id, input_parser: P) -> Option<PropertyContext<'b, V>> where P: ValueInput<V>, V: Display {
		let property_id: Cow<str> = property_id.into();
		let property_id = self.add_prefix(&property_id);

		self.executor.property(property_id, input_parser)
	}
}