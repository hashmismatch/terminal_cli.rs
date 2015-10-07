// todo: rewrite the parsers using nom!

use prelude::v1::*;
use keys::*;
use keys_terminal::*;

const IAC: u8 = 255;
const WILL: u8 = 251;
const WONT: u8 = 252;
const DO: u8 = 253;
const DONT: u8 = 254;
const NOP: u8 = 241;
const SB: u8 = 250;
const SE: u8 = 240;
const IS: u8 = 0;
const SEND: u8 = 1;

const CHARSET: u8 = 42;
const NAWS: u8 = 31;

#[derive(Debug, Copy, Clone, PartialEq)]
enum State {
	Negotiation,
	Main
}

pub struct TelnetTerminal {
	state: State,
	//terminal: TerminalKeyDecoder
}

pub struct TelnetTerminalState {
	echo: bool,
	suppress: bool
}

#[derive(Debug, Clone, PartialEq)]
pub enum TelnetStream {
	Command(TelnetCommand, TelnetSubcommand),
	Suboption(TelnetSubcommand, Vec<u8>),
	CtrlC,
	CtrlD,
	Data(u8)
}

impl TelnetStream {
	pub fn to_network(&self) -> Vec<u8> {
		let mut v = Vec::new();
		match *self {
			TelnetStream::Command(ref a, ref b) => {
				v.push(IAC);
				v.push(a.to_val());
				v.push(b.to_val());
			},
			TelnetStream::Suboption(ref a, ref b) => {
				v.push(IAC);
				v.push(SB);
				v.push(a.to_val());
				v.extend_from_slice(&b);
				v.push(IAC);
				v.push(SE);
			},
			TelnetStream::CtrlC => { v.push(236); }
			TelnetStream::CtrlD => { v.push(248); },
			TelnetStream::Data(d) => { v.push(d); }
		}
		v
	}

