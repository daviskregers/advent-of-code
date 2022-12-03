pub fn part1_most_calories(input: &String) -> [usize; 2]
{
    let mut total_max : usize = 0;
    let mut total_index : usize = 0;
    let mut current_total : usize = 0;
    let mut current_index : usize = 0;

    let lines = input.split("\n");

    for item in lines {
        if item.is_empty() {
            if current_total > total_max {
                total_max = current_total;
                total_index = current_index;
            }
            current_total = 0;
            current_index = current_index + 1;
            continue;
        }
        let units = item.parse::<usize>();
        current_total = current_total + units.unwrap();
    }

    return [total_index + 1, total_max];
}

#[cfg(test)]
mod tests {
    use crate::parser::part1_most_calories;

    const TEST_INPUT : &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn finds_most_calories() {
        let actual = part1_most_calories(&String::from(TEST_INPUT));
        let expected : [usize; 2] = [4, 24000];

        println!("{:?} == {:?}", actual, expected);
        assert_eq!(actual.len(), expected.len(), "Arrays don't have the same length");
        assert!(actual.iter().zip(expected.iter()).all(|(a,b)| a == b), "Arrays are not equal");
    }
}
