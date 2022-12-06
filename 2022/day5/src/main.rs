mod parser;

use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Should have been able to read the file");
    let _pairs = parser::parse_stacks(&input);

}
