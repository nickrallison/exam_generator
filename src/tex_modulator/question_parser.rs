use std::fs;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
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




/* Saved Regex's
 * --------------
 * ^(?P<key>([^:]+))[:][ ](?P<val>([^:]+))$     -- Saves each key value pair
 * (?P<text>([^~]+))[~]                         -- Saves each text block
 */

