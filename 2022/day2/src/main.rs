mod parser;

use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Should have been able to read the file");
    println!("PART 1:");
    let parsed = parser::Parser::from_part1(&input);
    println!("TOTAL SCORE: {:?}", parsed.get_score_total());

    println!("PART 2:");
    let parsed = parser::Parser::from_part2(&input);
    println!("TOTAL SCORE: {:?}", parsed.get_score_total());
}
