use eggbug::Session;
use std::thread;

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