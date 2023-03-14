import json
from io import open
import os
from typing import List, Set, Dict, Tuple

def input_json(file_name: str, debugging=False) -> dict:
    json_file = os.path.join("text", file_name)
    return json.loads(open(json_file).read())

def get_units(json_file: dict, class_name: str, debugging=False) -> List[str]:
    return list(json_file[class_name].keys())

def get_unit_length(json_file: dict, class_name: str, unit: str, debugging=False) -> int:
    return len(json_file[class_name][unit])
