use super::hex_colour::{HexColour, HexColourParseError, InvalidHexColour};
use crc32fast::Hasher;
use rand::random;
use warp::{
    http::Response,
    reject::{custom, Rejection},
    Reply,
};

pub mod fooling_around {
    use std::convert::Infallible;
    use warp::{
        http::Response,
        reject::{custom, Reject, Rejection},
        Reply,
    };

    pub async fn return_int(int: u32) -> Result<impl Reply, Infallible> {
        Ok(Response::builder()
            .header("my-custom-header", "some-value")
            .body(format!("here is cool int: {}", int)))
    }

    #[derive(Debug)]
    pub enum ThreeStringFailure {
        TooLong,
        TooShort,
    }

    impl Reject for ThreeStringFailure {}

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

const IMAGE: &'static [u8] = include_bytes!("./preview.png");

pub async fn gen_image(hex_colour: String) -> Result<impl Reply, Rejection> {
    let colour = match hex_colour.parse::<HexColour>() {
        Ok(colour) => colour,
        Err(HexColourParseError::Redirect(normalised)) => {
            return Ok(Response::builder()
                .status(302)
                .header("Location", format!("/colour/{}/preview", normalised))
                .body(vec![]));
        }
        Err(HexColourParseError::Invalid) => {
            return Err(custom(InvalidHexColour));
        }
    };

    let mut bytes = IMAGE.to_vec();

    bytes[0x4b..0x4e].copy_from_slice(&colour.channels());

    let mut hasher = Hasher::new();
    hasher.update(&bytes[0x47..0x4e]);

    let crc = hasher.finalize().to_be_bytes();
    bytes[0x4e..0x52].copy_from_slice(&crc);

    Ok(Response::builder()
        .header("Content-Type", "image/png")
        .body(bytes))
}

const PAGE: &'static str = include_str!("./preview.html");

pub async fn colour_preview(hex_colour: String) -> Result<impl Reply, Rejection> {
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
        .body(PAGE.replace("HEX", hex_colour.as_str())))
}

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
