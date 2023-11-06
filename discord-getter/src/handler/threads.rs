use std::env;
use dotenv::dotenv;
use csv::Writer;

use serenity::prelude::*;
use serenity::model::id::{ChannelId, MessageId};
use serenity::model::prelude::GuildId;
use serenity::http::client::Http;

use crate::constants::*;

/*
  ____ _____ _____   _____ _   _ ____  _____    _    ____  ____  
 / ___| ____|_   _| |_   _| | | |  _ \| ____|  / \  |  _ \/ ___| 
| |  _|  _|   | |     | | | |_| | |_) |  _|   / _ \ | | | \___ \ 
| |_| | |___  | |     | | |  _  |  _ <| |___ / ___ \| |_| |___) |
 \____|_____| |_|     |_| |_| |_|_| \_\_____/_/   \_\____/|____/ 
*/

pub async fn handle_archived_forum_threads(ctx: &Context) {
    // TODO: Get channel and guild Ids automatically
    let forum_channel_id = ChannelId(FORUM_CHANNEL_ID);
    let guild_id = GuildId(GUILD_ID);


    // ARCHIVED THREADS
    let _archived_threads = forum_channel_id.get_archived_public_threads(&ctx, None, None);
}

pub async fn handle_active_forum_threads(ctx: &Context) {
    // Configure the client with your Discord bot token in the environment.
    dotenv().ok();

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // ACTIVE THREAD 
    let forum_channel_id = ChannelId(FORUM_CHANNEL_ID);
    let guild_id = GuildId(GUILD_ID);

    let http = Http::new(&token);
    let mut active_threads = http.get_guild_active_threads(GUILD_ID).await.unwrap();
    println!("New active threads: {:?}", active_threads.threads.len());
    let mut archived_public_threads = http.get_channel_archived_public_threads(FORUM_CHANNEL_ID, None, None).await.unwrap();
    println!("Archived threads: {:?}", archived_public_threads.threads.len());

}