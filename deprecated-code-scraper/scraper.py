import re
import json
import os

def extract_comments_and_code_elements(ts_code):
    pattern = r'(/\*\*[\s\S]*?\*/)\s*([^\s].*?)\s*[{;]'

    matches = re.findall(pattern, ts_code)

    extracted = []
    for comment, code_element in matches:
        extracted.append({"comment": comment.strip(), "code": code_element.strip()})

    return extracted

def find_deprecated_code(directory):
    deprecated_code = []
    
    for root, dirs, files in os.walk(directory):
        for file in files:
            if file.endswith(".ts"):
                file_path = os.path.join(root, file)
                with open(file_path, 'r') as f:
                    content = f.read()
                    extracted_content = extract_comments_and_code_elements(content)
                    
                    for f in extracted_content:
                        if 'deprecated' in f['comment']:
                            deprecated_code.append(f)

    return deprecated_code

# Replace this with the path to your TypeScript directory
directory = './o1js'

deprecated_items = find_deprecated_code(directory)

# Optionally, write the results to a JSON file
with open('deprecated_code.json', 'w', encoding='utf-8') as json_file:
    json.dump(deprecated_items, json_file, indent=4)
