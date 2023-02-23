use tex_modulator::make_latex_file;
mod tex_modulator;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

// TODO
// Use JSON as opposed to txt
// Better Documentation & Uses
// Better panic checks
// Auto build TeX pdf
// Credit Latex Templates
// Sort Questions
// Deal with multipart questions



fn main() {
    let seed = 0xf0f0f0f0f0;
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let question_num: u32 = 5;
    let path_to_questions = "text/question_bank.json".to_string();
    let class = "PHYS 259";
    make_latex_file("tex/out.tex".to_string(), "tex/template_1.tex".to_string(), path_to_questions, &mut rng, seed, question_num, class.to_string());
    
}
