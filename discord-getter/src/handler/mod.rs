use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;

use termion::{color, style};
use std::io::{Write, stdout};
use std::{thread, time};

pub mod messages;
pub mod threads;
pub mod ready;

use crate::handler::{
    messages::{handle_chat, handle_archived_channel_threads},
    threads::{handle_archived_forum_threads, handle_active_forum_threads},
    ready::handle_ready,
};

pub struct Handler;

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

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, _msg: Message) {
        print_rainbow_text("Working... Don't forget to look chat output folders and threads folders :))");
        handle_chat(&ctx).await;
        handle_archived_channel_threads(&ctx).await;
        handle_archived_forum_threads(&ctx).await;
        handle_active_forum_threads(&ctx).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        print_rainbow_text("Ready now!");
        handle_ready(ctx, ready).await;
    }

}