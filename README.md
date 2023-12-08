# Discord QA Scraper

## Description
This project aims to retrieve and process Questions and Answers from specific Discord channel messages and threads. It consists of various components developed in Rust and Python, designed to handle chat messages and both active and archived threads for Discord Forum Channels.

Using Serenity and Python Discord libraries.

## Features
- **Discord Chat Messages and Threads Getter (Rust)**: Retrieves messages and threads from specified Discord channel.
- **Chat Processor and Thread Processor**: Separately processes the retrieved chat messages and threads with Open AI API for upserting a Vector DB as Question Answer Pair.
- **Archived and Active Thread Getter/Scraper (Python)**: Gets both active and archived threads from Discord Forum Channel.
- **Thread Processors (Active and Archived)**: Separate processing modules for active and archived threads with Open AI API. Extracts Question and Answer Pairs for each thread.

# Usage

### Prerequisites
- Rust (version 1.26.0 or higher)
- cargo (version 1.74.0 or higher)
- Python (version 3.9 or higher)
- Created Discord Bot which has authorization read message history and its DISCORD TOKEN

## Discord Getter
It is actually discord bot which can read the message history of the channel. You must create a bot using the Discord developer dashboard.

It downloads message history and threads as batches of csv files.

### Setup:
1. Clone the repository
2. ```cd discord-getter````
3. Create .env file and paste your DISCORD TOKEN which belongs to your Discord Bot
4. Also the bot needs to be in the Server and Channel and has access to read message history of related channel.
5. Go into the src/constants.rs and set your id's like this:
```rs
pub const CHANNEL_ID: u64 = 915745847692636181;
pub const FORUM_CHANNEL_ID: u64 = 1047214314349658172;
pub const GUILD_ID: u64 = 484437221055922177;
```

6. Be sure you have dependencies:
```rs
[dependencies]
csv = "1.2.2"
dotenv = "0.15.0"
serenity = "0.12.0"
tokio = { version = "1.21.2", features = ["macros", "rt-multi-thread"] }
termion = "2.0.1"
```

7. ```cargo run```
8. So, your bot is ready. Send a message to the related Discord Channel and trigger your bot.
9. You can see your csv datas in the ```outputs``` folder.

## Thread Scraper

1. Install Requirements:
    - discord
    - asyncio
2. Create .env file and add your bot's discord token:
```
DISCORD_TOKEN=XXXXXX
```
3. run ```python main.py```
4. It will export all threads from the forum channel as csv files.

## Chat & Thread Processors

### Warning
⚠️ **Important:** Currently having OPENAI API Hanging issue and processors are depend on the API's performance!

These processors use OPENAI API with **gpt-4-1106-preview** model to process csv files. They firstly exports data as txt file for make it easier to process the data for AI. After that, AI Model gets the data and exports as json files.

### Chat Processor

#### Chat Data
First and foremost, create virtual environment and activate it.

Chat datas(batched csv files from the discord-getter) were manually uploaded and processed by OPEN AI Assistant. You need to do that manually for now until the OpenAI Assistant works stable and consistently. You can find the related prompts and codes in the notebooks folder. 

- Firstly, create ```assistant_processed_chat```folder and you must copy and paste assistant processed data to the folder named like sequentally: _*chat1.csv*_, _*chat2.csv*_, _*chat3.csv*_

- Then you can run the processor like ```python processor.py```.

#### Archived Chat Threads

They are threads in the chat channel. 

- So you should copy and paste them into a folder named ```chat_archived_threads```. They have unique names which are the ids of threads. But the code will handle it.

- You can run ```python chat_thread_processor.py```.


### Thread Processor

First and foremost, create virtual environment and activate it.

- Create ```.env``` file and add your OPEN AI API KEY as in the example env.

- Copy and paste folders ```active_threads``` and ```archived_threads``` which you got from the *thread_scraper* under the thread_processor.

- You can run these scripts and it will process datas automatically.



