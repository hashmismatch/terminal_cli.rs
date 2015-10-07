use prelude::v1::*;
use terminal::*;

/// Context for the execution of the command
pub struct CommandContext<'b> {
	pub args: Cow<'b, str>,
	pub terminal: &'b mut CharacterTerminalWriter,
	pub current_path: &'b str
}

impl<'b> CommandContext<'b> {
	pub fn get_args(&self) -> &str {
		&self.args
	}

	pub fn get_terminal(&mut self) -> &mut CharacterTerminalWriter {
		self.terminal
	}

	pub fn get_current_path(&self) -> &str {
		&self.current_path
	}
}
