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

    use std::fmt;
    impl fmt::Display for ThreeStringFailure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ThreeStringFailure::TooLong => write!(f, "too long."),
                ThreeStringFailure::TooShort => write!(f, "too short."),
            }
        }
    }

    #[derive(Debug)]
    pub struct StrTooLong;
    impl Reject for StrTooLong {}

    pub async fn three_string(string: String) -> Result<impl Reply, Rejection> {
        let length = string.len();
        if length == 3 {
            Ok(string)
        } else if length < 3 {
            Err(custom(ThreeStringFailure::TooShort))
        } else {
            Err(custom(ThreeStringFailure::TooLong))
        } /*{
              Err(StrTooLong)
          }*/
    }
}
