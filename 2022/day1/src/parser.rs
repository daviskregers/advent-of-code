use indexmap::IndexMap;

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

pub fn part2_top_calories(input: &String) -> [[usize; 2]; 3]
{
    let lines = input.split("\n");
    let mut current_index : usize = 0;
    let mut top = IndexMap::new();

    for item in lines {
        if item.is_empty() {
            current_index += 1;
            continue;
        }
        let units = item.parse::<usize>();
        *top.entry(current_index).or_insert(0) += units.unwrap();
    }

    top.sort_by(|_, av, _, bv| av.cmp(&bv));

    let mut result : [[usize; 2]; 3] = [[0; 2]; 3];
    for i in 0..3 {
        let record = top.pop().unwrap();
        result[i] = [record.0 + 1, record.1];
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::parser::{part1_most_calories, part2_top_calories};

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

    #[test]
    fn finds_top3_calories() {
        let actual = part2_top_calories(&String::from(TEST_INPUT));
        let expected : [[usize; 2]; 3] = [[4, 24000], [3, 11000], [5, 10000]];

        println!("{:?} == {:?}", actual, expected);
        assert_eq!(actual.len(), expected.len(), "Arrays don't have the same length");
        assert!(actual.iter().zip(expected.iter()).all(|(a,b)| a == b), "Arrays are not equal");
    }
}
