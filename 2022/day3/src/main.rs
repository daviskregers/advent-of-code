mod parser;

use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Should have been able to read the file");

    println!("PART 1:");
    let parsed = parser::get_sum_priorities(&input, &parser::PriorityTable::new());
    println!("TOTAL SCORE: {:?}", parsed);
}
