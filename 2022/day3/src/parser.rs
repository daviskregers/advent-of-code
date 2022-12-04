const CHAR_TABLE : &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn get_rucksacks(input: &String) -> Vec<&str>
{
    input.split("\n").collect::<Vec<&str>>()
}

fn get_all_matched_items(input: &String) -> Vec<String>
{
    get_rucksacks(input)
        .iter()
        .filter(|x| !x.is_empty())
        .map(|x| {
            let c = get_compartments(x);
            get_matched_items(c[0].as_str(), c[1].as_str())
        })
        .collect::<Vec<String>>()
}

fn get_compartments(rucksack: &str) -> Vec<String>
{
    let size = rucksack.len() / 2;
    if size == 0 {
        panic!("rucksack {:?} size is 0", rucksack);
    }

    rucksack
        .chars()
        .collect::<Vec<char>>()
        .chunks(size)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
}

fn get_matched_items<'a>(i1: &'a str, i2: &'a str) -> String {
    let mut result : Vec<char> = vec![];
    for c1 in i1.chars() {
        if i2.contains(c1) && !result.contains(&c1) {
            result.push(c1);
        }
    }

    result.iter().cloned().collect::<String>()
}

pub fn get_sum_priorities(input: &String, table: &PriorityTable) -> usize {
    get_all_matched_items(input)
        .iter()
        .map(|x| {
            table.get_priority(
                x.as_str().chars().collect::<Vec<char>>().first().unwrap()
            )
        })
        .collect::<Vec<usize>>()
        .iter()
        .sum()
}

fn get_groups(input: &String) -> Vec<[&str;3]> {
    let binding = get_rucksacks(input);
    binding
        .chunks(3)
        .map(|x| {
            let mut r = [""; 3];
            for i in 0..x.len() {
                r[i] = x[i]
            }
            r
        })
        .collect::<Vec<[&str;3]>>()
}

fn get_group_common(group: &[&str; 3]) -> String {
    let mut result : Vec<char> = vec![];
    for c1 in group[0].chars() {
        if
            group[1].contains(c1) &&
            group[2].contains(c1) &&
            !result.contains(&c1) {
            result.push(c1);
        }
    }

    result.iter().cloned().collect::<String>()
}

pub fn get_group_scores(input: &String, table: &PriorityTable) -> usize {
    get_groups(input)
        .iter()
        .filter(|x| !x[0].is_empty())
        .map(|x| -> usize {
            let common = get_group_common(x);
            let ch = *common.as_str().chars().collect::<Vec<char>>().first().unwrap();
            table.get_priority(&ch)
        })
        .sum()
}

pub struct PriorityTable{
    table: Vec<char>
}

impl PriorityTable {
    pub fn new() -> Self {
        let table = CHAR_TABLE.chars().collect::<Vec<char>>();
        Self { table }
    }

    fn get_priority(&self, letter: &char) -> usize {
        self.table
            .iter()
            .position(|&x| x == *letter)
            .unwrap() + 1
    }
}


#[cfg(test)]
mod tests {
    use crate::parser::*;
    const TEST_INPUT : &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_get_rucksacks() {
        let binding = String::from(TEST_INPUT);
        let actual = get_rucksacks(&binding);
        let expected : Vec<&str> = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw"
        ];

        println!("{:?} == {:?}", actual, expected);
        assert_eq!(actual.len(), expected.len(), "Arrays don't have the same length");
        assert!(actual.iter().zip(expected.iter()).all(|(a,b)| a == b), "Arrays are not equal");
    }

    #[test]
    fn test_get_compartments() {
        let actual = get_compartments("vJrwpWtwJgWrhcsFMMfFFhFp");
        let expected : [&str; 2] = ["vJrwpWtwJgWr", "hcsFMMfFFhFp"];

        println!("{:?} == {:?}", actual, expected);
        assert_eq!(actual.len(), expected.len(), "Arrays don't have the same length");
        assert!(actual.iter().zip(expected.iter()).all(|(a,b)| a == b), "Arrays are not equal");

        assert_eq!(
            get_compartments("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            ["jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"]
        );

        assert_eq!(
            get_compartments("PmmdzqPrVvPwwTWBwg"),
            ["PmmdzqPrV", "vPwwTWBwg"]
        );
    }


    #[test]
    fn test_get_matched_items() {
        let input : [&str; 2] = ["vJrwpWtwJgWr", "hcsFMMfFFhFp"];
        let actual = get_matched_items(input[0], input[1]);
        let expected = "p";

        assert_eq!(actual, expected);

        assert_eq!(
            get_matched_items("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"),
            "L"
        );

        assert_eq!(
            get_matched_items("PmmdzqPrV", "vPwwTWBwg"),
            "P"
        );
    }

    #[test]
    fn test_get_all_matched_items() {
        let binding = String::from(TEST_INPUT);
        let actual = get_all_matched_items(&binding);
        let expected = ["p", "L", "P", "v", "t", "s"];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_priority() {
        let table = PriorityTable::new();
        assert_eq!(1,  table.get_priority(&'a'));
        assert_eq!(26, table.get_priority(&'z'));
        assert_eq!(27, table.get_priority(&'A'));
        assert_eq!(52, table.get_priority(&'Z'));
    }

    #[test]
    fn test_get_sum_priorities() {
        let binding = String::from(TEST_INPUT);
        let actual = get_sum_priorities(&binding, &PriorityTable::new());
        let expected = 157;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_groups() {
        let binding = String::from(TEST_INPUT);
        let actual = get_groups(&binding);
        let expected : Vec<[&str; 3]> = vec![
            [
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg"
            ],
            [
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw"
            ]
        ];

        println!("{:?} == {:?}", actual, expected);
        assert_eq!(actual.len(), expected.len(), "Arrays don't have the same length");
        assert!(actual.iter().zip(expected.iter()).all(|(a,b)| a == b), "Arrays are not equal");
    }

    #[test]
    fn test_get_group_common() {
        assert_eq!(
            get_group_common(
                &[
                    "vJrwpWtwJgWrhcsFMMfFFhFp",
                    "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                    "PmmdzqPrVvPwwTWBwg"
                ]
            ), "r"
        );
        assert_eq!(
            get_group_common(
                &[
                    "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                    "ttgJtRGJQctTZtZT",
                    "CrZsJsPPZsGzwwsLwLmpwMDw"
                ]
            ), "Z"
        );
    }

    #[test]
    fn test_get_group_scores() {
        let binding = String::from(TEST_INPUT);
        let actual = get_group_scores(&binding, &PriorityTable::new());
        let expected = 70;

        assert_eq!(actual, expected);
    }
}
