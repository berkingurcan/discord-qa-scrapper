use serenity::prelude::*;
use serenity::model::id::{ChannelId, MessageId};
use serenity::model::prelude::GuildId;
use csv::Writer;

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

    match (_archived_threads).await {
        Ok(data) => {
            let requested_channel_id = Some(ChannelId(FORUM_CHANNEL_ID));
            println!("Archived threads: {:?}", data.threads.len());
            let archived_thread_ids: Vec<ChannelId> = data.threads.iter()
                .filter(|channel| channel.parent_id == requested_channel_id)
                .map(|channel| channel.id)
                .collect();

            for thread_id in archived_thread_ids {
                let _thread_messages = thread_id.messages(&ctx, |retriever| retriever.after(MessageId(AFTER_MESSAGE_ID)).limit(LIMIT)).await;
                let extracted_data: Result<Vec<_>, _> = _thread_messages.map(|messages| {
                    messages.iter().map(|message| {
                        (
                            message.id,
                            message.channel_id,
                            message.author.name.clone(),
                            message.content.clone(),
                            message.timestamp,
                            message.mentions.iter().map(|user| (user.id, user.name.clone())).collect::<Vec<_>>(),
                            message.reactions.iter().map(|reaction| (reaction.count, reaction.reaction_type.clone())).collect::<Vec<_>>(),
                            message.referenced_message.as_ref().map(|referenced_message| {
                                (referenced_message.id, referenced_message.content.clone())
                            }),
                            message.member.as_ref().map(|memb| {
                                (memb.nick.as_ref().map(String::to_string), memb.roles.iter().map(|x| x.to_string() + ",").collect::<String>())
                            }),
                        )
                    }).collect()
                });

                let unwrapped_extracted_data = extracted_data.unwrap();
                let mut writer = Writer::from_path(format!("./outputs/archived_threads/{}.csv", thread_id.to_string())).unwrap();

                for data in unwrapped_extracted_data {
                    writer.write_record(&[
                        data.0.to_string(),
                        data.1.to_string(),
                        data.2.to_string(),
                        data.3.to_string(),
                        data.4.to_string(),
                        data.5.iter()
                            .map(|(id, name)| format!("{}: {}", id, name))
                            .collect::<Vec<_>>()
                            .join(",")
                            .to_string(),
                        data.6.iter()
                            .map(|(count, reaction_type)| format!("{}, {}", count, reaction_type))
                            .collect::<Vec<_>>()
                            .join(",")
                            .to_string(),
                        data.7.iter()
                            .map(|(id, content)| format!("{}: {}", id, content))
                            .collect::<Vec<_>>()
                            .join(",")
                            .to_string(),
                        data.8.iter()
                            .map(|(nick, roles)| format!("{}: {}", nick.as_ref().map_or("NONE", |n| n), roles))
                            .collect::<Vec<_>>()
                            .join(",")
                            .to_string(),
                    ]);
                }
            }
        },
        Err(error) => println!("{:?}", error),
    }
}

pub async fn handle_active_forum_threads(ctx: &Context) {
    // ACTIVE THREAD 
    let forum_channel_id = ChannelId(FORUM_CHANNEL_ID);
    let guild_id = GuildId(GUILD_ID);

    let _active_threads = guild_id.get_active_threads(&ctx);

    match (_active_threads).await {
        Ok(data) => {
            let requested_channel_id = Some(ChannelId(FORUM_CHANNEL_ID));
            println!("Active threads: {:?}", data.threads.len());

            let active_thread_ids: Vec<ChannelId> = data.threads.iter()
                .filter(|channel| channel.parent_id == requested_channel_id)
                .map(|channel| channel.id)
                .collect();

            for thread_id in active_thread_ids {     
                let mut last_message_id = None;
                loop {
                    let _thread_messages = thread_id.messages(&ctx, |retriever| retriever.after(last_message_id.unwrap_or(MessageId(AFTER_MESSAGE_ID))).limit(LIMIT)).await;
                    match _thread_messages {
                        Ok(ref messages) => {
                            if messages.is_empty() {
                                break;
                            }
                            last_message_id = Some(messages[0].id);

                            let extracted_data: Result<Vec<_>, _> = _thread_messages.map(|messages| {
                                messages.iter().map(|message| {
                                    (
                                        message.id,
                                        message.channel_id,
                                        message.author.name.clone(),
                                        message.content.clone(),
                                        message.timestamp,
                                        message.mentions.iter().map(|user| (user.id, user.name.clone())).collect::<Vec<_>>(),
                                        message.reactions.iter().map(|reaction| (reaction.count, reaction.reaction_type.clone())).collect::<Vec<_>>(),
                                        message.referenced_message.as_ref().map(|referenced_message| {
                                            (referenced_message.id, referenced_message.content.clone())
                                        }),
                                        message.member.as_ref().map(|memb| {
                                            (memb.nick.as_ref().map(String::to_string), memb.roles.iter().map(|x| x.to_string() + ",").collect::<String>())
                                        }),
                                    )
                                }).collect()
                            });
            
                            let unwrapped_extracted_data = extracted_data.unwrap();
                            let mut writer = Writer::from_path(format!("./outputs/active_threads/{}.csv", thread_id.to_string())).unwrap();
            
                            for data in unwrapped_extracted_data {
                                writer.write_record(&[
                                    data.0.to_string(),
                                    data.1.to_string(),
                                    data.2.to_string(),
                                    data.3.to_string(),
                                    data.4.to_string(),
                                    data.5.iter()
                                        .map(|(id, name)| format!("{}: {}", id, name))
                                        .collect::<Vec<_>>()
                                        .join(",")
                                        .to_string(),
                                    data.6.iter()
                                        .map(|(count, reaction_type)| format!("{}, {}", count, reaction_type))
                                        .collect::<Vec<_>>()
                                        .join(",")
                                        .to_string(),
                                    data.7.iter()
                                        .map(|(id, content)| format!("{}: {}", id, content))
                                        .collect::<Vec<_>>()
                                        .join(",")
                                        .to_string(),
                                    data.8.iter()
                                        .map(|(nick, roles)| format!("{}: {}", nick.as_ref().map_or("NONE", |n| n), roles))
                                        .collect::<Vec<_>>()
                                        .join(",")
                                        .to_string(),
                                ]);
                            }
                        }
                        Err(error) => {
                            println!("Error fetching messages: {:?}", error);
                            break;
                        }
                    }
                }
            }
        },
        Err(error) => println!("{:?}", error),
    }
}