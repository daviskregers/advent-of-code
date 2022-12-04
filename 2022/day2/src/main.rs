mod parser;

use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Should have been able to read the file");
    let parsed = parser::Parser::new(input);

    println!("TOTAL SCORE: {:?}", parsed.get_score_total());
}
