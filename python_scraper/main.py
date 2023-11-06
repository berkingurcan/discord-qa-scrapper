from typing import AsyncIterator
import discord
import asyncio
import os
import csv

# Create a new instance of the Discord client
intents = discord.Intents.default()
intents.message_content = True
client = discord.Client(intents=intents)

CHANNEL_ID = 915745847692636181
FORUM_CHANNEL_ID = 1047214314349658172
GUILD_ID = 484437221055922177

@client.event
async def on_ready():
    print(f'Logged in as {client.user}')

    # Specify the guild and channel by ID
    guild = discord.utils.get(client.guilds, id=GUILD_ID)
    channel = discord.utils.get(guild.channels, id=FORUM_CHANNEL_ID)

    if guild is None or channel is None:
        print('Guild or Channel not found')
        return

    threads = await guild.active_threads()
    channel_threads: AsyncIterator = channel.archived_threads(limit=None)
    archived_threads: list[discord.Thread] = [ t async for t in channel_threads ]

    thread_ids = [id.id for id in threads]
    archived_thread_ids = [id.id for id in archived_threads]

    os.makedirs('active_threads', exist_ok=True)
    os.makedirs('archived_threads', exist_ok=True)

    for counter, thread_id in enumerate(thread_ids):
        thread = guild.get_thread(thread_id)
                
        if thread:
            with open(f"active_threads/{counter}.csv", "w", newline='') as f:
                try:
                    message_writer = csv.writer(f)
                    message_writer.writerow([
                        "id",
                        "channel_id",
                        "author",
                        "content",
                        "timestamp",
                        "mentions",
                        "reactions",
                        "referenced_message",
                        "member"
                    ])
                except:
                    print("error")

                print(f"Retrieving messages for thread: {thread.name}")
                async for message in thread.history(limit=100):
                    try:
                        message_writer = csv.writer(f)
                        message_writer.writerow([
                            message.id,
                            message.channel.id,
                            message.author.id,
                            message.content,
                            message.created_at,
                            message.mentions,
                            message.reactions,
                            message.reference.message_id if message.reference is not None else None,
                            message.author
                        ])
                    except Exception as e:
                        print(e)
        else:
            print(f"Thread with ID {thread_id} not found in guild {guild.name}")

    print("Getting archived threads")
    for counter, thread in enumerate(archived_threads):
        if thread:
            with open(f"archived_threads/{counter}.csv", "w") as f:
                message_writer = csv.writer(f)
                message_writer.writerow([
                    "id",
                    "channel_id",
                    "author",
                    "content",
                    "timestamp",
                    "mentions",
                    "reactions",
                    "referenced_message",
                    "member"
                ])

                print(f"Retrieving messages for thread: {thread.name}")
                async for message in thread.history(limit=100):
                    try:
                        message_writer.writerow([
                            message.id,
                            message.channel.id,
                            message.author.id,
                            message.content,
                            message.created_at,
                            message.mentions,
                            message.reactions,
                            message.reference.message_id if message.reference is not None else None,
                            message.author
                        ])
                    except Exception as e:
                        print(e)
        else:
            print(f"Thread with ID {thread_id} not found in guild {guild.name}")

# Run the client
from dotenv import load_dotenv, find_dotenv

_ = load_dotenv(find_dotenv(), override=True)
discord_token = os.getenv('DISCORD_TOKEN')

cor = client.start(discord_token)

asyncio.get_event_loop().run_until_complete(cor)
