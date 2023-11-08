use tokio;
use clokwerk::{AsyncScheduler, TimeUnits};
use cohost::{log_in, new_post};
use files::{get_config, get_words};

mod files;
mod cohost;

#[tokio::main]
async fn main() {
    let interval_hours = 5;

    let user = get_config("config.json");
    let session = log_in(&user.email, &user.password).await;
    let words = get_words("nouns.json", "adjectives.json");

    let mut scheduler = AsyncScheduler::new();

    scheduler.every(interval_hours.seconds()).run(move || {
        println!("Running!");
        let mut post = new_post(&words);
        let ses = session.clone();
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
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
