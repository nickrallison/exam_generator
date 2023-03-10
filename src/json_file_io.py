import json
from io import open
from pathlib import Path

def input_json(file_name: str, debugging=False) -> dict:
    json_folder = Path("../text")
    json_file = json_folder / file_name
    return json.loads(open(file_to_open).read)

def get_units(json_file: dict, class_name: str, debugging=False) -> [str]:
    return list(json_file[class_name].keys())

def get_unit_length(json_file: dict, class_name: str, unit: str, debugging=False) -> int:
    return len(json_file[class_name][unit])
