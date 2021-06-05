//! Starts the HTTP server based on the command line arguments.

mod filters;
mod handlers;
mod hex_colour;

use clap::{App, Arg};

/// Main
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
