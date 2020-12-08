extern crate terminal_cli;
extern crate crossterm;

use terminal_cli::{CharacterTerminalReader, CharacterTerminalWriter, KeyDecoder, KeyDecoderError, TerminalError, TerminalKeyDecoder};
use crossterm::{ExecutableCommand, event::KeyCode, cursor};

use std::io::{stdout, Write};

pub struct TerminalCrossterm {
    stdout: std::io::Stdout
}

impl TerminalCrossterm {
	pub fn new() -> Self {
		let mut stdout = stdout();
		stdout.execute(cursor::Show);

        TerminalCrossterm {         
        	stdout
        }
	}
}

impl Drop for TerminalCrossterm {
	fn drop(&mut self) {
		self.stdout.execute(cursor::Show);
	}
}

impl CharacterTerminalWriter for TerminalCrossterm {
	fn print(&mut self, bytes: &[u8]) {
		self.stdout.write(bytes);
		self.stdout.flush();
	}
}

impl std::fmt::Write for TerminalCrossterm {
    fn write_str(&mut self, s: &str) -> Result<(), std::fmt::Error> {
        self.print(s.as_bytes());
        Ok(())
    }
}

impl CharacterTerminalReader for TerminalCrossterm {
	fn read(&mut self) -> Result<::terminal_cli::Key, TerminalError> {
		use crossterm::event::Event;
		use terminal_cli::Key;

		loop {
			match crossterm::event::read() {
				Ok(Event::Key(ev)) => {
					let keycode = ev.code;
					let modifiers = ev.modifiers;
					

					let k = match keycode {
						KeyCode::Enter => Key::Newline,
						KeyCode::Tab => Key::Tab,
						KeyCode::Char(c) if c.is_ascii() => Key::Character(c as u8),
						KeyCode::Backspace => Key::Backspace,
						_ => { continue; }
					};

					return Ok(k);
				},
				_ => ()
			}
		}
	}
}
