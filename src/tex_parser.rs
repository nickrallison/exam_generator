use std::fs;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rand::seq::SliceRandom;
use regex::Regex;
use std::convert::TryFrom;
use serde::{Deserialize, Serialize};

enum RandGen {
    Float(f32),
    Int(i32),
}

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
    pub ranges: Vec<String>,
    pub image_source: String,
    pub image_scale: String,
    pub source: String,
    pub marks: String,
    pub subparts: Vec<SubQuestion>
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct SubQuestion {
    pub item_marker: String,
    pub question: String,
    pub marks: String,
    
}

pub(crate) fn parse_json_to_question_bank(file_path: String) -> QuestionBank {
    let contents = fs::read_to_string(file_path).unwrap();
    let json: Result<QuestionBank, serde_json::Error> = serde_json::from_str(&contents);

    match json {
        Ok(result) => return result,
        Err(_) => panic!("Incorrect JSON File input / Struct matching Json"),
    }
}

pub(crate) fn choose_questions_from_bank(question_bank: QuestionBank, class: u32, units: Vec<(u32, u32)>, rng: &mut ChaCha8Rng) -> Vec<SingleQuestion> {
    let mut questions_out: Vec<SingleQuestion> = Vec::new();
    let class_index: usize = usize::try_from(class).unwrap();
    for unit in units { //(unit, questions in unit)
        let unit_index: usize = usize::try_from(unit.0).unwrap();
        let indices: Vec<u32> = gen_rand_non_matching_ints(unit.1, question_bank.classes[class_index].units[unit_index].questions.len().try_into().unwrap(), rng);
        for index in indices {
            let questions_index: usize = usize::try_from(index).unwrap();
            questions_out.push(question_bank.classes[class_index].units[unit_index].questions[questions_index].clone());
        }
    }
    questions_out.shuffle(rng);
    return questions_out;
}

fn gen_rand_non_matching_ints(nums_generated: u32, ceiling: u32, rng: &mut ChaCha8Rng) -> Vec<u32> {

    let mut rand_vec: Vec<u32> = Vec::new();
    for _i in 0..nums_generated {
        loop {
            //let rand_instance = (*rng).gen_range(0..question_size);
            let rand_index = (*rng).gen_range(0..ceiling);
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
    let question_bank = parse_json_to_question_bank(questions_path);
    let questions = choose_questions_from_bank(question_bank, 0, vec!((0, question_num)), rng);
    let questions_rand_filled = fill_rand_values(questions, rng);
    let questions_tex = parse_questions_to_tex(questions_rand_filled);
    let tex_key_val_pairs = format_tex_key_values(class, seed, questions_tex);
    let tex_string: String = parse_tex_to_doc(tex_template, tex_key_val_pairs);
    fs::write(tex_out_path, tex_string).expect("Unable to write file");

}

fn fill_rand_values(questions: Vec<SingleQuestion>, rng: &mut ChaCha8Rng) -> Vec<SingleQuestion> {
    let mut questions_filled = questions.clone();

    for (index, question) in questions.iter().enumerate() {
        for range in &question.ranges {
            let val: RandGen = gen_rand_from_range(range.to_string(), rng);
            match val {
                RandGen::Float(val_f) => {
                    questions_filled[index].question = str::replace(&questions_filled[index].question, range, &val_f.to_string());
                }
                RandGen::Int(val_i) => {
                    questions_filled[index].question = str::replace(&questions_filled[index].question, range, &val_i.to_string());
                }
            }
        }
    }

    return questions_filled;
}

fn gen_rand_from_range(range: String, rng: &mut ChaCha8Rng) -> RandGen {
    let range_capture: regex::Regex = Regex::new(r"^\((?P<lower>(-?\d*\.?\d*)), (?P<upper>(-?\d*\.?\d*))\)$").unwrap();
    println!("{}", range);
    let cap = range_capture.captures(&range).unwrap();
    if range.contains(".") {
        let lower: f32 = cap["lower"].parse::<f32>().unwrap();
        let upper: f32 = cap["upper"].parse::<f32>().unwrap();
        RandGen::Float(rng.gen_range(lower..upper))
    }
    else {
        let lower: i32 = cap["lower"].parse::<i32>().unwrap();
        let upper: i32 = cap["upper"].parse::<i32>().unwrap();
        RandGen::Int(rng.gen_range(lower..upper))
    }
}

fn parse_questions_to_tex(questions: Vec<SingleQuestion>) -> String {

    let mut questions_tex: String = "".to_string();
    for question in questions {
        questions_tex.push_str("\\addpoints\n");
        questions_tex.push_str("\\question[10] ");
        questions_tex.push_str(&(question.question));
        if question.source != "none" {

            questions_tex.push_str("\\begin{figure}[H]\n");
            questions_tex.push_str("\\centering\n");
            questions_tex.push_str("\\includegraphics[scale=");
            questions_tex.push_str(&question.image_scale);
            questions_tex.push_str("]{assets/");
            questions_tex.push_str(&question.image_source);
            questions_tex.push_str(".png}\n");
            questions_tex.push_str("\\end{figure}");
        }
        questions_tex.push_str("\n\\newpage\n");
        questions_tex.push_str("\n");
    }
    return questions_tex;

}

fn format_tex_key_values(class: String, seed: u64, questions: String) -> Vec<(String, String)> {
    let mut key_val_pairs: Vec<(String, String)> = Vec::new();
    key_val_pairs.push(("(Seed)".to_owned(), format!("{:08X}", seed)));
    key_val_pairs.push(("(Class)".to_owned(), class));
    key_val_pairs.push(("(Questions)".to_owned(), questions));

    return key_val_pairs;
}

fn parse_tex_to_doc(tex_template: String, tex_key_pairs: Vec<(String, String)>) -> String {
    let mut tex_doc = tex_template;
    for pair in tex_key_pairs {
        tex_doc = str::replace(&tex_doc, &pair.0, &pair.1);
    }

    return tex_doc;
}




