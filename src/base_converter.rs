

#[derive(Debug, Copy, Clone)]
pub enum  Bases
{
    Binary,
    Octal,
    Decimal,
    Hexadecimal
}

pub fn convert(input: &str, source_base: &Bases, target_base: &Bases) -> Result<String, String>
{
    Ok("".to_string())
}

