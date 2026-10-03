#[derive(Debug)]
enum RouteError {
    InvalidLocation,
    RoadNotFound,
    NoRoute,
}
fn find_route(from: u32, to: u32) -> Result<String, RouteError> {
    if from == 0 || to == 0 {
        return Err(RouteError::InvalidLocation);
    }

    if from == 999 {
        return Err(RouteError::RoadNotFound);
    }

    if from == 100 && to == 200 {
        return Err(RouteError::NoRoute);
    }

    Ok("Shimla → Kufri".to_string())
}

fn main() {
    match find_route(20, 20) {
        Ok(route) => println!("Route: {route}"),

        Err(RouteError::InvalidLocation) => {
            println!("Invalid location");
        }

        Err(RouteError::RoadNotFound) => {
            println!("Could not find road");
        }

        Err(RouteError::NoRoute) => {
            println!("No route exists");
        }
    }
}
