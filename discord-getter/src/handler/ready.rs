use serenity::prelude::*;
use serenity::model::gateway::Ready;

// Set a handler to be called on the `ready` event. This is called when a
// shard is booted, and a READY payload is sent by Discord. This payload
// contains data like the current user's guild Ids, current user data,
// private channels, and more.
//
// In this case, just print what the current user's username is.
pub async fn handle_ready(_: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
}