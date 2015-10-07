use prelude::v1::*;
use keys::*;

#[derive(Debug, Clone, PartialEq)]
pub enum TerminalError {
	Error,
	EndOfStream
}

/// Terminal trait.
pub trait LineTerminalWriter {
	/// Output a string with the newline characters at the end. The implementation
	/// adds the newline control characters.
	fn print_line(&mut self, line: &str);	
}

pub trait CharacterTerminalWriter {
	fn print(&mut self, bytes: &[u8]);
    fn print_str(&mut self, s: &str) {
        self.print(s.as_bytes())
    }
    fn print_str_line(&mut self, s: &str) {
        self.print(s.as_bytes());
        self.print(b"\r\n");
    }
}

pub trait CharacterTerminalReader {
	fn read(&mut self) -> Result<Key, TerminalError>;
}


impl Write for CharacterTerminalWriter {
	fn write_str(&mut self, s: &str) -> Result<(), Error> {
		self.print_str(s);
		Ok(())
	}
}