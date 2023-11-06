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
    print("threads")
    print(len(threads))
    print(threads[0])
    threads2 = guild.threads
    print("threads")
    print(len(threads2))
    print(threads2[0])
    channel_threads = channel.threads
    print("channel_threads")
    print(len(channel_threads))
    print(channel_threads[0])
    channel_threads2: AsyncIterator = channel.archived_threads(limit=None)
    archived_threads: list[discord.Thread] = [ t async for t in channel_threads2 ]
    print("archived_threads")
    print(len(archived_threads))
    print(archived_threads[0])
    # 4 sets for our 4 different thread types, than calculate intersection between all of them
    # 1. active threads
    # 2. guild threads
    # 3. channel threads
    # 4. archived threads
    print("calculating intersections")
    active_threads_set = {t.id for t in threads}
    print("active_threads_set is ready")
    guild_threads_set = {t.id for t in threads2}
    print("guild_threads_set is ready")
    channel_threads_set = {t.id for t in channel_threads}
    print("channel_threads_set is ready")
    archived_threads_set = {t.id for t in archived_threads}
    print("archived_threads_set is ready")
    print("cardinality of intersection of active and guild")
    print(len(active_threads_set.intersection(guild_threads_set)))
    print("cardinality of intersection of active and channel")
    print(len(active_threads_set.intersection(channel_threads_set)))
    print("cardinality of intersection of active and archived")
    print(len(active_threads_set.intersection(archived_threads_set)))
    print("cardinality of intersection of guild and channel")
    print(len(guild_threads_set.intersection(channel_threads_set)))
    print("cardinality of intersection of guild and archived")
    print(len(guild_threads_set.intersection(archived_threads_set)))
    print("cardinality of intersection of channel and archived")
    print(len(channel_threads_set.intersection(archived_threads_set)))
    # append  all active threads to the file threads.txt, than append archived threads to the file
    with open("threads.txt", "a+") as f:
        for t in threads:
            f.write(repr(t) + "\n")
        for t in archived_threads:
            f.write(repr(t) + "\n")
    print("done")
    os.makedirs("threads", exist_ok=True)
    for i in range(10):
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


# Run the client
cor = client.start('MTE1MzM0NTA4NTI5MTkwOTEyMQ.GxwExR.IrdkFOPUiRcVafB_ymZOMcBoNPCbZ4GpA49xRg')
asyncio.get_event_loop().run_until_complete(cor)
