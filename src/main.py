from tex import TexDocument

def main(debugging=False):
    doc = TexDocument("text", "latex")
    doc.get_questions("PHYS 259", "question_bank.json", ["Coulomb's Law"], [3])
    doc.print_tex("out.tex")
    pass

if __name__ == '__main__':
    main()