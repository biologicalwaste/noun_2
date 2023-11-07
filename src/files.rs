use std::fs::File;
use serde_json::{self, Value};

pub struct User {
    pub email: String,
    pub password: String
}

#[derive(Clone)]
pub struct Words {
    pub nouns: Vec<String>,
    pub adjectives: Vec<String>
}

pub fn get_config(path: &str) -> User {
    let file = match File::open(path){
        Ok(f) => f,
        Err(e) => {
            println!("Couldn't find config file!!!!!");
            println!("{}", e);
            std::process::exit(1)
        }
    };
    let config: Value = match serde_json::from_reader(file) {
        Ok(user_data) => user_data,
        Err(e) => {
            println!("Couldn't parse config file!!!!!!");
            println!("{}", e);
            std::process::exit(1)
        }
    };

    let user = User {
        email: config[0].as_str().unwrap().to_string(),
        password: config[1].as_str().unwrap().to_string()
    };

    user
}

pub fn get_words(nouns_path: &str, adjectives_path: &str) -> Words {
    let nouns_file = match File::open(nouns_path){
        Ok(f) => f,
        Err(e) => {
            println!("Couldn't find nouns file!!!!!");
            println!("{}", e);
            std::process::exit(1)
        }
    };
    let adjectives_file = match File::open(adjectives_path){
        Ok(f) => f,
        Err(e) => {
            println!("Couldn't find adjectives file!!!!!");
            println!("{}", e);
            std::process::exit(1)
        }
    };

    let words = Words {
        nouns: match serde_json::from_reader(nouns_file) {
            Ok(data) => data,
            Err(e) => {
                println!("Could not parse nouns file!");
                println!("{}", e);
                std::process::exit(1)
            }
        },
        adjectives: match serde_json::from_reader(adjectives_file) {
            Ok(data) => data,
            Err(e) => {
                println!("Could not parse adjectives file!");
                println!("{}", e);
                std::process::exit(1)
            }
        }
    };

    words
}