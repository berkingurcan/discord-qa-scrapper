import os
from uuid import uuid4
import openai
import pinecone
import time
from json import loads
from dotenv import load_dotenv, find_dotenv
_ = load_dotenv(find_dotenv(), override=True) # read local .env file

openai.api_key = os.getenv('OPENAI_API_KEY') or 'OPENAI_API_KEY'
pinecone_api_key = os.getenv('PINECONE_API_KEY') or 'YOUR_API_KEY'
pinecone_env = os.getenv('PINECONE_ENVIRONMENT') or "YOUR_ENV"
print(pinecone_api_key)
pinecone.init(api_key=pinecone_api_key, environment=pinecone_env)

index_name = 'qa-forum-index'

if index_name not in pinecone.list_indexes():
    pinecone.create_index(
        name=index_name,
        metric='dotproduct',
        dimension=1536
    )
    while not pinecone.describe_index(index_name).status['ready']:
        time.sleep(1)

index = pinecone.Index(index_name)

def embed_json_file(path):
    with open(path, 'r') as file:
        lines = list(file.readlines())
        text = "\n".join(lines)
    
    json_content = text.split("```json")[1].split("```")[0]
    json_content = json_content.replace("\n", "")
    data = loads(json_content)
    embedding = openai.embeddings.create(
        input=data["question"],
        model= 'text-embedding-ada-002'
    )
    vector_data = embedding.data[0].embedding
    vectors = [(
        str(uuid4()),
        vector_data,
        {
            "question": data["question"],
            "answer": data["answer"]
        }
    )]
    index.upsert(vectors)

embed_json_file("data.csv")