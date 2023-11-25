import os
import time
import logging
import pandas as pd
import openai
import dotenv
dotenv.load_dotenv()

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

client = openai.OpenAI(
    api_key="",
)


def process_csv(path):
    df = pd.read_csv(path)
    if len(df) == 0:
        return None
    df = df.iloc[::-1]

    file_path = f"processed/{path}"
    os.makedirs(os.path.dirname(file_path), exist_ok=True)
    with open(file_path, 'w+') as file:
        for _, row in df.iterrows():
            reactions = row["reactions"].count(",")+row["reactions"].count("[")
            file.write(f"(reactions: {reactions}){row['member']}: {row['content']}\n")
    return file_path


def post_process(path):
    with open(path, 'r') as file:
        lines = list(file.readlines())
        text = "\n".join(lines)
    
    result = openai.chat.completions.create(
        model="gpt-4-1106-preview",
        messages=[
            {
                "role": "system",
                "content": 
"""
You are a QA Forum data processor.
It is thread datas from a Discord QA Forum Channel.

It is a list of messages in a discord channel, the first message is the probably question and the rest are probably answers.
All messages are from the same thread in a forum channel. Read all the messages and infer the question and answer pair.
Find the correct answer for the question and return it as the final result.

Infer, evaulate and create Full Question: Detailed Answer Pair
Return a valid JSON as the final result, if there is no answer in the messages, return null. Thank you is not an answer, this data will be used for training so please remove unnecessary data.
Give me a JSON file with the following format in markdown format:
```json
{
"question": "The question",
"answer": "The answer" or None
}
```
"""
            },
            {
                "role": "user",
                "content": text
            }
        ])
    result = result.choices[0].message.content
    path = path.replace("processed", "post_processed")
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, 'w+') as file:
        file.write(result)



if __name__ == "__main__":
    for path in os.listdir("Archive/active_threads"):
        path = f"Archive/active_threads/{path}"
        logger.info(f"Processing %s", path)
        processed_path = process_csv(path)
        if processed_path is None:
            logger.info(f"Skipping %s", path)
            continue
        logger.info(f"Post Processing %s", processed_path)
        post_process(processed_path)
        logger.info(f"Processed %s", path)
    for path in os.listdir("Archive/archived_threads"):
        path = f"Archive/archived_threads/{path}"
        logger.info(f"Processing %s", path)
        processed_path = process_csv(path)
        if processed_path is None:
            logger.info(f"Skipping %s", path)
            continue
        logger.info(f"Post Processing %s", processed_path)
        post_process(processed_path)
        logger.info(f"Processed %s", path)
