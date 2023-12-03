import os
import json
import pandas as pd

pair1 = pd.read_csv('assistant_processed_chat/chat1.csv')
pair2 = pd.read_csv('assistant_processed_chat/chat2.csv')
pair3 = pd.read_csv('assistant_processed_chat/chat3.csv')

pair2.columns = pair1.columns
pair3.columns = pair1.columns

df = pd.concat([pair1, pair2, pair3], ignore_index=True)
print(len(df))