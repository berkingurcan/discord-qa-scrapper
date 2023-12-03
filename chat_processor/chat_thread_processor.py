import os
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

dir_path = 'chat_archived_threads'
files = []

for filename in os.listdir(dir_path):
    if filename.endswith(".csv"):
        files.append(filename)

os.mkdir('chat_archived_threads/textfiles')
os.mkdir('chat_archived_threads/results')

def export_json(number_of_csv):
    if not os.path.exists(f"chat_archived_threads/{number_of_csv}"):
        return
    
    column_names = [
        "MessageID", "ThreadID", "author", 
        "content", "Timestamp", "ReferencedMessage", 
        "Reactions", "Mentions", "Member"
    ]
    
    df = pd.read_csv(f"chat_archived_threads/{number_of_csv}", header=None)
    df.columns = column_names

    df = df.iloc[::-1]
    df['author'].fillna('Unknown', inplace=True)
    df['content'].fillna('None', inplace=True)

    df['ReferencedMessage'].fillna('None', inplace=True)
    df['Reactions'].fillna(0, inplace=True)
    df['Mentions'].fillna('None', inplace=True)
    df['Member'].fillna(0, inplace=True)

    if df.empty:
        return
    
    question = {str(df.iloc[0]['author']): df.iloc[0]['Mentions'] }

    answers = []
    for i in range(1, len(df)):
        answers.append({str(df.iloc[i]['author']): df.iloc[i]['content']})

    data = [
        question,
        answers
    ]

    file_path = f"chat_archived_threads/textfiles/processed{number_of_csv[:-4]}.txt"
    json_data = json.dumps(data, indent=2)

    with open(file_path, 'w') as file:
        file.write(json_data)


for file in files:
    print(f"Processing {file}")
    export_json(file)