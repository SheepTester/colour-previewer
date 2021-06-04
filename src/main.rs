use clap::{App, Arg};
mod filters;
mod handlers;
mod hex_colour;

#[derive(Debug)]
struct Hex(u32);

impl std::str::FromStr for Hex {
    type Err = std::num::ParseIntError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        u32::from_str_radix(src, 16).map(|parsed| Hex(parsed))
    }
}

#[tokio::main]
async fn main() {
    let matches = App::new("Colour previewer")
        .about("An HTTP server that previews colours")
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .help("Sets a custom port (default: 3030)")
                .takes_value(true),
        )
        .get_matches();

    let port = matches
        .value_of("port")
        .and_then(|port| port.parse::<u16>().ok())
        .unwrap_or(3030);

    let routes = filters::routes();

    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}
