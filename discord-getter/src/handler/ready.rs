use serenity::prelude::*;
use serenity::model::gateway::Ready;

use termion::{color, style};
use std::io::{Write, stdout};
use std::{thread, time};

use tokio::task;


pub fn print_rainbow_text(text: &str) {
    let colors = [
        (255, 0, 0),    // Red
        (255, 165, 0),  // Orange
        (255, 255, 0),  // Yellow
        (0, 255, 0),    // Green
        (0, 0, 255),    // Blue
        (128, 0, 128),  // Purple
    ];

    for (i, c) in text.chars().enumerate() {
        let (r, g, b) = colors[i % colors.len()];
        print!(
            "{}{}{}",
            color::Fg(color::Rgb(r, g, b)),
            c,
            style::Reset
        );
        stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(100)); // Add a delay for effect
    }

    println!(); // Print a newline at the end
}

// Set a handler to be called on the `ready` event. This is called when a
// shard is booted, and a READY payload is sent by Discord. This payload
// contains data like the current user's guild Ids, current user data,
// private channels, and more.
//
// In this case, just print what the current user's username is.
pub async fn handle_ready(_: Context, ready: Ready) {
    let rainbow_task = task::spawn_blocking(move || {
        print_rainbow_text(&format!("{} is connected!", ready.user.name));
    });

    if let Err(e) = rainbow_task.await {
        eprintln!("Error in rainbow_task: {:?}", e);
    }
}