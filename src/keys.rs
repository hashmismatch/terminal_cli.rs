
/// Direction key
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DirectionKey {
    Up,
    Down,
    Left,
    Right
}

/// Decoded key press
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Key {	
    Arrow(DirectionKey),
    Backspace,
    Tab,
    Newline,
    CarriageReturn,
    Break,
    Eot,
    Character(u8)
}

/// Key decoder error
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeyDecoderError {
    UnknownSequence,
    MoreInputRequired
}

/// Key decoder trait
pub trait KeyDecoder {
    fn decode(&mut self, byte: u8) -> Result<Key, KeyDecoderError>;
}
