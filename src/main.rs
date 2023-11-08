use tokio;
use clokwerk::{AsyncScheduler, TimeUnits};
use cohost::{log_in, new_post};
use files::{get_config, get_words};

mod files;
mod cohost;

#[tokio::main]
async fn main() {
    let interval_hours = 3;

    // Get user data from config file, store it in a User struct
    let user = get_config("config.json");
    // Get a Session from cohost by calling log_in
    let session = log_in(&user.email, &user.password).await;
    // Read the nouns and adjectives files and get a Words struct from it.
    let words = get_words("nouns.json", "adjectives.json");

    let mut scheduler = AsyncScheduler::new();

    scheduler.every(interval_hours.hours()).run(move || {
        println!("Running!");
        let ses = session.clone();
        let mut post = new_post(&words);

        async move {
            println!("Generating post!");
            match ses.create_post("when-the", &mut post).await {
                Ok(id) => {
                    println!("Post successfully created!");
                    println!("Contents: {}, ID: {}", post.markdown, id);
                },
                Err(e) => {
                    println!("Unable to create post!");
                    println!("{}", e);
                }
            };
        }
    });

    loop {
        scheduler.run_pending().await;
        tokio::time::sleep(std::time::Duration::from_secs(30)).await;
    }
}
