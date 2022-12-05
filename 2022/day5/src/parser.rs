use core::str::FromStr;
use regex::Regex;

fn parse_stacks(input: &str) -> Vec<Vec<&str>>
{
    let mut result : Vec<Vec<&str>> = vec![];
    let lines = input.split("\n").collect::<Vec<&str>>();
    let count = get_stack_count(lines.clone());

    // USE THIS:
    // regex replace "   " to -, [\w] to $1, parse to Vec
    let re = Regex::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})").unwrap();
    let before = "2012-03-14, 2013-01-01 and 2014-07-05";
    let after = re.replace_all(before, "$m/$d/$y");
    assert_eq!(after, "03/14/2012, 01/01/2013 and 07/05/2014");
    // place each column into specific stack
    // remove -

    result
}

#[derive(Debug)]
struct StackSize {
    pub linenr: usize,
    pub num_stacks: usize
}

fn get_stack_count(lines: Vec<&str>) -> StackSize
{
    for linenr in 0..lines.len() {
        let s = String::from(*lines.get(linenr).unwrap());
        let l = s.trim();
        if !l.is_empty() && !l.contains("[") {
            let num_stacks : usize = FromStr::from_str(&l.chars().last().unwrap().to_string()).expect("couldn't get stack size");
            return StackSize {linenr, num_stacks}
        }
    }

    panic!("Couldn't find stack size declaration!");
}

#[cfg(test)]
mod tests {
    use crate::parser::*;

    const TEST_INPUT : &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn test_parse_stacks() {
        let binding = String::from(TEST_INPUT);
        let actual = parse_stacks(&binding);

        let mut expected : Vec<Vec<&str>> = vec![
            vec!["Z", "N"],
            vec!["M", "C", "D"],
            vec!["P"],
        ];

        println!("{:?} == {:?}", actual, expected);
        assert_eq!(actual.len(), expected.len(), "Arrays don't have the same length");
        assert!(actual.iter().zip(expected.iter()).all(|(a,b)| a == b), "Arrays are not equal");
    }
}
