use serenity::prelude::*;
use serenity::model::gateway::Ready;

pub async fn handle_ready(_: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
}