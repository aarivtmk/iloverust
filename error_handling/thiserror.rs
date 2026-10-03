use thiserror::Error;

#[derive(Debug, Error)]
enum RouteError {
    #[error("invalid location")]
    InvalidLocation,

    #[error("road not found")]
    RoadNotFound,

    #[error("no route exists between these locations")]
    NoRoute,
}
