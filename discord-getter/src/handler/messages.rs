use serenity::prelude::*;
use serenity::model::id::{ChannelId, MessageId};
use csv::Writer;

use crate::constants::*;


/*
  ____ _____ _____    ____ _   _    _    _   _ _   _ _____ _        ____ _   _    _  _____ 
 / ___| ____|_   _|  / ___| | | |  / \  | \ | | \ | | ____| |      / ___| | | |  / \|_   _|
| |  _|  _|   | |   | |   | |_| | / _ \ |  \| |  \| |  _| | |     | |   | |_| | / _ \ | |  
| |_| | |___  | |   | |___|  _  |/ ___ \| |\  | |\  | |___| |___  | |___|  _  |/ ___ \| |  
 \____|_____| |_|    \____|_| |_/_/   \_\_| \_|_| \_|_____|_____|  \____|_| |_/_/   \_\_|  
*/
pub async fn handle_chat(ctx: &Context) {
    // Getting Main channel chat data
    let channel_id = ChannelId(CHANNEL_ID);
    let mut last_message_id = None;

    println!("Getting channel messages");

    let mut counter = 0;

    loop {
        counter += 1;
        let mut writer = Writer::from_path(format!("./outputs/chat/output{}.csv", counter)).unwrap();
        writer.write_record(&[
            "id",
            "channel_id",
            "author",
            "content",
            "timestamp",
            "mentions",
            "reactions",
            "referenced_message",
            "member",
        ]);

        let _messages = channel_id
            .messages(&ctx, |retriever| {
                retriever.after(last_message_id.unwrap_or(MessageId(AFTER_MESSAGE_ID))).limit(LIMIT)
            })
            .await;
    


        match _messages {
            Ok(messages) => {
                if messages.is_empty() {
                    break;
                }

                last_message_id = Some(messages.last().unwrap().id);

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
            Err(error) => {
                println!("Error fetching messages: {:?}", error);
                break;
            }
        }
    }
}

/*
  _______                      __  ________                   __  
 / ___/ /  ___ ____  ___  ___ / / /_  __/ /  _______ ___ ____/ /__
/ /__/ _ \/ _ `/ _ \/ _ \/ -_) /   / / / _ \/ __/ -_) _ `/ _  (_-<
\___/_//_/\_,_/_//_/_//_/\__/_/   /_/ /_//_/_/  \__/\_,_/\_,_/___/
*/
pub async fn handle_archived_channel_threads(ctx: &Context) {
    let channel_id = ChannelId(CHANNEL_ID);
    let _archived_channel_threads = channel_id.get_archived_public_threads(&ctx, None, None);
    
    match (_archived_channel_threads).await {
        Ok(data) => {
            let requested_channel_id = Some(ChannelId(CHANNEL_ID));
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
                let mut writer = Writer::from_path(format!("./outputs/chat_archived_threads/{}.csv", thread_id.to_string())).unwrap();

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