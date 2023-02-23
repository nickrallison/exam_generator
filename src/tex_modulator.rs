use regex::Regex;
use std::fs;
mod question_parser;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;



 
pub fn make_latex_file(tex_out_path: String, tex_template_path: String, questions_path: String, rng: &mut ChaCha8Rng, seed: u64, question_num: u32, class: String) {
    let tex_template = fs::read_to_string(tex_template_path)
    .expect("Should have been able to read the question input text file");
    let question_bank = question_parser::parse_json(questions_path);
    let questions = question_parser::choose_questions(question_bank, 0, vec!((0, question_num)), rng);
    let mut tex_string: String = parse_latex(questions, tex_template, class, seed);
    fs::write(tex_out_path, tex_string).expect("Unable to write file");

}

fn parse_latex(questions: Vec<question_parser::SingleQuestion>, tex_template: String, class: String, seed: u64) -> String {
    let mut questions_tex: String = "".to_string();
    for question in questions {
        questions_tex.push_str("\\addpoints\n");
        questions_tex.push_str("\\question[10] ");
        questions_tex.push_str(&(question.question));
        if question.source != "none" {
            /*
            \begin{figure}[H]
            \centering
            \includegraphics[scale=1.5]{atom.png}
            \end{figure}
            */

            questions_tex.push_str("\\begin{figure}[H]\n");
            questions_tex.push_str("\\centering\n");
            questions_tex.push_str("\\includegraphics[scale=0.8]{assets/");
            questions_tex.push_str(&question.image_source);
            questions_tex.push_str(".png}\n");
            questions_tex.push_str("\\end{figure}");
        }
        questions_tex.push_str("\n\\newpage\n");
        questions_tex.push_str("\n");
    }
    let tex_questions = str::replace(&tex_template, "(Questions)", &questions_tex);
    let tex_class = str::replace(&tex_questions, "(Class)", &class);
    return str::replace(&tex_class, "(Seed)", &format!("{:08X}", seed));
}