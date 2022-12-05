use core::str::FromStr;

pub fn get_pairs(input: &str) -> Vec<[Vec<usize>; 2]>
{
    input.split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| {
            let ranges : Vec<&str> = x.split(",").collect::<Vec<&str>>();
            let mut pair : [Vec<usize>; 2] = [vec![], vec![]];

            for i in 0..2 {
                let range = ranges.get(i).unwrap().split("-");
                let start : usize = FromStr::from_str(range.clone().nth(0).unwrap()).expect("Expected unsingned integer!");
                let end : usize = FromStr::from_str(range.clone().last().unwrap()).expect("Expected unsigned integer!");
                for j in start..end+1 {
                    pair[i].push(j);
                }
            }

            pair
        }).collect::<Vec<[Vec<usize>; 2]>>()
}

pub fn get_overlap(input: Vec<[Vec<usize>; 2]>) -> usize
{
    input
        .iter()
        .map(|x| {
            overlaps(x.get(0).unwrap(), x.get(1).unwrap()) ||
            overlaps(x.get(1).unwrap(), x.get(0).unwrap())
        })
        .filter(|x| *x == true)
        .collect::<Vec<bool>>()
        .len()
}

fn overlaps(v1 : &Vec<usize>, v2 : &Vec<usize>) -> bool
{
    for i in v1 {
        if !v2.contains(i) {
            return false
        }
    }
    return true
}

#[cfg(test)]
mod tests {
    use crate::parser::*;

    const TEST_INPUT : &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn test_parses_pairs() {
        let binding = String::from(TEST_INPUT);
        let actual = get_pairs(&binding);
        let expected : Vec<[Vec<usize>;2]> = vec![
            [vec![2,3,4], vec![6,7,8]],
            [vec![2,3], vec![4,5]],
            [vec![5,6,7], vec![7,8,9]],
            [vec![2,3,4,5,6,7,8], vec![3,4,5,6,7]],
            [vec![6], vec![4,5,6]],
            [vec![2,3,4,5,6], vec![4,5,6,7,8]]
        ];

        println!("{:?} == {:?}", actual, expected);
        assert_eq!(actual.len(), expected.len(), "Arrays don't have the same length");
        assert!(actual.iter().zip(expected.iter()).all(|(a,b)| a == b), "Arrays are not equal");
    }

    #[test]
    fn test_get_overlap() {
        let input : Vec<[Vec<usize>; 2]> = vec![
            [vec![2,3,4], vec![6,7,8]],
            [vec![2,3], vec![4,5]],
            [vec![5,6,7], vec![7,8,9]],
            [vec![2,3,4,5,6,7,8], vec![3,4,5,6,7]],
            [vec![6], vec![4,5,6]],
            [vec![2,3,4,5,6], vec![4,5,6,7,8]]
        ];
        let actual = get_overlap(input);
        let expected : usize = 2;

        assert_eq!(actual, expected);
    }
}
