extern crate termion;

use self::termion::*;
use self::termion::event::Key;
use self::termion::input::TermRead;
use self::termion::raw::IntoRawMode;

use terminal::*;
use keys::*;
use keys_terminal::*;

use std::io::prelude::*;
use std::io::{stdout, stdin, Bytes, Write, Stdout};
use std::fmt::Write as FmtWrite;
use std::fmt::Error as FmtError;

pub struct TerminalTermion {
	decoder: TerminalKeyDecoder,
    stdout: self::termion::raw::RawTerminal<Stdout>
}

impl TerminalTermion {
	pub fn new() -> Self {
        self::termion::init();
    
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();

        write!(stdout,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide).unwrap();
        stdout.flush().unwrap();

        TerminalTermion {         
        	decoder: TerminalKeyDecoder::new(),
            stdout: stdout
        }
	}
}

impl Drop for TerminalTermion {
	fn drop(&mut self) {
		write!(self.stdout, "{}", self::termion::cursor::Show).unwrap();
	}
}

impl CharacterTerminalWriter for TerminalTermion {
	fn print(&mut self, bytes: &[u8]) {
		self.stdout.write(bytes);
		self.stdout.flush().unwrap();
	}
}

impl FmtWrite for TerminalTermion {
    fn write_str(&mut self, s: &str) -> Result<(), FmtError> {
        self.print(s.as_bytes());
        Ok(())
    }
}

impl CharacterTerminalReader for TerminalTermion {
	fn read(&mut self) -> Result<::keys::Key, TerminalError> {
		let mut input = stdin().bytes();


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


	}
}
