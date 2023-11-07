use eggbug::{Session, Post};
use std::thread;
use rand::Rng;
use crate::files::Words;

pub async fn log_in(email: &String, pswd: &String) -> Session {
    println!("Logging in now.");
    let session = loop {
        match Session::login(email, pswd).await {
            Ok(ses) => {
                println!("Login successful!");
                break ses;
            },
            Err(error) => {
                println!("Couldn't log in!");
                println!("{}", error);
                println!("Trying again in a minute!");
                thread::sleep(std::time::Duration::from_secs(60))
            }
        }
    };
    session
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