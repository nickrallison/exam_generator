import json
from io import open
from pathlib import Path

def input_json(file_name: str) -> dict:
    json_folder = Path("../text")
    json_file = json_folder / file_name
    return json.loads(open(file_to_open).read)
