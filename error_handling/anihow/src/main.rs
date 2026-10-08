// thiserror
#[derive(Debug)]
enum RouteError {
    InvalidLocation,
    RoadNotFound,
    NoRoute,
}
fn find_route(from: u32, to: u32) -> Result<String, RouteError> {
    if from == 0 {
        return Err(RouteError::InvalidLocation);
    }

    if from == 999 {
        return Err(RouteError::RoadNotFound);
    }

    Ok("Shimla → Kufri".to_string())
}
fn main() {
    //     match find_route(0, 200) {
    //         Ok(route) => println!("Route: {route}"),
    //
    //         Err(RouteError::InvalidLocation) => {
    //             println!("Invalid location!");
    //         }
    //
    //         Err(RouteError::RoadNotFound) => {
    //             println!("Road not found!");
    //         }
    //
    //         Err(RouteError::NoRoute) => {
    //             println!("No route exists!");
    //         }
    //     }
    //
    let result = find_route(0, 200);
    println!("herohero");
    println!("result is {:?}", result);
    match result {
        Ok(route) => println!("Route: {:?}", route),

        Err(error) => {
            println!("Route failed: {:?}", error);
        }
    }
}

#[derive(Debug, Error)]
enum RouteError {
    #[error("invalid location")]
    InvalidLocation,

    #[error("road not found")]
    RoadNotFound,

    #[error("no route exists")]
    NoRoute,
}
