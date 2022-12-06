use core::str::FromStr;
use regex::Regex;

pub fn parse_stacks(input: &str) -> StackSet
{
    let mut stacks : Vec<Vec<String>> = vec![];
    let mut commands : Vec<Command> = vec![];
    let lines = input.split("\n").collect::<Vec<&str>>();
    let count = get_stack_count(lines.clone());

    for _ in 0..count.num_stacks {
        stacks.push(vec![]);
    }

    let mut current_line : String;
    for line in (0..count.linenr).rev() {
        current_line = cleanup_string(lines[line]);
        let chars = current_line.chars().collect::<Vec<char>>();
        for s in 0..count.num_stacks {
            let item : String = chars[s].to_string();
            if item != "-" {
                stacks[s].push(item);
            }
        }
    }

    for line in (count.linenr + 1)..lines.len() {
        let ln = lines[line];
        if ln.is_empty() {
            continue;
        }
        let command = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let replaced: String = command
            .replace_all(ln, "$1 $2 $3")
            .to_string();
        let after = replaced
            .split(" ")
            .map(|x| FromStr::from_str(x).expect("to be parsable usize"))
            .collect::<Vec<usize>>();
        commands.push(Command {count: after[0], from: after[1], to: after[2]});
    }

    StackSet::new(stacks, commands)
}

#[derive(Debug, PartialEq, Clone)]
pub struct StackSet {
    stacks: Vec<Vec<String>>,
    commands: Vec<Command>
}

impl StackSet {
    fn new(stacks: Vec<Vec<String>>, commands: Vec<Command>) -> Self { Self { stacks, commands} }
    pub fn execute(&mut self) -> bool {
        if self.commands.is_empty() {
            return false
        }
        let command = self.commands[0];
        self.commands.remove(0);
        println!("COMMAND: {:?}", command);

        for i in 0..command.count {
            let item = self.stacks[command.from-1].pop();
            match item {
                Some(itm) => self.stacks[command.to-1].push(itm),
                _ => continue
            }
        }

        true
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Command {
    count: usize,
    from: usize,
    to: usize
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

fn cleanup_string(input : &str) -> String
{
    let items = Regex::new(r"\[(\w)\]").unwrap();
    let empty = Regex::new(r"   ").unwrap();
    let non_breaking_whitespace = Regex::new(r" ").unwrap();
    let mut after = items.replace_all(input, "$1").to_string();
    after = empty.replace_all(after.as_str(), "-").to_string();
    non_breaking_whitespace.replace_all(after.as_str(), "").to_string()
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

        let expected : StackSet = StackSet::new(vec![
            vec![String::from("Z"), String::from("N")],
            vec![String::from("M"), String::from("C"), String::from("D")],
            vec![String::from("P")],
        ], vec![
            Command { count: 1, from: 2, to: 1},
            Command { count: 3, from: 1, to: 3},
            Command { count: 2, from: 2, to: 1},
            Command { count: 1, from: 1, to: 2}
        ]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_execute_command() {
        let initial : StackSet = StackSet::new(vec![
            vec![String::from("Z"), String::from("N")],
            vec![String::from("M"), String::from("C"), String::from("D")],
            vec![String::from("P")],
        ], vec![
            Command { count: 1, from: 2, to: 1},
            Command { count: 3, from: 1, to: 3},
            Command { count: 2, from: 2, to: 1},
            Command { count: 1, from: 1, to: 2}
        ]);
        let mut actual = initial.clone();
            actual.execute();
        let expected : StackSet = StackSet::new(vec![
            vec![String::from("Z"), String::from("N"), String::from("D")],
            vec![String::from("M"), String::from("C")],
            vec![String::from("P")],
        ], vec![
            Command { count: 3, from: 1, to: 3},
            Command { count: 2, from: 2, to: 1},
            Command { count: 1, from: 1, to: 2}
        ]);

        assert_eq!(actual, expected);

        actual.execute();
        actual.execute();
        actual.execute();

        let finished : StackSet = StackSet::new(vec![
            vec![String::from("C")],
            vec![String::from("M")],
            vec![String::from("P"), String::from("D"), String::from("N"), String::from("Z")],
        ], vec![]);

        assert_eq!(actual, finished);

        actual.execute();
        assert_eq!(actual, finished);
    }
}
