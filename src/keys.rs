
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Key {	
    Arrow(Direction),
    Backspace,
    Tab,
    Newline,
    Break,
    Eot,
    Character(u8)
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeyDecoderError {
    UnknownSequence,
    MoreInputRequired
}

pub trait KeyDecoder {
    fn decode(&mut self, byte: u8) -> Result<Key, KeyDecoderError>;
}
