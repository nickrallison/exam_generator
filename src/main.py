from tex import TexDocument

def main(debugging=False):
    doc = TexDocument("text", "latex")
    doc.get_questions("PHYS 259", "question_bank.json", ["Coulomb's Law"], [3])
    doc.print_tex("out.tex")

    print_cmd = "pdflatex -output-directory=latex  latex/out.tex"
    returned_value = os.system(cmd)  # returns the exit code in unix
    print('returned value:', returned_value)

    if (not debugging):
        pass
    else:
        pass
    pass

if __name__ == '__main__':
    main()