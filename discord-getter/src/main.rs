use std::env;
use dotenv::dotenv;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};
use serenity::model::id::{ChannelId, MessageId};

use csv::Writer;
use std::error::Error;


#[group]
#[commands(ping)]
struct General;

struct Handler;


#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.

    async fn message(&self, ctx: Context, _msg: Message) {
        let channel_id = ChannelId(1153348653122076676);
        let _messages = channel_id
            .messages(&ctx, |retriever| retriever.after(MessageId(1153348746881540206)).limit(100))
            .await;
    
        let mut writer = Writer::from_path("output.csv").unwrap();
    
        if let Ok(messages) = _messages {
            for message in messages {
                let data = (
                    message.id,
                    message.channel_id,
                    message.author.name.clone(),
                    message.content.clone(),
                    message.timestamp,
                    message
                        .mentions
                        .iter()
                        .map(|user| format!("{}: {}", user.id, user.name))
                        .collect::<Vec<_>>()
                        .join(","),
                    message
                        .reactions
                        .iter()
                        .map(|reaction| format!("{}, {}", reaction.count, reaction.reaction_type))
                        .collect::<Vec<_>>()
                        .join(","),
                    message
                        .referenced_message
                        .as_ref()
                        .map(|referenced_message| format!("{}: {}", referenced_message.id, referenced_message.content)),
                    message.member.as_ref().map(|memb| {
                        (
                            memb.nick.as_ref().map_or("NONE", |n| &n),
                            memb.roles.iter().map(|x| x.to_string() + ",").collect::<String>(),
                        )
                    }),
                );
    
                writer.write_record(&[
                    data.0.to_string(),
                    data.1.to_string(),
                    data.2.to_string(),
                    data.3.to_string(),
                    data.4.to_string(),
                    data.5,
                    data.6,
                    data.7.unwrap_or_default(), // referenced_message
                    data.8.map_or("NONE,NONE".to_string(), |(nick, roles)| format!("{}: {}", nick, roles)),
                ]);
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    dotenv().ok();

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).framework(framework).await.expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}