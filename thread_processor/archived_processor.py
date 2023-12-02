import json
import os

import pandas as pd

os.mkdir("archived_threads/textfiles")

def export_json(number_of_csv):
    df = pd.read_csv(f"archived_threads/{number_of_csv}.csv")
    df = df.iloc[::-1]

    question = {str(df.iloc[0]['author']): df.iloc[0]['content']}

    answers = []
    for i in range(1, len(df)):
        answers.append({str(df.iloc[i]['author']): df.iloc[i]['content']})

    data = [
        question,
        answers
    ]

    file_path = f"archived_threads/textfiles/processed{number_of_csv}.txt"
    json_data = json.dumps(data, indent=2)

    with open(file_path, 'w') as file:
        file.write(json_data)

    print(f'Data has been exported to {file_path}')

    try:
        with open(file_path, 'r') as file:
            contents = file.read()
            print(contents)
    except FileNotFoundError:
        print(f"File not found at {file_path}")
    except Exception as e:
        print(f"An error occurred: {e}")
    

for i in range(402):
    export_json(i)