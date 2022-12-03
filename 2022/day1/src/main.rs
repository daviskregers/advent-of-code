mod parser;

use std::fs;

fn main() {
    println!("Reading input data...");

    let input = fs::read_to_string("input").expect("Should have been able to read the file");
    let parsed = parser::part1_most_calories(&input);
    println!("The Elf number {:?} has the most calories: {:?}", parsed[0], parsed[1]);
}
