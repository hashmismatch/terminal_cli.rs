use prelude::v1::*;
use keys::*;

/// Terminal key decoder, from raw bytes to decoded key sequences
pub struct TerminalKeyDecoder {
    buffer: Vec<u8>
}

impl TerminalKeyDecoder {
    /// Create a new decoder
	pub fn new() -> TerminalKeyDecoder {
		TerminalKeyDecoder {
			buffer: Vec::new()
		}
	}
}

impl KeyDecoder for TerminalKeyDecoder {
	fn decode(&mut self, byte: u8) -> Result<Key, KeyDecoderError> {
		self.buffer.push(byte);

		let r = match self.buffer[0] {
			// ESC
			0x1B => {
                match self.buffer.get(1).cloned() {
					Some(0x5B) => {
						match self.buffer.get(2).cloned() {
							Some(0x41) => {
                                Ok(Key::Arrow(DirectionKey::Up))
                            },
                            Some(0x42) => {
                                Ok(Key::Arrow(DirectionKey::Down))
                            },
                            Some(0x43) => {
                                Ok(Key::Arrow(DirectionKey::Right))
                            },
                            Some(0x44) => {
                                Ok(Key::Arrow(DirectionKey::Left))
                            },
                            Some(_) => Err(KeyDecoderError::UnknownSequence),
                            None => Err(KeyDecoderError::MoreInputRequired)
						}
					},
					Some(_) => Err(KeyDecoderError::UnknownSequence),
					None => Err(KeyDecoderError::MoreInputRequired)
				}
			},

            0x7F => {
                Ok(Key::Backspace)
            },
            0x09 => {
                Ok(Key::Tab)
            },            
            0xA => {
                Ok(Key::Newline)
            },
            0x3 => {
                Ok(Key::Break)
            },
            0xD => {
                Ok(Key::CarriageReturn)
            },
            4 => {
                Ok(Key::Eot)
            },
            c => {
            	Ok(Key::Character(c))
            }
		};

        match r {
            Ok(_) | Err(KeyDecoderError::UnknownSequence) => {
                self.buffer.clear()
            },
            Err(KeyDecoderError::MoreInputRequired) => {
                
            }
        }

		r
	}
}

#[test]
fn test_escape_sequences() {
    {
        let mut decoder = TerminalKeyDecoder::new();
        assert_eq!(Err(KeyDecoderError::MoreInputRequired), decoder.decode(27));
        assert_eq!(Err(KeyDecoderError::MoreInputRequired), decoder.decode(91));
        assert_eq!(Ok(Key::Arrow(DirectionKey::Up)), decoder.decode(65));
    }
}