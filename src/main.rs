use tex_modulator::make_latex_file;

mod tex_modulator;

// TODO
// Better Documentation & Uses
// Better panic checks
// Auto build TeX pdf
// Question ID calculation
// Unique exam specifier
// Credit Latex Templates
// Sort Questions
// Deal with multipart questions



fn main() {
    let path_to_questions = "text/q.txt".to_string();
    make_latex_file("tex/out.tex".to_string(), "tex/template_1.tex".to_string(), path_to_questions);
    
}
