use std::fs;
use regex::Regex;

pub fn parse_questions(file_path: String) -> Vec<Question> {
    let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the question input text file");
    
    let regex = Regex::new(r"^(?P<key>([^:]+))[:][ ](?P<val>([^:]+))$").unwrap(); 
    // Looks like a nightmare but captures each key/value pair in the input file
    
    let mut question: Question = Default::default();
    
    let mut line_num = 0;

    let mut question_vec: Vec<Question> = Vec::new();
    for line in contents.lines() {
        
	    if let Some(c) = regex.captures(line) {
	        println!("Key: {}, Val = {}", &c["key"], &c["val"]);
            let key = &c["key"];
            match &c["key"] {
                "Class" => question.class = (c["val"]).to_string(),
                "Section" => question.section = (c["val"]).to_string(),
                "Text" => question.text = (c["val"]).to_string(),
                "ID" => question.id = (c["val"]).to_string().parse::<u32>().unwrap(),
                "Images" => question.image_number = (c["val"]).to_string().parse::<u32>().unwrap(),
                "Source" => question.source = (c["val"]).to_string(),
                _ => panic!("Invalid Specifier: {}, at line {}\nDoes not match one of: Class, Section, Text, ID, Images or Source", key, line_num+1),
            }
        }
        else {
            question_vec.push(question.clone());
            question = Default::default();
        }
        line_num += 1;
	}

    return question_vec;
}

#[derive(Default, Clone)]
pub struct Question {
    class: String,
    section: String,
    text: String,
    source: String,
    id: u32,
    image_number: u32,
}   


/* Saved Regex's
 * --------------
 * ^(?P<key>([^:]+))[:][ ](?P<val>([^:]+))$     -- Saves each key value pair
 * (?P<text>([^~]+))[~]                         -- Saves each text block
 */

