use super::handlers;
use warp::{get, http::Response, path, Filter, Rejection, Reply};

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    fooling_around::return_int()
        .or(fooling_around::three_string())
        .or(gen_image())
        .or(colour_preview())
        .or(static_files())
}

mod fooling_around {
    use super::handlers;
    use warp::{get, path, Filter, Rejection, Reply};

    pub fn return_int() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        path!("hi" / u32)
            .and(get())
            .and_then(handlers::fooling_around::return_int)
    }

    pub fn three_string() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        path!("woink" / String)
            .and(get())
            .and_then(handlers::fooling_around::three_string)
    }
}

fn gen_image() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("colour" / String / "preview")
        .and(get())
        .and_then(handlers::gen_image)
}

fn colour_preview() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("colour" / String)
        .and(get())
        .and_then(handlers::colour_preview)
}

fn static_files() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("styles").and(get()).map(|| {
        Response::builder()
            .header("Content-Type", "text/css; charset=utf-8")
            .body(include_str!("./styles.css"))
    })
}
