use thiserror::Error;

#[derive(Debug, Error)]
enum RouteError {
    // #[error("...")] is an attribute macro provided by thiserror.
    #[error("invalid location")]
    InvalidLocation,

    #[error("road not found")]
    RoadNotFound,

    #[error("no route exists between these locations")]
    NoRoute,
}

fn main() {
    let result: Result<String, RouteError> = Err(RouteError::NoRoute);

    println!("{:?}", result.unwrap_err());
}
