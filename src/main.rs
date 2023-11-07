use tokio;
use clokwerk::AsyncScheduler;
use cohost::log_in;
use files::{get_config, get_words};

mod files;
mod cohost;

#[tokio::main]
async fn main() {
    let user = get_config("config.json");
    let session = log_in(&user.email, &user.password).await;
    let words = get_words("nouns.json", "adjectives.json");

    let scheduler = AsyncScheduler::new();
}
