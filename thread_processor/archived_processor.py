import json
import os
import openai
import pandas as pd

from dotenv import load_dotenv, find_dotenv
_ = load_dotenv(find_dotenv(), override=True) # read local .env file

openai.api_key = os.getenv('OPENAI_API_KEY') or 'OPENAI_API_KEY'

client = openai.OpenAI(
    api_key = openai.api_key
)

def export_json(number_of_csv):
    df = pd.read_csv(f"archived_threads/{number_of_csv}.csv")
    df = df.iloc[::-1]

    thread_name = df.iloc[0]['thread_name']
    question = {str(df.iloc[0]['author']): df.iloc[0]['content'] }

    answers = []
    for i in range(1, len(df)):
        answers.append({str(df.iloc[i]['author']): df.iloc[i]['content']})

    data = [
        thread_name,
        question,
        answers
    ]

    file_path = f"archived_threads/textfiles/processed{number_of_csv}.txt"
    json_data = json.dumps(data, indent=2)

    with open(file_path, 'w') as file:
        file.write(json_data)

    try:
        with open(file_path, 'r') as file:
            contents = file.read()
    except FileNotFoundError:
        print(f"File not found at {file_path}")
    except Exception as e:
        print(f"An error occurred: {e}")
    


""" os.mkdir("archived_threads/textfiles")
os.mkdir("archived_threads/results")

for i in range(402):
    export_json(i) """


PROMPT = """
You are a QA Forum data processor.
It is thread datas from a Discord QA Forum Channel.

It is a name of the thread and the list of messages in a discord channel, the first message is the probably question and the rest are probably answers.
All messages are from the same thread in a forum channel. Read all the messages and infer the full question and answer pair.
If question or answer includes code blocks, read and evaulate the code blocks as well.
Include thread name and full question in question field completely with also code blocks if exists.
Please do not summarize the answer. Explain the answer as detailed as possible with all necessary information.
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

def process_txt(number_of_txt):
    file_path = f"archived_threads/textfiles/processed{number_of_txt}.txt"
    with open(file_path, 'r') as file:
        contents = file.read()

    response = client.chat.completions.create(
        model="gpt-4-1106-preview",
        response_format={ "type": "json_object" },
        messages=[
            {
                "role": "system",
                "content": PROMPT
            },
            {
                "role": "user",
                "content": str(contents)
            }
        ]
    )

    result = response.choices[0].message.content
    print(result)

    return result
    
res = process_txt(86)