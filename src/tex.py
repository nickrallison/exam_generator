from json_file_io import input_json, get_unit_length
import random
import os
from typing import List, Set, Dict, Tuple

class Config:
    spacing: str = "1"
    font: float = 11
    question_spacer: str = "\\newpage"

    debugging: bool = False

    def __init__(self, debugging=False):
        pass


class TexDocument:

    school_class: str = "PHYS 259"
    term: str = "Winter 2023"
    exam_identifier: str = "Midterm Exam"
    time_limit: str = "50 Minutes"
    exam_date: str = "Today"
    boilerplate: str = "Boilerplate"
    questions: List[str] = []
    header: str = """   
        \\pagestyle{head}
        \\firstpageheader{}{}{}
        \\runningheader{\\class}{\\examnum\\ - Page \\thepage\\ of \\numpages}{\\examdate}
        \\runningheadrule
        """
    num_questions: int = 3
    catagories: List[str] = ["Coulomb's Law"]
    seed: int = 0xFFFFFFFF
    preamble: str = """
\\documentclass[11pt]{exam}
\\RequirePackage{amssymb, amsfonts, amsmath, latexsym, verbatim, xspace, setspace}
\\RequirePackage{tikz, pgflibraryplotmarks}
\\usepackage[margin=1in]{geometry}
\\usepackage{float}

\\newcommand{\\class}{(Class)}
\\newcommand{\\term}{(Term)}
\\newcommand{\\examnum}{(Exam Num)}
\\newcommand{\\examdate}{(Date)}
\\newcommand{\\timelimit}{(Time Limit)}
\\newcommand{\\seed}{(Seed)}
\\newcommand{\\questionspaste}{(Questions)}
\\newcommand{\\headerpaste}{(Header)}
\\newcommand{\\boilerplatepaste}{(Boilerplate)}
\\newcommand{\\parindentpaste}{0ex}
        
        """
    config: Config = Config()
    tex_folder: str = "latex"

    random.seed(seed)

    debugging: bool = False

    def __init__(self, text_folder: str, latex_folder: str, debugging=False):
        self.debugging = debugging

    def get_questions(self, class_name: str, questions_file: str, units: List[str], questions_per_unit: List[int]) -> None:
        json_questions: dict = input_json(questions_file)
        
        #if self.debugging:
            #for (index, unit) in enumerate(units):
                #assert get_unit_length(json_questions, class_name, unit) >= questions_per_unit[index], f"More questions chosen: {questions_per_unit[index]}, than are contained in {unit}: {get_unit_length(questions_file, class_name, unit)}"
            #assert
            
        # Generates random indices for questions & grabs those questions & appends them to the question list
        # Also shuffles the list
        for (unit_index, unit) in enumerate(units):
            questions_chosen: List[int] = random.sample(range(get_unit_length(json_questions, class_name, unit)), questions_per_unit[unit_index])
            for question_index in questions_chosen:
                self.questions.append(self.format_questions(json_questions[class_name][unit][question_index]))
        random.shuffle(self.questions)
        return

    def format_questions(self, question: dict) -> str:
        question_text = """
\\addpoints
        """
        if "image_source" in question:
            question_text+="""
\\begin{figure}[H]\n
\\centering
\\includegraphics[scale="""+question["image_scale"]+"""]{(Image)}    
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
        document = document.replace("(Seed)", "0xFFFFFF")
        document = document.replace("(Questions)", questions_temp)
        document = document.replace("(Header)", self.header)
        document = document.replace("(Boilerplate)", self.boilerplate)
        return document
        
    def print_tex(self, file_name):
        latex_file = os.path.join(self.tex_folder, file_name)
        f = open(latex_file, "w+")
        f.write(self.format_document())
        f.close()
