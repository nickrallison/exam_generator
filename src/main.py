from tex import TexDocument
import os

def main(debugging=False):
    seed = 0x00F0F0F0
    doc = TexDocument("text", "latex", seed)
    doc.get_questions("PHYS 259", "question_bank.json", ["Coulomb's Law"], [3])
    doc.print_tex("out.tex")

    print_cmd = "pdflatex -output-directory=latex  latex/out.tex"
    returned_value_print = os.system(print_cmd)  # returns the exit code in unix

    #if (not debugging):
    #clean_cmd = "rm -rf latex/*.log && rm -rf latex/*.aux && rm -rf latex/out.tex"
    #returned_value_clean = os.system(clean_cmd)  # returns the exit code in unix

    #if debugging:
        #print('exit code for latex print:', returned_value_print)
        #print('exit code for latex clean:', returned_value_clean)
    pass

if __name__ == '__main__':
    main()