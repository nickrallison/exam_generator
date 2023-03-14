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
    exam_date: str = "Today"
    questions: List[str] = [
        """\\addpoints
        \\question[10] Figure 21-12 shows three pairs of identical spheres that are to be touched together and then separated. The initial charges on them are indicated. Rank the pairs according to\begin{figure}[H]
        \\centering
        \\includegraphics[scale=0.8]{assets/Halliday_ch21q2.png}
        \\end{figure}
        \\newpage""",

        """\\addpoints
        \\question[10] In Fig. 21-26, particle 1 of charge -3.453011$\mu$ C and particle  2 of charge 3$\mu$ C are held at separation L = 211 cm on an x axis. If particle 3 of unknown charge q3 is to be located such that the net electrostatic force on it from particles 1 and 2 is zero, what must be the (a) x and (b) y coordinates of particle 3?\begin{figure}[H]
        \\centering
        \\includegraphics[scale=0.8]{assets/Halliday_ch21p13.png}
        \\end{figure}
        \\newpage""",

        """\\addpoints
        \\question[10] In Fig. 21-15, a central particle of charge -q is surrounded by two circular rings of charged particles. What are the magnitude and direction of the net electrostatic force on the central particle due to the other particles? (Hint - Consider symmetry.)\begin{figure}[H]
        \\centering
        \\includegraphics[scale=0.8]{assets/Halliday_ch21q5.png}
        \\end{figure}
        \\newpage""",

        """\\addpoints
        \\question[10] Figure 21-14 shows two charged particles on an axis. The charges are free to move. However, a third charged particle can be placed at a certain point such that all three particles are then in equilibrium. (a) Is that point to the left of the first two particles, to their right, or between them? (b) Should the third particle be positively or negatively charged? (c)  Is the equilibrium stable or unstable?\begin{figure}[H]
        \\centering
        \\includegraphics[scale=0.8]{assets/Halliday_ch21q4.png}
        \\end{figure}
        \\newpage""",

        """\addpoints
        \\question[10] Figure 21-13 shows four situations in which charged particles are fixed in place on an axis. In which situations is there a point to the left of the particles where an electron will be in equilibrium?\begin{figure}[H]
        \\centering
        \\includegraphics[scale=0.8]{assets/Halliday_ch21q3.png}
        \\end{figure}
        \\newpage"""
    ]
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

        \\newcommand{\\class}{""" + school_class + """}
        \\newcommand{\\term}{(Term)}
        \\newcommand{\\examnum}{(E)}
        \\newcommand{\\examdate}{Today}
        \\newcommand{\\timelimit}{50 Minutes}
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
        \\(Question)
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
        return document
        
    def print_tex(self, file_name):
        latex_file = os.path.join(self.tex_folder, file_name)
        f = open(latex_file, "w+")
        f.write(self.format_document())
        f.close()
