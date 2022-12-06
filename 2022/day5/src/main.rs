mod parser;

use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Should have been able to read the file");
    let mut stackset = parser::parse_stacks(&input);

    while stackset.execute() {
        println!("-----");
        println!("{:?}", stackset);
    }
}
