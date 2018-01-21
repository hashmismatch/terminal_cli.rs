use prelude::v1::*;
use keys::*;

#[derive(Debug, Clone, PartialEq)]
pub enum TerminalError {
	Error,
	EndOfStream
}

/// Character based terminal trait
pub trait CharacterTerminalWriter : FmtWrite {
	fn print(&mut self, bytes: &[u8]);
    fn print_str(&mut self, s: &str) {
        self.print(s.as_bytes())
    }
    fn print_line(&mut self, s: &str) {
        self.print(s.as_bytes());
		self.print_newline_sequence();
    }

	fn write_str(&mut self, s: &str) -> Result<(), FmtError> {
		self.print_str(s);
		Ok(())
	}

	fn newline(&mut self) {
		self.print_newline_sequence()
	}
	
	fn print_newline_sequence(&mut self) {
		self.print_str("\r\n");
	}
}

/// Terminal key reader
pub trait CharacterTerminalReader {
	fn read(&mut self) -> Result<Key, TerminalError>;
}

#[cfg(feature="std")]
pub struct StdoutTerminal;

#[cfg(feature="std")]
impl CharacterTerminalWriter for StdoutTerminal {
	fn print(&mut self, bytes: &[u8]) {
		io::stdout().write(bytes).unwrap();
	}
}

#[cfg(feature="std")]
impl FmtWrite for StdoutTerminal {
	fn write_str(&mut self, s: &str) -> Result<(), FmtError> {
		self.print_str(s);
		Ok(())
	}
}
