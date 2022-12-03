mod parser;

use std::fs;

fn main() {
    println!("Reading input data...");

    let input = fs::read_to_string("input").expect("Should have been able to read the file");
    let parsed = parser::part1_most_calories(&input);
    println!("The Elf number {:?} has the most calories: {:?}", parsed[0], parsed[1]);

    println!("PART2:");
    let parsed = parser::part2_top_calories(&input);

    println!("Elf top:");
    let mut total : usize = 0;
    for elf in parsed {
        total += elf[1];
        println!("Elf NR: {:?}: {:?}", elf[0], elf[1]);
    }
    println!("Total for those elves: {:?}", total);
}
