use std::fs;
use regex::Regex;

pub fn parse_questions(file_path: String) -> Vec<Question> {
    let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the question input text file");
    
    let regex_key_value = Regex::new(r"^(?P<key>([^:]+))[:][ ](?P<val>([^:]+))$").unwrap(); 
	let regex_four_space_indent = Regex::new(r"^[^\S\t\n\r]{4}(?P<key>([^:]+))[:][ ](?P<val>([^:]+))$").unwrap(); 
    // Looks like a nightmare but captures each key/value pair in the input file
    
    let mut question: Question = Default::default();
    
    let mut line_num = 0;

    let mut question_vec: Vec<Question> = Vec::new();
    for line in contents.lines() {
        
	    if let Some(c) = regex_key_value.captures(line) {
            let key = &c["key"];
            println!("Key: {}, Val: {}", key, &c["val"]);
            match &c["key"] {
                "Class" => question.class = (c["val"]).to_string(),
                "Section" => question.section = (c["val"]).to_string(),
                "Text" => question.text = insert_value(c["val"].to_string(), ranges),
                "ID" => question.id = (c["val"]).to_string().parse::<u32>().unwrap(),
                "Figure" => question.figure_type = (c["val"]).to_string(),
                "Source" => question.source = (c["val"]).to_string(),
                "Course Code" => question.code =  (c["val"]).to_string(),
                _ => panic!("Invalid Specifier: {}, at line {}\nDoes not match one of: Class, Course Code, Section, Text, ID, Images or Source", key, line_num+1),
            }
        }
        else {
            println!("~~~");
            question_vec.push(question.clone());
            question = Default::default();
        }
        line_num += 1;
	}
    question_vec.push(question.clone());

    return question_vec;
}

fn insert_value(empty_question: String, range: (f64, f64)) {

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

