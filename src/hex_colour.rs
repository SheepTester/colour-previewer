//! A module for parsing hexadecimal colour codes.

use std::str::FromStr;
use warp::reject::Reject;

/// Contains a colour in RGB form.
pub struct HexColour(u8, u8, u8);

impl HexColour {
    /// Returns a byte array with the red, green, and blue channels of the
    /// colour.
    pub fn channels(&self) -> [u8; 3] {
        [self.0, self.1, self.2]
    }
}

impl FromStr for HexColour {
    type Err = HexColourParseError;

    /// Determines whether the hex colour code is perfect (6 digits, all
    /// lowercase). If not, the string parsing fails, but the error will
    /// contain the normalised form.
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

/// An error from calling `.parse::<HexColour>()` on a string.
pub enum HexColourParseError {
    /// Indicates that the hex colour code isn't malformed but should be
    /// normalised into the perfect, ideal 6-digit all lowercase form. The web
    /// server uses this to redirect the client to the normalised colour
    /// preview page.
    Redirect(String),
    /// The string is not a valid hex colour code, or some other mysterious
    /// error occurred while parsing the string as a hexadecimal integer.
    Invalid,
}

/// An rejection for the web server to use if the string is not a valid hex
/// colour code.
#[derive(Debug)]
pub struct InvalidHexColour;

impl Reject for InvalidHexColour {}
