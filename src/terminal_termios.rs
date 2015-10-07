extern crate termios;
use self::termios::*;
use terminal::*;
use keys::*;
use keys_terminal::*;

use std::io::prelude::*;
use std::io::{stdout, stdin, Bytes};

use std::fmt::Write as FmtWrite;
use std::fmt::Error as FmtError;

pub struct TerminalTermios {
	termios: Termios,
	termios_startup: Termios,

	decoder: TerminalKeyDecoder
}

impl TerminalTermios {
	pub fn new() -> TerminalTermios {

        let mut termios = Termios::from_fd(0).unwrap();
        let termios_startup = termios.clone();

        termios.c_lflag &= !(ICANON | IEXTEN | ISIG | ECHO);
        tcsetattr(0, TCSANOW, &termios).unwrap();
        tcflush(0, TCIOFLUSH).unwrap();

        TerminalTermios {
        	termios: termios,
        	termios_startup: termios_startup,
        	decoder: TerminalKeyDecoder::new()
        }
	}
}

impl Drop for TerminalTermios {
	fn drop(&mut self) {
		tcsetattr(0, termios::TCSANOW, &self.termios_startup).unwrap();
	}
}

impl CharacterTerminalWriter for TerminalTermios {
	fn print(&mut self, bytes: &[u8]) {
		stdout().write(bytes);
		stdout().flush().unwrap();
	}
}

impl FmtWrite for TerminalTermios {
    fn write_str(&mut self, s: &str) -> Result<(), FmtError> {
        self.print(s.as_bytes());
        Ok(())
    }
}

impl CharacterTerminalReader for TerminalTermios {
	fn read(&mut self) -> Result<Key, TerminalError> {
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
