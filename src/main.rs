use std::fs;
use regex::Regex;

fn main() {
    // Writing File
    let mut path: String = "src/".to_owned();
    let file_name: String = "foo.tex".to_owned();
    path.push_str(&file_name);
    let data = "Some data!";
    fs::write(path, data).expect("Unable to write file");

    // Questions Input
    let input_file_path = "text/q.txt";
    let contents = fs::read_to_string(input_file_path)
        .expect("Should have been able to read the question input text file");


    // Finds (Start): (End)
    let rx = Regex::new(r"^(?P<key>([^:]+))[:][ ](?P<val>([^:]+))$").unwrap();
    for line in contents.lines() {
        
	    if let Some(c) = rx.captures(line) {
	        println!("Key: {}, Val = {}", &c["key"], &c["val"]);
	    }
        else {
            println!("~~~~~~");
        }
	}

}


/* Saved Regex's
 * --------------
 * ^(?P<key>([^:]+))[:][ ](?P<val>([^:]+))$     -- Saves each key value pair
 * (?P<text>([^~]+))[~]                         -- Saves each text block
 */