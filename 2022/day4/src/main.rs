mod parser;

use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Should have been able to read the file");
    let pairs = parser::get_pairs(&input);

    println!("TOTAL OVERLAPPING: {:?}", parser::get_overlap(pairs.clone()));
    println!("PARTIAL OVERLAPS: {:?}", parser::get_partial_overlap(pairs))
}
