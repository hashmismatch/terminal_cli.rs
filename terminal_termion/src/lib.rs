extern crate terminal_cli;
extern crate termion;
#[cfg(unix)]
extern crate libc;

use terminal_cli::*;

use self::termion::*;
use self::termion::event::Key;
use self::termion::input::TermRead;
use self::termion::raw::IntoRawMode;

use std::io::prelude::*;
use std::io::{stdout, stdin, Bytes, Write, Stdout};
use std::fmt::Write as FmtWrite;
use std::fmt::Error as FmtError;


#[cfg(unix)]
fn init_cmode() {
	use libc::c_int;
	use std::mem;

	use libc::termios as Termios;

	extern "C" {
		fn tcgetattr(fd: c_int, termptr: *mut Termios) -> c_int;
		fn tcsetattr(fd: c_int, opt: c_int, termptr: *mut Termios) -> c_int;
		fn cfmakeraw(termptr: *mut Termios);
	}

	fn get_terminal_attr() -> (Termios, c_int) {
		unsafe {
			let mut ios = mem::zeroed();
			let attr = tcgetattr(0, &mut ios);
			(ios, attr)
		}
	}

	fn set_terminal_attr(ios: *mut Termios) -> c_int {
		unsafe { tcsetattr(0, 0, ios) }
	}

	let (mut ios, exit) = get_terminal_attr();
	if exit != 0 {
		return;
	}

	ios.c_oflag |= libc::ONLCR | libc::OPOST;

    if set_terminal_attr(&mut ios as *mut _) != 0 {
    	return;
	}
}

#[cfg(not(unix))]
fn init_cmode() {

}


pub struct TerminalTermion {
	decoder: TerminalKeyDecoder,
    stdout: self::termion::raw::RawTerminal<Stdout>
}

impl TerminalTermion {
	pub fn new() -> Self {
        //self::termion::init();

        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();

		init_cmode();
		
        write!(stdout, "{}", termion::cursor::Show).unwrap();		
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
	fn read(&mut self) -> Result<::terminal_cli::Key, TerminalError> {
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
