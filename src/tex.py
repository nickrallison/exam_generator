from json_file_io import input_json, get_unit_length
import random
import os
import math
import re
from typing import List, Set, Dict, Tuple

class Config:
    spacing: str = "1"
    font: float = 11
    question_spacer: str = """
\\newpage"""

    debugging: bool = False

    def __init__(self, debugging=False):
        pass


class TexDocument:

    school_class: str = "PHYS 259"
    term: str = "Winter 2023"
    exam_identifier: str = "Midterm Exam"
    time_limit: str = "50 Minutes"
    exam_date: str = "Today"
    boilerplate = ""
    questions: List[str] = []
    header: str = ""
    num_questions: int = 3
    catagories: List[str] = ["Coulomb's Law"]
    seed: int = 0
    preamble: str = ""
    config: Config = Config()
    tex_folder: str = "latex"

    debugging: bool = False

    def __init__(self, text_folder: str, latex_folder: str, seed: int, debugging=False):
        self.debugging = debugging

        with open(os.path.join(latex_folder, "boilerplate.tex"), 'r') as file:
            self.boilerplate = file.read()
        with open(os.path.join(latex_folder, "header.tex"), 'r') as file:
            self.header = file.read()
        with open(os.path.join(latex_folder, "preamble.tex"), 'r') as file:
            self.preamble = file.read()

        self.seed = seed
        self.tex_folder = latex_folder 
        self.text_folder = text_folder

        random.seed(seed)

    def get_questions(self, class_name: str, questions_file: str, units: List[str], questions_per_unit: List[int]) -> None:
        json_questions: dict = input_json(questions_file)
    
        for (unit_index, unit) in enumerate(units):
            questions_chosen: List[int] = random.sample(range(get_unit_length(json_questions, class_name, unit)), questions_per_unit[unit_index])
            for question_index in questions_chosen:
                self.questions.append(self.format_questions(json_questions[class_name][unit][question_index]))
        random.shuffle(self.questions)
        
        regex =  r"\((?P<min>-?\d*\.?\d*), (?P<max>-?\d*\.?\d*)\)"

        for (index, question) in enumerate(self.questions):
            result = re.search(regex, self.questions[index])

            while result != None:
                decimals = 0
                decimals_regex = r"\d(\.(?P<decimals>\d+))?"
                decimals_result = re.search(decimals_regex, result.group("min"))
                if decimals_result.group("decimals") != None:
                    decimals = len(decimals_result.group("decimals"))
                min = float(result.group("min"))
                max = float(result.group("max"))
                val = round(random.uniform(min, max), decimals)
                print(decimals)
                if decimals == 0:
                    val = round(val)

                self.questions[index] = re.sub(regex, str(val), self.questions[index], count=1)
                result = re.search(regex, self.questions[index])



        return

    def format_questions(self, question: dict) -> str:
        question_text = """
            \\addpoints
            \\question """ + question["question"]
        if "image_source" in question:
            question_text+="""
                \\begin{figure}[H]\n
                \\centering
                \\includegraphics[scale="""+question["image_scale"]+"""]{""" + os.path.join("assets", question["image_source"]+".png") + """}    
                \\end{figure}
            """
        question_text+=self.config.question_spacer
        return question_text

    def format_document(self) -> str:
        document: str = ""
        document += self.preamble

        if self.config.spacing == "1":
            document+="\\singlespacing"
        elif self.config.spacing == "1.5":
            document+="\\onehalfspacing"
        elif self.config.spacing == "2":
            document+="\\doublespacing"
        else:
            document+="\\singlespacing"

        document+= """
            \\parindent \\parindentpaste

            \\begin{document} 

            \\headerpaste

            \\boilerplatepaste

            \\begin{questions}

            \\questionspaste

            \\end{questions}
            \\end{document}
        """
        questions_temp = ""
        for question in self.questions:
            questions_temp += question
            questions_temp += "\n"

        document = document.replace("(Class)", self.school_class)
        document = document.replace("(Term)", self.term)
        document = document.replace("(Exam Num)", self.exam_identifier)
        document = document.replace("(Date)", "Today")
        document = document.replace("(Time Limit)", self.time_limit)
        document = document.replace("(Seed)", "0x{:08X}".format(self.seed))
        document = document.replace("(Questions)", questions_temp)
        document = document.replace("(Header)", self.header)
        document = document.replace("(Boilerplate)", self.boilerplate)

        # regex = r"\((?P<min>-?\d*\.?\d*), (?P<max>-?\d*\.?\d*)\)"
        # m = re.findall(document, regex)
        # print(m)

        return document
        
    def print_tex(self, file_name):
        latex_file = os.path.join(self.tex_folder, file_name)
        f = open(latex_file, "w+")
        f.write(self.format_document())
        f.close()



