pub mod funny_mod {
    use warp::{Filter};
    use rand::seq::SliceRandom;
    use std::fs;
    use std::io::Read;

    pub fn funny_endpoint() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
        let mut file_list = Vec::new();

        let paths = fs::read_dir("./funny/").unwrap();
        for path in paths {
            let mut f = fs::File::open(path.unwrap().path().display().to_string().to_owned()).unwrap();
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer).unwrap();
            &file_list.push(buffer);
        }

        warp::path("funny").and(warp::get())
            .map(move || warp::reply::with_header(file_list.to_owned().choose(&mut rand::thread_rng()).unwrap().to_owned(), "content-type", "image/gif"))
    }
}