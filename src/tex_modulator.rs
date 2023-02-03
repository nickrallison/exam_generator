use regex::Regex;
use std::fs;
use question_parser::parse_questions;
mod question_parser;

// Basic question
//  \addpoints
//  \question[10] |
//  \newpage

pub fn make_latex_file(tex_out_path: String, tex_template_path: String, questions_path: String, seed: u64) {
    let tex_template = fs::read_to_string(tex_template_path)
    .expect("Should have been able to read the question input text file");
    let questions = question_parser::parse_questions(questions_path, seed);
    let mut questions_tex: String = "".to_string();
    println!("{}", questions.len());
    for question in questions {
        questions_tex.push_str("\\addpoints\n");
        questions_tex.push_str("\\question[10] ");
        questions_tex.push_str(&(question.text));
        if question.figure_type == "Image" {
            /*
            \begin{figure}[H]
            \centering
            \includegraphics[scale=1.5]{atom.png}
            \end{figure}
            */

            questions_tex.push_str("\\begin{figure}[H]\n");
            questions_tex.push_str("\\centering\n");
            questions_tex.push_str("\\includegraphics[scale=0.8]{assets/");
            questions_tex.push_str(&question.source);
            questions_tex.push_str(".png}\n");
            questions_tex.push_str("\\end{figure}");
        }
        questions_tex.push_str("\n\\newpage\n");
        questions_tex.push_str("\n");
    }
    let tex_questions = str::replace(&tex_template, "(Questions)", &questions_tex);
    let tex_out = str::replace(&tex_questions, "(Seed)", &format!("{:08X}", seed));
    fs::write(tex_out_path, tex_out).expect("Unable to write file");

}