use super::handlers;
use warp::Filter;

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    fooling_around::return_int().or(fooling_around::three_string())
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
