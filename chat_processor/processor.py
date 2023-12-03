import os
import json
import pandas as pd

pair1 = pd.read_csv('assistant_processed_chat/chat1.csv')
pair2 = pd.read_csv('assistant_processed_chat/chat2.csv')
pair3 = pd.read_csv('assistant_processed_chat/chat3.csv')

pair2.columns = pair1.columns
pair3.columns = pair1.columns

df = pd.concat([pair1, pair2, pair3], ignore_index=True)

os.mkdir('assistant_processed_chat/results')

for index, row in df.iterrows():
    question = row['question']
    answer = row['answer']

    file_path = 'assistant_processed_chat/results/' + str(index) + '.json'
    json_data = json.dumps({
        "question": question,
        "answer": answer
    }, indent=2)
    
    with open(file_path, 'w') as file:
        file.write(json_data)
    
