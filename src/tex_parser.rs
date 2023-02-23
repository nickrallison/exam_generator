use regex::Regex;
use std::fs;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rand::prelude::*;
use rand::seq::SliceRandom;
use std::convert::TryFrom;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct QuestionBank {
    classes: Vec<ClassQuestionBank>,
}
#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct ClassQuestionBank {
    class: String,
    units: Vec<UnitQuestionBank>,
}
#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct UnitQuestionBank {
    questions: Vec<SingleQuestion>,
    unit: String
}
#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct SingleQuestion {
    pub question: String,
    pub image_source: String,
    pub source: String
}

pub(crate) fn parse_json(file_path: String) -> QuestionBank {
    let contents = fs::read_to_string(file_path).unwrap();
    serde_json::from_str(&contents).unwrap()
}

pub(crate) fn choose_questions(question_bank: QuestionBank, class: u32, units: Vec<(u32, u32)>, rng: &mut ChaCha8Rng) -> Vec<SingleQuestion> {
    let mut questions_out: Vec<SingleQuestion> = Vec::new();
    let class_index: usize = usize::try_from(class).unwrap();
    for unit in units { //(unit, questions in unit)
        let unit_index: usize = usize::try_from(unit.0).unwrap();
        let indices: Vec<u32> = gen_rand_non_matching(unit.1, question_bank.classes[class_index].units[unit_index].questions.len().try_into().unwrap(), rng);
        for index in indices {
            let questions_index: usize = usize::try_from(index).unwrap();
            questions_out.push(question_bank.classes[class_index].units[unit_index].questions[questions_index].clone());
        }
    }
    questions_out.shuffle(rng);
    return questions_out;
}

fn gen_rand_non_matching(num_qs: u32, question_bank_size: u32, rng: &mut ChaCha8Rng) -> Vec<u32> {

    let mut rand_vec: Vec<u32> = Vec::new();
    for i in 0..num_qs {
        while true {
            //let rand_instance = (*rng).gen_range(0..question_size);
            let rand_index = (*rng).gen_range(0..question_bank_size);
            let found =  (&rand_vec).into_iter().position(|r| r == &rand_index); 
            match found {
                None => {
                    rand_vec.push(rand_index);
                    break;
                },
                Some(_question_index) => {},
            }
            
        }
    }

    return rand_vec;
} 
 
pub fn make_latex_file(tex_out_path: String, tex_template_path: String, questions_path: String, rng: &mut ChaCha8Rng, seed: u64, question_num: u32, class: String) {
    let tex_template = fs::read_to_string(tex_template_path)
    .expect("Should have been able to read the question input text file");
    let question_bank = parse_json(questions_path);
    let questions = choose_questions(question_bank, 0, vec!((0, question_num)), rng);
    let mut tex_string: String = parse_latex(questions, tex_template, class, seed);
    fs::write(tex_out_path, tex_string).expect("Unable to write file");

}

fn parse_latex(questions: Vec<SingleQuestion>, tex_template: String, class: String, seed: u64) -> String {
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