use std::str::FromStr;

pub struct HexColour(u8, u8, u8);

impl FromStr for HexColour {
    type Err = HexColourParseError;

    fn from_str(source: &str) -> Result<HexColour, Self::Err> {
        let chars: Vec<char> = source.chars().collect();
        match chars.len() {
            3 => {
                if chars.iter().all(|&char| char.is_ascii_hexdigit()) {
                    Err(HexColourParseError::Redirect(
                        format!("{0}{0}{1}{1}{2}{2}", chars[0], chars[1], chars[2]).to_lowercase(),
                    ))
                } else {
                    Err(HexColourParseError::Invalid)
                }
            }
            6 => {
                if chars.iter().any(|&char| char.is_uppercase()) {
                    Err(HexColourParseError::Redirect(source.to_lowercase()))
                } else if let Ok(int) = u32::from_str_radix(source, 16) {
                    let bytes = int.to_be_bytes();
                    // First byte *should* be 0x00
                    Ok(HexColour(bytes[1], bytes[2], bytes[3]))
                } else {
                    Err(HexColourParseError::Invalid)
                }
            }
            _ => Err(HexColourParseError::Invalid),
        }
    }
}

pub enum HexColourParseError {
    Redirect(String),
    Invalid,
}
