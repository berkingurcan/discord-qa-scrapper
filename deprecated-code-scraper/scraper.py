import re
import json
import os

def find_deprecated_code(directory):
    deprecated_code = []
    
    

    return deprecated_code

# Replace this with the path to your TypeScript directory
directory = './o1js'

deprecated_items = find_deprecated_code(directory)

# Optionally, write the results to a JSON file
with open('deprecated_code.json', 'w', encoding='utf-8') as json_file:
    json.dump(deprecated_items, json_file, indent=4)
