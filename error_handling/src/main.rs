use thiserror::Error;

// #[derive(...)]
// derive macro
// #[error(...)]
// attribute macro
#[derive(Debug, Error)]
enum RouteError {
    #[error("invalid location")]
    InvalidLocation,

    #[error("road not found")]
    RoadNotFound,

    #[error("no route exists between these locations")]
    NoRoute,
}

fn main() {
    let result: Result<String, RouteError> = Err(RouteError::NoRoute);

    println!("{}", result.unwrap_err());
}

/*
* impl std::error::Error for RouteError {}

impl std::fmt::Display for RouteError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RouteError::InvalidLocation => {
                write!(f, "invalid location")
            }
        }
    }
}
*
*
 */
