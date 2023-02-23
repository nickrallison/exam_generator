use core::panic;
use std::fs;
use regex::Regex;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct QuestionBank {
    classes: Vec<ClassQuestionBank>,
}
#[derive(Serialize, Deserialize)]
struct ClassQuestionBank {
    units: Vec<UnitQuestionBank>,
}
#[derive(Serialize, Deserialize)]
struct UnitQuestionBank {
    questions: Vec<SingleQuestion>,
}
#[derive(Serialize, Deserialize)]
struct SingleQuestion {
    question: String,
    source: String
}

pub fn parse_questions(file_path: String, rng: &mut ChaCha8Rng, num_qs: u32) -> Vec<Question> {
    let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the question input text file");
    
    let regex_key_value = Regex::new(r"^(?P<key>([^:]+))[:][ ](?P<val>([^:]+))$").unwrap(); 
	let regex_four_space_indent = Regex::new(r"^[^\S\t\n\r]{4}(?P<key>([^:]+))[:][ ](?P<val>([^:]+))$").unwrap(); 
    let regex_tuple = Regex::new(r"\((?P<x>(-?\d*\.?\d*)), (?P<y>(-?\d*\.?\d*))\)").unwrap(); 
    // Looks like a nightmare but captures each key/value pair in the input file
    let result = serde_json::from_str::<QuestionBank>(&contents);
    let bank;
    match result {
        Ok(questionBank) => {
            bank = questionBank;
        },
        Err(_) => {
            panic!()
        },
    }

    //questio
    
    let indices: Vec<u32> = gen_rand_non_matching(num_qs, question_vec.len().try_into().unwrap(), rng);
    let mut questions_out: Vec<Question> = Vec::new();
    for index in indices {
        questions_out.push(question_vec[index as usize].clone());
    }


    return questions_out;
}


fn insert_values(empty_question: &str, regex: &Regex, rng: &mut ChaCha8Rng) -> String {
    let mut q_holder: String = empty_question.to_string();
    while (true) {
        let mut val: f64;
        if let Some(c) = regex.captures(&q_holder) {
            let mut min = (&c["x"]).to_string().parse::<f64>().unwrap();
            let mut max = (&c["y"]).to_string().parse::<f64>().unwrap();
            if min > max {
                let tmp = max;
                max = min;
                min = tmp;
            }
            val = (*rng).gen_range(min..max);
            if min == min.round() && max == max.round() {
                val = val.round();
            }

        }
        else {
            break;
        }
        q_holder = regex.replace(&q_holder, &val.to_string()).to_string();
    }
    
    
    return q_holder;
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

#[derive(Default, Clone)]
pub struct Question {
    pub class: String,
    pub code: String,
    pub section: String,
    pub text: String,
    pub source: String,
    pub id: u32,
    pub figure_type: String,
}   




/* Saved Regex's
 * --------------
 * ^(?P<key>([^:]+))[:][ ](?P<val>([^:]+))$     -- Saves each key value pair
 * (?P<text>([^~]+))[~]                         -- Saves each text block
 */

