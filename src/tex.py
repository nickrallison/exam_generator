from json_file_io import input_json, get_unit_length
import random
import os
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
    boilerplate: str = """
\\begin{flushright}
\\begin{tabular}{p{2.8in} r l}
\\textbf{\class} & \\textbf{Name (Print):} & \makebox[2in]{\hrulefill}\\\\
\\textbf{\\term} &&\\\\
\\textbf{\examnum} &&\\\\
\\textbf{\examdate} &&\\\\
\\textbf{Time Limit: \\timelimit} & Teaching Assistant & \\makebox[2in]{\\hrulefill}
\\end{tabular}\\
\\end{flushright}
\\rule[1ex]{\\textwidth}{.1pt}


This exam contains \\numpages\ pages (including this cover page) and
\\numquestions\\ problems.  Check to see if any pages are missing.  Enter
all requested information on the top of this page, and put your initials
on the top of every page, in case the pages become separated.\\\\

You may \\textit{not} use your books, notes, or any calculator on this exam.\\\\

You are required to show your work on each problem on this exam.  The following rules apply:\\\\

\\begin{minipage}[t]{3.7in}
\\vspace{0pt}
\\begin{itemize}

\\item \\textbf{If you use a ``fundamental theorem'' you must indicate this} and explain
why the theorem may be applied.

\\item \\textbf{Organize your work}, in a reasonably neat and coherent way, in
the space provided. Work scattered all over the page without a clear ordering will 
receive very little credit.  

\\item \\textbf{Mysterious or unsupported answers will not receive full
credit}.  A correct answer, unsupported by calculations, explanation,
or algebraic work will receive no credit; an incorrect answer supported
by substantially correct calculations and explanations might still receive
partial credit.


\\item If you need more space, use the back of the pages; clearly indicate when you have done this.
\\end{itemize}

Do not write in the table to the right.
\\end{minipage}
\\hfill
\\begin{minipage}[t]{2.3in}
\\vspace{0pt}
%\\cellwidth{3em}
\\gradetablestretch{2}
\\vqword{Problem}
\\addpoints % required here by exam.cls, even though questions haven't started yet.	
\\gradetable[v]%[pages]  
% Use [pages] to have grading table by page instead of question

\\end{minipage}
\\newpage \% End of cover page
    """
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
\\newcommand{\\parindentpaste}{0ex}"""
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
