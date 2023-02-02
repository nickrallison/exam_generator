use std::fs;
use regex::Regex;
mod question_parser;

// TODO
// Format into library export
// Better Documentation & Uses
// Better panic checks
// Auto build TeX pdf
// Question ID calculation
// Unique exam specifier



fn main() {
    let path_to_questions = "text/q.txt".to_string();
    let questions = question_parser::parse_questions(path_to_questions);
    
}
