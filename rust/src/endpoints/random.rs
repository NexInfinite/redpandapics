pub mod random_mod {
    use warp::{Filter};
    use rand::seq::SliceRandom;
    use std::fs;
    use serde_json::{Value};

    pub fn random_endpoint() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
        let mut file_list = Vec::new();

        let paths = fs::read_dir("./cdn_all/").unwrap();
        for path in paths {
            let split = path.unwrap().path().display().to_string().to_owned().split("/").last().unwrap().to_owned();
            &file_list.push(split);
        }

        warp::path("random").and(warp::get())
            .map(move || {
                let random_image = file_list.to_owned().choose(&mut rand::thread_rng()).unwrap().to_owned();
                let chosen_image: Value = serde_json::from_str(&format!(r#"{{ "url": "https://cdn.redpanda.pics/{}"  }}"#, random_image)).unwrap();
                warp::reply::json(&chosen_image)
            })
    }
}