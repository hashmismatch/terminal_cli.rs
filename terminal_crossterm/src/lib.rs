extern crate terminal_cli;
extern crate crossterm;


use terminal_cli::{CharacterTerminalReader, CharacterTerminalWriter, KeyDecoder, KeyDecoderError, TerminalError, TerminalKeyDecoder};
use crossterm::{ExecutableCommand, event::KeyCode, cursor};

use std::io::{stdout, Write};


pub struct TerminalCrossterm {
	//decoder: TerminalKeyDecoder,
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
        //self.stdout.flush().unwrap();
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
						_ => { continue; }
					};

					return Ok(k);

					/*
					let d = self.decoder.decode(b);
					match d {
						Ok(k) => {
							return Ok(k);
						},
						Err(KeyDecoderError::MoreInputRequired) => {
							continue;
						},
						Err(KeyDecoderError::UnknownSequence) => {
							continue;
						}
					}
					*/
				},
				_ => ()
			}
		}

		//let mut input = stdin().bytes();

        /*
		loop {
			match input.next() {
				Some(Ok(b)) => {
					let d = self.decoder.decode(b);
					match d {
						Ok(k) => {
							return Ok(k);
						},
						Err(KeyDecoderError::MoreInputRequired) => {
							continue;
						},
						Err(KeyDecoderError::UnknownSequence) => {
							continue;
						}
					}
				},
				Some(Err(_)) => {
					return Err(TerminalError::Error);
				},
				None => {
					return Err(TerminalError::EndOfStream);
				}
			}
        }
        */
	}
}
