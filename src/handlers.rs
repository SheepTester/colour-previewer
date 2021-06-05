//! Handlers handle arguments from a URL and provide a response. Most of the
//! magic happens here.
//!
//! See [super::filters] for the URL routes that correspond with each handler.

use super::hex_colour::{HexColour, HexColourParseError, InvalidHexColour};
use crc32fast::Hasher;
use rand::random;
use warp::{
    http::Response,
    reject::{custom, Rejection},
    Reply,
};

/// Handlers for experimenting with and learning about Warp.
pub mod fooling_around {
    use std::convert::Infallible;
    use warp::{
        http::Response,
        reject::{custom, Reject, Rejection},
        Reply,
    };

    /// Responds with an int from the URL. Used to determine what happens if
    /// the int is too large or invalid.
    pub async fn return_int(int: u32) -> Result<impl Reply, Infallible> {
        Ok(Response::builder()
            .header("my-custom-header", "some-value")
            .body(format!("here is cool int: {}", int)))
    }

    /// An error type for [three_string]
    #[derive(Debug)]
    pub enum ThreeStringFailure {
        TooLong,
        TooShort,
    }

    impl Reject for ThreeStringFailure {}

    /// Responds with a string from the URL. Used to figure out how to reject
    /// bad URLs. This is relevant for this project because I wanted to reject
    /// non-hex colour codes.
    pub async fn three_string(string: String) -> Result<impl Reply, Rejection> {
        let length = string.len();
        if length == 3 {
            Ok(string)
        } else if length < 3 {
            Err(custom(ThreeStringFailure::TooShort))
        } else {
            Err(custom(ThreeStringFailure::TooLong))
        }
    }
}

/// A constant containing the bytes for preview.png. This is used as a template
/// for the preview images.
const IMAGE: &'static [u8] = include_bytes!("./preview.png");

/// Generates a PNG based on preview.png given a hex_colour. Normalises the
/// hex code in the URL and rejects invalid hex codes.
///
/// In the PNG, there's a PLTE chunk containing the palette for the image.
/// Since the image only has one colour, its palette only contains one colour.
/// Thus, I can just change the colour in that palette and update the CRC at
/// the end of the chunk.
///
/// The CRC ("cyclic redundancy code") is like a hash of the rest of the chunk,
/// and I'm assuming the PNG format uses it to ensure that the PNG was kept
/// successfully intact when sent over a network, for example.
pub async fn gen_image(hex_colour: String) -> Result<impl Reply, Rejection> {
    // Parse `hex_colour`
    let colour = match hex_colour.parse::<HexColour>() {
        Ok(colour) => colour,
        Err(HexColourParseError::Redirect(normalised)) => {
            // Normalise hex code by redirecting to the normalised (6-digit
            // lowercase) form
            return Ok(Response::builder()
                .status(302)
                .header("Location", format!("/colour/{}/preview", normalised))
                .body(vec![]));
        }
        Err(HexColourParseError::Invalid) => {
            // Reject strings that are not hex codes
            return Err(custom(InvalidHexColour));
        }
    };

    // Clone `IMAGE` (the preview.png data) to a mutable Vec<u8>
    let mut bytes = IMAGE.to_vec();

    // Replace the colour in the palette (PLTE chunk) with the given colour
    bytes[0x4b..0x4e].copy_from_slice(&colour.channels());

    // Update the CRC at the end of the PLTE chunk
    let mut hasher = Hasher::new();
    hasher.update(&bytes[0x47..0x4e]);

    let crc = hasher.finalize().to_be_bytes();
    bytes[0x4e..0x52].copy_from_slice(&crc);

    Ok(Response::builder()
        .header("Content-Type", "image/png")
        .body(bytes))
}

/// A string containing the template HTML for the colour preview page
const PAGE: &'static str = include_str!("./preview.html");

/// Generates an HTML page for previewing a given colour
pub async fn colour_preview(hex_colour: String) -> Result<impl Reply, Rejection> {
    // Normalise hex colour codes and reject invalid strings, as is done in
    // `gen_image`.
    // We don't need the parsed colour; it just needs to be valid.
    if let Err(error) = hex_colour.parse::<HexColour>() {
        return match error {
            HexColourParseError::Redirect(normalised) => Ok(Response::builder()
                .status(302)
                .header("Location", format!("/colour/{}/", normalised))
                .body(String::new())),
            HexColourParseError::Invalid => Err(custom(InvalidHexColour)),
        };
    }

    Ok(Response::builder()
        .header("Content-Type", "text/html; charset=utf-8")
        // Replace all `HEX` in the template HTML with the colour code.
        .body(PAGE.replace("HEX", hex_colour.as_str())))
}

/// Generate a random hex code and redirect to its colour preview page
pub async fn random_colour() -> Result<impl Reply, Rejection> {
    let colour = random::<(u8, u8, u8)>();

    Ok(Response::builder()
        .status(302)
        .header(
            "Location",
            format!("/colour/{:02x}{:02x}{:02x}/", colour.0, colour.1, colour.2),
        )
        .body(vec![]))
}
