mod handlers;
mod endpoints;

use crate::endpoints::{home_endpoints, endpoints_endpoint, boop_endpoint, tired_endpoint,
                       fact_endpoint, funny_endpoint, tall_endpoint, random_endpoint};
use crate::handlers::{handle_rejection};

use warp::Filter;

#[tokio::main]
async fn main() {
    let routes = home_endpoints()
        .or(endpoints_endpoint())
        .or(boop_endpoint())
        .or(tired_endpoint())
        .or(fact_endpoint())
        .or(funny_endpoint())
        .or(tall_endpoint())
        .or(random_endpoint())
        .recover(handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 2003)).await;
}