	pub fn all_to_network(stream: &[TelnetStream]) -> Vec<u8> {
		let mut v = Vec::new();
		for s in stream {
			v.extend_from_slice(&s.to_network());
		}
		v
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum TelnetCommand {
	Will,
	Wont,
	Do,
	Dont,
	Nop,
	Suboption,
	SuboptionEnd
}

impl TelnetCommand {
	pub fn parse(val: u8) -> Option<Self> {
		match val {
			WILL => Some(TelnetCommand::Will),
			WONT => Some(TelnetCommand::Wont),
			DO => Some(TelnetCommand::Do),
			DONT => Some(TelnetCommand::Dont),
			NOP => Some(TelnetCommand::Nop),
			SB => Some(TelnetCommand::Suboption),
			SE => Some(TelnetCommand::SuboptionEnd),
			_ => None
		}
	}

	pub fn to_val(&self) -> u8 {
		match *self {
			TelnetCommand::Will => WILL,
			TelnetCommand::Wont => WONT,
			TelnetCommand::Do => DO,
			TelnetCommand::Dont => DONT,
			TelnetCommand::Nop => NOP,
			TelnetCommand::Suboption => SB,
			TelnetCommand::SuboptionEnd => SE
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum TelnetSubcommand {
	Echo,
	SuppressGoAhead,
	NegotiateAboutWindowSize,
	TerminalSpeed,
	TerminalType,
	NewEnvironmentOption
}

impl TelnetSubcommand {
	pub fn parse(val: u8) -> Option<Self> {
		match val {
			1 => Some(TelnetSubcommand::Echo),
			3 => Some(TelnetSubcommand::SuppressGoAhead),
			0x1f => Some(TelnetSubcommand::NegotiateAboutWindowSize),
			0x20 => Some(TelnetSubcommand::TerminalSpeed),
			0x18 => Some(TelnetSubcommand::TerminalType),
			0x27 => Some(TelnetSubcommand::NewEnvironmentOption),
			_ => None
		}
	}

	pub fn to_val(&self) -> u8 {
		match *self {
			TelnetSubcommand::Echo => 1,
			TelnetSubcommand::SuppressGoAhead => 3,
			TelnetSubcommand::NegotiateAboutWindowSize => 0x1f,
			TelnetSubcommand::TerminalSpeed => 0x20,
			TelnetSubcommand::TerminalType => 0x18,
			TelnetSubcommand::NewEnvironmentOption => 0x27
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TelnetError {
	MoreDataRequired
}

pub enum TelnetEvent {
	None,
	ReceivedData(Vec<u8>),
	ServerResponse(Vec<TelnetStream>)
}


impl TelnetTerminal {
	pub fn new() -> Self {
		TelnetTerminal {
			state: State::Negotiation
		}
	}

	pub fn handle_input(&mut self, data: &[u8]) -> Result<TelnetEvent, TelnetError> {
		let parsed = Self::parse(data);

		let mut data = Vec::new();
		let mut commands = Vec::new();

		for p in parsed {
			match p {
				TelnetStream::Data(d) => { data.push(d); },
				TelnetStream::CtrlC => { data.push(0x3); },
				TelnetStream::CtrlD => { data.push(0x4); },
				c @ TelnetStream::Command(..) => { commands.push(c); },
				c @ TelnetStream::Suboption(..) => { commands.push(c); }				
			}
		}

		if commands.len() > 0 {
			try!(self.handle_commands(&commands));
		}

		if data.len() > 0 {
			return Ok(TelnetEvent::ReceivedData(data));
		}		

		Ok(TelnetEvent::None)
	}

	fn handle_commands(&mut self, commands: &[TelnetStream]) -> Result<TelnetEvent, TelnetError> {
		// todo
		//trace!("received commands: {:?}", commands);
		Ok(TelnetEvent::ServerResponse(Vec::new()))
	}


	pub fn hello(&mut self) -> Vec<TelnetStream> {
		let mut v = Vec::new();

		if self.state == State::Negotiation {
			self.state = State::Main;

			v.push(TelnetStream::Command(TelnetCommand::Do, TelnetSubcommand::Echo));
			v.push(TelnetStream::Command(TelnetCommand::Will, TelnetSubcommand::Echo));
			v.push(TelnetStream::Command(TelnetCommand::Will, TelnetSubcommand::SuppressGoAhead));

		}

		v
	}

	pub fn parse(data: &[u8]) -> Vec<TelnetStream> {
		let mut ret = Vec::new();
		let mut i = data.iter();

		loop {
			let x = match i.next().cloned() {
				Some(IAC) => match i.next().cloned() {
					Some(IAC) => TelnetStream::Data(255),
					Some(236) => TelnetStream::CtrlC,
					Some(248) => TelnetStream::CtrlD,
					Some(a) => {
						match TelnetCommand::parse(a) {
							Some(TelnetCommand::Suboption) => {
								match i.next().cloned().map(TelnetSubcommand::parse) {
									Some(Some(sub_cmd)) => {
										let mut data = Vec::new();
										loop {
											match i.next().cloned() {
												Some(d) => data.push(d),
												None => break
											}
										}

										let l = data.len();
										if l >= 2 {
											match (data.get(l-2).cloned(), data.get(l-1).cloned()) {
												(Some(IAC), Some(SE)) => {
													data.pop();
													data.pop();
												},
												_ => ()
											}
										}

										TelnetStream::Suboption(sub_cmd, data)
									},
									_ => { continue; }
								}
							},
							Some(cmd) => match i.next().cloned() {								
								Some(b) => match TelnetSubcommand::parse(b) {
									Some(sub_cmd) => {
										TelnetStream::Command(cmd, sub_cmd)
									},
									None => { continue; }
								},
								None => { continue; }
							},
							None => { continue; }
						}
					},
					None => { continue; }
				},
				Some(d) => { TelnetStream::Data(d) },
				None => { break; }
			};

			ret.push(x);
		}

		ret
	}
}


#[test]
fn test_telnet_dec() {
	let client_hello = [
		0xff, 0xfb, 0x1f, 0xff, 0xfb, 0x20, 0xff, 0xfb, 0x18, 0xff, 0xfb, 0x27, 0xff, 0xfd, 0x01, 0xff,
		0xfb, 0x03, 0xff, 0xfd, 0x03
	];

	let s = TelnetTerminal::parse(&client_hello);

	assert_eq!(&s, 
		&[
			TelnetStream::Command(TelnetCommand::Will, TelnetSubcommand::NegotiateAboutWindowSize),
			TelnetStream::Command(TelnetCommand::Will, TelnetSubcommand::TerminalSpeed),
			TelnetStream::Command(TelnetCommand::Will, TelnetSubcommand::TerminalType),
			TelnetStream::Command(TelnetCommand::Will, TelnetSubcommand::NewEnvironmentOption),
			TelnetStream::Command(TelnetCommand::Do, TelnetSubcommand::Echo),
			TelnetStream::Command(TelnetCommand::Will, TelnetSubcommand::SuppressGoAhead),
			TelnetStream::Command(TelnetCommand::Do, TelnetSubcommand::SuppressGoAhead)
		]
	);

	let n = TelnetStream::all_to_network(&s);
	assert_eq!(&n, &client_hello);

	let window_size = [
		0xff, 0xfa, 0x1f, 0x00, 0x50, 0x00, 0x18, 0xff, 0xf0
	];
	let s = TelnetTerminal::parse(&window_size);
	assert_eq!(&s, &[TelnetStream::Suboption(TelnetSubcommand::NegotiateAboutWindowSize, vec![0, 80, 0, 24])]);

	let n = TelnetStream::all_to_network(&s);
	assert_eq!(&n, &window_size);
}