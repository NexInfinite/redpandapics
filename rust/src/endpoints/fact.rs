pub mod fact_mod {
    use warp::{Filter};
    use rand::seq::SliceRandom;
    use serde_json::{Value};

    pub fn fact_endpoint() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
        let facts = vec!(
            ["Red pandas live in the Eastern Himalayas and South-western China.", "https://www.folly-farm.co.uk/zoo/meet-the-zoo-animals/red-panda/"],
            ["Red pandas are roughly the size of a large domestic cat.", "https://www.folly-farm.co.uk/zoo/meet-the-zoo-animals/red-panda/"],
            ["The red panda diet is 97% bamboo and they occasionally eat small mammals, eggs and flowers.", "https://www.folly-farm.co.uk/zoo/meet-the-zoo-animals/red-panda/"],
            ["Red pandas are mainly active at night and they tend to look for food at dawn and at dusk.", "https://www.folly-farm.co.uk/zoo/meet-the-zoo-animals/red-panda/"],
            ["Red pandas can live up to 15 years.", "https://www.folly-farm.co.uk/zoo/meet-the-zoo-animals/red-panda/"],
            ["Red pandas are endangered mainly because of their natural habitat being destroyed and also due to them being hunted.", "https://www.folly-farm.co.uk/zoo/meet-the-zoo-animals/red-panda/"],
            ["The scientific name for a red panda is 'Ailurus fulgens'", "https://www.nationalgeographic.com/animals/mammals/facts/red-panda"],
            ["Currently, red pandas live in the Eastern Himalayas. But the first red panda fossil was found a little bit further afield than that—in the United Kingdom.", "https://www.mentalfloss.com/article/63578/12-furry-facts-about-red-pandas"],
            ["Red pandas have false thumbs.", "https://www.mentalfloss.com/article/63578/12-furry-facts-about-red-pandas"]
        );

        warp::path("fact").and(warp::get())
            .map(move || {
                let random_fact = facts.to_owned().choose(&mut rand::thread_rng()).unwrap().to_owned();
                let chosen_fact: Value = serde_json::from_str(&format!(r#"{{ "fact": "{}", "source": "{}"  }}"#, random_fact[0], random_fact[1])).unwrap();
                warp::reply::json(&chosen_fact)
            })
    }
}