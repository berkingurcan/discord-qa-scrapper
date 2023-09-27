use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;

pub mod messages;
pub mod threads;
pub mod ready;

use crate::handler::{
    messages::{handle_chat, handle_archived_channel_threads},
    threads::{handle_archived_forum_threads, handle_active_forum_threads},
    ready::handle_ready,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.

    async fn message(&self, ctx: Context, _msg: Message) {
        handle_chat(&ctx).await;
        handle_archived_channel_threads(&ctx).await;
        handle_archived_forum_threads(&ctx).await;
        handle_active_forum_threads(&ctx).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        handle_ready(ctx, ready).await;
    }

}