//! Warp filters define URL routes and combine them with handlers.
//!
//! You can think of this as like a really long chain of if-else statements;
//! "if the URL has 'hi' then a number, then respond with this." It might seem
//! inefficient, but for any router it probably does something similar.

use super::handlers;
use warp::{get, http::Response, path, Filter, Rejection, Reply};

/// Combines all the routes and their handlers into a single route for Warp.
pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    fooling_around::return_int()
        .or(fooling_around::three_string())
        .or(gen_image())
        .or(colour_preview())
        .or(random_colour())
        .or(static_files())
}

/// Routes for my experimentations for learning about Warp
mod fooling_around {
    use super::handlers;
    use warp::{get, path, Filter, Rejection, Reply};

    /// Defines a route at `/hi/<int>`. See
    /// [handlers::fooling_around::return_int].
    pub fn return_int() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        path!("hi" / u32)
            .and(get())
            .and_then(handlers::fooling_around::return_int)
    }

    /// Defines a route at `/woink/<string>`. See
    /// [handlers::fooling_around::three_string].
    pub fn three_string() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        path!("woink" / String)
            .and(get())
            .and_then(handlers::fooling_around::three_string)
    }
}

/// Defines a route at `/colour/<hex colour>/preview`. See [handlers::gen_image].
fn gen_image() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("colour" / String / "preview")
        .and(get())
        .and_then(handlers::gen_image)
}

/// Defines a route at `/colour/<hex colour>`. See [handlers::colour_preview].
fn colour_preview() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("colour" / String)
        .and(get())
        .and_then(handlers::colour_preview)
}

/// Defines a route at `/` (the root). See [handlers::random_colour].
fn random_colour() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path::end().and(get()).and_then(handlers::random_colour)
}

/// Defines a route at `/styles` and responds with the contents from
/// styles.css. Maybe in the future, this is also where I define other static
/// files, though it's probably better to make a static/ folder and serve
/// everything from that directory.
fn static_files() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("styles").and(get()).map(|| {
        Response::builder()
            .header("Content-Type", "text/css; charset=utf-8")
            .body(include_str!("./styles.css"))
    })
}
