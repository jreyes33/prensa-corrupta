use rand::{thread_rng, sample};

use twitter::last_tweet;
use oauth::Credentials;

pub fn new_tweet(credentials: &Credentials) -> String {
    let media_outlets = ["el_telegrafo", "eluniversocom", "elcomerciocom", "andesecuador",
    "ecuavisa", "ecuadortv", "ElCiudadano_ec", "mercurioec", "lahoraecuador", "eldiarioec"];
    let mut rng = thread_rng();
    let chosen_outlets = sample(&mut rng, media_outlets.iter(), 2);
    println!("chosen_outlets: {:?}", chosen_outlets);
    let generated_tweet = format!("{} {}",
                                  halves(last_tweet(credentials, chosen_outlets[0])).0,
                                  halves(last_tweet(credentials, chosen_outlets[1])).1);
    println!("{}", generated_tweet);
    generated_tweet
}

fn halves(text: String) -> (String, String) {
    let words: Vec<&str> = text.split_whitespace().collect();
    let count = words.len();
    let mut iter = words.chunks(count / 2);
    (iter.next().unwrap().join(" "), iter.next().unwrap().join(" "))
}
