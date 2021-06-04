use crc32fast::Hasher;
use warp::{http::Response, reject::Rejection, Reply};

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

const IMAGE: &'static [u8] = include_bytes!("preview.png");

pub async fn gen_image(hex_colour: String) -> Result<impl Reply, Rejection> {
    let mut bytes = IMAGE.to_vec();

    bytes[0x4b] = 0x12;
    bytes[0x4c] = 0x34;
    bytes[0x4d] = 0x56;

    let mut hasher = Hasher::new();
    hasher.update(&bytes[0x47..0x4e]);

    let crc = hasher.finalize().to_be_bytes();
    bytes[0x4e..0x52].copy_from_slice(&crc);

    Ok(Response::builder()
        .header("Content-Type", "image/png")
        .body(bytes))
}