pub mod endpoints_mod {
    use warp::{Filter};
    use serde_json::{Value};

    pub fn endpoints_endpoint() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
        let endpoints_vec = vec!(
            ["/", "Get a random red panda from the cdn.", "GET"],
            ["/boop", "Get a random red panda boop!", "GET"],
            ["/tired", "Get a yawning/sleeping red panda.", "GET"],
            ["/fact", "Get a random fact about redpandas.", "GET"],
            ["/funny", "Get a random red panda doing something funny.", "GET"],
            ["/tall", "Get a random image of a red panda standing up.", "GET"],
            ["/random", "Get a random red panda from the cdn but it comes as a static image.", "GET"],
            ["/endpoints", "Get a list of all endpoints with descriptions and methods.", "GET"]
        );
        let mut endpoints_data = "".to_owned();
        for endpoint in endpoints_vec {
            endpoints_data.push_str(&*format!(r#"{{ "endpoint": "{}", "description": "{}", "methods": "{}" }},"#, endpoint[0], endpoint[1], endpoint[2]));
        }

        &endpoints_data.truncate(&endpoints_data.len() - 1);

        let endpoints: Value = serde_json::from_str(&format!("[{}]", &endpoints_data)).unwrap();
        warp::path("endpoints")
            .and(warp::get())
            .map(move || {
                warp::reply::json(&endpoints)
            })
    }
}