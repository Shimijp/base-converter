use crate::utils::{get_sign, ConversionError, ALPHABET};

#[derive(Debug, Copy, Clone)]
pub enum  Bases
{
    Binary,
    Octal,
    Decimal,
    Hexadecimal
}

pub fn convert(input: &str, source_base: &Bases, target_base: &Bases) -> Result<String, ConversionError>
{
    Ok("".to_string())
}

pub fn convert_dec_to_hex(input: &str) -> Result<String, ConversionError>
{


    let mut sign = 1;

    let mut string = input;
    if input.starts_with('-') || input.starts_with('+')
    {
        sign = get_sign(&input);
        string = &input[1..];

    }
    let mut num  = convert_srt_to_num(string, Bases::Decimal)?;
    num = match sign{
        -1 => num * -1,
        _ => num
    };
    let mut unum = num as u64;
    let mut hex = String::new();
    if unum == 0 {

        return Ok("0".to_string());
    }
    while unum > 0 {
        let dig  = ALPHABET[(unum % 16) as usize];
        hex.push(dig as char);
        unum /= 16;
    }
    Ok(hex.chars().rev().collect::<String>())




}
fn convert_srt_to_num(string: &str, source_base :Bases) -> Result<i64, ConversionError>
{
    if string.is_empty()
    {
        return Err(ConversionError::EmptyInput);
    }
    let mut num :i64 = 0;
    let base:i64 = match  source_base{
        Bases::Decimal => 10,
        Bases::Binary => 2,
        Bases::Octal => 8,
        Bases::Hexadecimal => 16,
    };
    for ch in string.chars()
    {

        let  digit = ch.to_digit(base as u32).ok_or(ConversionError::IllegalCharacter(ch))?;

        num=  num.checked_mul(base)
        .and_then(|res| res.checked_add(digit as i64))
        .ok_or(ConversionError::SourceOverflow)?;

    }
    Ok(num)
}
