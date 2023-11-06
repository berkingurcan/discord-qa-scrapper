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
    threads2 = guild.threads
    channel_threads = channel.threads
    channel_threads2: AsyncIterator = channel.archived_threads(limit=None)
    archived_threads: list[discord.Thread] = [ t async for t in channel_threads2 ]
    # 4 sets for our 4 different thread types, than calculate intersection between all of them
    # 1. active threads
    # 2. guild threads
    # 3. channel threads
    # 4. archived threads
    active_threads_set = {t.id for t in threads}
    guild_threads_set = {t.id for t in threads2}
    channel_threads_set = {t.id for t in channel_threads}
    archived_threads_set = {t.id for t in archived_threads}

    with open("threads.txt", "a+") as f:
        for t in threads:
            f.write(repr(t) + "\n")
        for t in archived_threads:
            f.write(repr(t) + "\n")

    os.makedirs("threads", exist_ok=True)

    for i in range(len(active_threads_set)):
        with open(f"threads/{i}.csv", "w+") as f:
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
            thread = archived_threads[i]
            history = thread.history(limit=None)
            async for message in history:
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

    print("done")
        


# Run the client
from dotenv import load_dotenv, find_dotenv

_ = load_dotenv(find_dotenv(), override=True)
discord_token = os.getenv('DISCORD_TOKEN')

cor = client.start(discord_token)

asyncio.get_event_loop().run_until_complete(cor)
