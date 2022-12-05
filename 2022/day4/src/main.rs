mod parser;

use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Should have been able to read the file");

    println!("PART 1:");
    let pairs = parser::get_pairs(&input);
    println!("TOTAL OVERLAPPING: {:?}", parser::get_overlap(pairs));
}
