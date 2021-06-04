use warp::Filter;

#[derive(Debug)]
struct Hex(u32);

impl std::str::FromStr for Hex {
    type Err = std::num::ParseIntError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        u32::from_str_radix(src, 16)
            .map(|parsed| Hex(parsed))
    }
}

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("colour" / Hex)
        .map(|name| format!("Hello, {:?}!", name));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
