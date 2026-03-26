

#[derive(Debug, Copy, Clone)]
pub enum ConversionError {
    EmptyInput,
    SourceOverflow,
    IllegalCharacter(char),
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
