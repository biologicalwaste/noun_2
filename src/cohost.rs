use crate::files::Words;
use eggbug::{Post, Session, Error};
use rand::Rng;

pub async fn log_in(email: &String, pswd: &String) -> Result<Session, Error> {
    let ses = Session::login(email, pswd).await;
    ses
}

pub fn new_post(words: &Words) -> Post {
    let mut rng = rand::thread_rng();

    let noun_choice = rng.gen_range(0..words.nouns.len());
    let adjective_choice = rng.gen_range(0..words.adjectives.len());

    let mut statement = String::from("When the ");
    statement.push_str(&words.nouns[noun_choice]);
    statement.push_str(" is ");
    statement.push_str(&words.adjectives[adjective_choice]);
    statement.push('!');
    let post = Post {
        markdown: statement,
        tags: vec!["eggbug-rs".to_string(), "bots of cohost".to_string()],
        ..Default::default()
    };

    post
}
