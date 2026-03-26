use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub enum ConversionError {
    EmptyInput,
    SourceOverflow,
    IllegalCharacter(char),
}

impl Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConversionError::EmptyInput => write!(f, "Error: Input cannot be empty!"),
            ConversionError::IllegalCharacter(c) => write!(f, "Error: The character '{}' is not valid for this base.", c),
            ConversionError::SourceOverflow => write!(f, "Error: The number is too large for a 64-bit integer."),
           
        }
    }
}
pub const ALPHABET: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";
pub fn get_sign(string: &str) -> i32
{
    let mut sign: i32 = 1;
    if string.starts_with('-') {
        sign = -sign;
    };
    sign
}
