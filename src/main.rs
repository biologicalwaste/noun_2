use clokwerk::{AsyncScheduler, TimeUnits};
use cohost::{log_in, new_post};
use files::{get_config, get_words};
use miraculous_term::UI;

mod cohost;
mod files;

#[tokio::main]
async fn main() {
    let mut ui = UI::new();
    let mut scheduler = AsyncScheduler::new();
    let (tx, rx) = std::sync::mpsc::channel();
    let itx = tx.clone();

    let interval_hours = 3;

    let words = get_words("nouns.json", "adjectives.json");
    let user = get_config("config.json");
    ui.set_info("Press 'q' to quit!".to_string());
    ui.push_draw("Logging in now!".to_string());
    let session = loop {
        match log_in(&user.email, &user.password).await {
            Ok(session) => {
                ui.push_draw("Login successful!".to_string());
                break session;
            }
            Err(e) => {
                ui.push_draw("Login failed! Trying again in a minute!".to_string());
                ui.push_draw(e.to_string());
            }
        }
    };

    scheduler.every(interval_hours.hours()).run(move || {
        let stx = tx.clone();
        let mut post = new_post(&words);
        let ses = session.clone();
        stx.send("Running now!".to_string()).unwrap();
        async move {
            match ses.create_post("when-the", &mut post).await {
                Ok(id) => {
                    stx.send("Post created!".to_string()).unwrap();
                    stx.send(post.markdown).unwrap();
                    stx.send(id.to_string()).unwrap();
                }
                Err(e) => {
                    stx.send("Failed to create post!".to_string()).unwrap();
                    stx.send(e.to_string()).unwrap();
                }
            };
        }
    });

    let _run_ui = std::thread::spawn(move || {
        let mut nui = ui.clone();
        loop {
            let to_draw: String = rx.recv().unwrap().to_string();
            nui.push_draw(to_draw);
        }
    });

    let run_scheduler = tokio::task::spawn(async move {
        loop {
            scheduler.run_pending().await;
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    });

    let _input = std::thread::spawn(move || loop {
        match UI::key('q') {
            Ok(k) => match k {
                true => {
                    let _ = UI::exit();
                    std::process::exit(0);
                }
                false => continue,
            },
            Err(_) => itx.send("Failed to read input!".to_string()).unwrap(),
        }
    });

    run_scheduler.await;
}
