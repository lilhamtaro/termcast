use std::env;
use dotenv::dotenv;
use inquire::{Confirm, Text};
use teloxide::{prelude::*, types::InputFile, Bot};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let bot = Bot::from_env();

    let default_channel = env::var("CHANNEL").expect("CHANNEL not set in .env");

    // Ask for file path
    let content = Text::new("Enter the file path to send:")
    .prompt()
    .unwrap();

    // Confirm Send
    println!("Current Target: {}", default_channel);
    let proceed = Confirm::new("Send to this channel?")
    .with_default(true)
    .prompt()
    .unwrap();

    // Option: allow to change target
    let target = if proceed {
        default_channel
    } else {
        Text::new("Enter the group or channel ID:")
        .prompt()
        .unwrap()
    };

    let comment = {
        Text::new("Enter a caption: ")
        .prompt()
        .unwrap()
    };

    // Determine if it's a file path or message
    if std::path::Path::new(&content).exists() {
        let file = InputFile::file(content.clone());
        if let Err(err) = bot.send_document(target.clone(), file).caption(comment).send().await {
            eprintln!("Failed to send document to {}: {:?}", target, err);
            return;
        }
    } else {
        bot.send_message(target, content).send().await.unwrap();
    }

    println!("Message sent successfully!");
}