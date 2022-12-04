use core::fmt::Debug;

pub struct Parser
{
    moves: Vec<Move>
}

impl Parser {
    pub fn from_part1(input: &String) -> Self
    {
        let mut moves : Vec<Move> = vec![];
        let lines = input.split("\n");

        for movement in lines {
            let sides : Vec<&str> = movement.split(" ").collect::<Vec<&str>>();
            if sides.len() != 2 {
                println!("Line: {:?} doesn't have 2 columns!", movement);
                continue;
            }
            moves.push(Move::from_string(sides[0], sides[1]));
        }

        Parser {
            moves
        }
    }

    pub fn from_part2(input: &String) -> Self
    {
        let mut moves : Vec<Move> = vec![];
        let lines = input.split("\n");

        for movement in lines {
            let sides : Vec<&str> = movement.split(" ").collect::<Vec<&str>>();
            if sides.len() != 2 {
                println!("Line: {:?} doesn't have 2 columns!", movement);
                continue;
            }

            let m : Move = match sides[0] {
                "A" => match sides[1] {
                    "X" => Move::new(Box::new(Rock::new()), Box::new(Scissors::new())),
                    "Y" => Move::new(Box::new(Rock::new()), Box::new(Rock::new())),
                    "Z" => Move::new(Box::new(Rock::new()), Box::new(Paper::new())),
                    _ => panic!("Unknown rock move!"),
                },
                "B" => match sides[1] {
                    "X" => Move::new(Box::new(Paper::new()), Box::new(Rock::new())),
                    "Y" => Move::new(Box::new(Paper::new()), Box::new(Paper::new())),
                    "Z" => Move::new(Box::new(Paper::new()), Box::new(Scissors::new())),
                    _ => panic!("Unknown paper move!"),
                },
                "C" => match sides[1] {
                    "X" => Move::new(Box::new(Scissors::new()), Box::new(Paper::new())),
                    "Y" => Move::new(Box::new(Scissors::new()), Box::new(Scissors::new())),
                    "Z" => Move::new(Box::new(Scissors::new()), Box::new(Rock::new())),
                    _ => panic!("Unknown scissors move!"),
                },
                _ => panic!("Unknown opponent move!"),
            };

            moves.push(m);
        }

        Parser {
            moves
        }
    }

    fn get_moves(&self) -> &Vec<Move>
    {
        return &self.moves;
    }

    pub fn get_move_scores(&self) -> Vec<usize>
    {
        let mut result : Vec<usize> = vec![];

        for movement in self.get_moves() {
            result.push(movement.me.move_points());
            result.push(movement.me.win_points(movement.opponent.as_string()));
        }

        return result;
    }

    pub fn get_score_total(&self) -> usize {
        self.get_move_scores().iter().sum()
    }
}

#[derive(Debug)]
struct Move
{
    opponent: Box<dyn MoveType>,
    me: Box<dyn MoveType>,
}

impl Move {
    fn new(opponent: Box<dyn MoveType>, me: Box<dyn MoveType>) -> Self { Self { opponent, me } }
    fn from_string(opponent: &str, me: &str) -> Self {
        let o : Box<dyn MoveType> = match opponent {
            "A" => Box::new(Rock::new()),
            "B" => Box::new(Paper::new()),
            "C" => Box::new(Scissors::new()),
            _ => panic!("Unknown my move type!"),
        };
        let m : Box<dyn MoveType> = match me {
            "X" => Box::new(Rock::new()),
            "Y" => Box::new(Paper::new()),
            "Z" => Box::new(Scissors::new()),
            _ => panic!("Unknown my move type!"),
        };

        Move {opponent: o, me: m}
    }
}

trait MoveType {
    fn as_string(&self) -> &str;
    fn win_points(&self, other: &str) -> usize;
    fn move_points(&self) -> usize;
}

impl Debug for dyn MoveType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "MoveType{{{}}}", self.as_string())
    }
}

struct Paper {}

impl Paper {
    fn new() -> Self { Self {  } }
}
impl MoveType for Paper {
    fn as_string(&self) -> &str {
        "Paper"
    }

    fn win_points(&self, other: &str) -> usize {
        match other {
            "Scissors" => 0,
            "Paper" => 3,
            "Rock" => 6,
            _ => panic!("Unknown opponent type for Paper!"),
        }
    }

    fn move_points(&self) -> usize {
        2
    }
}

struct Rock {}

impl Rock {
    fn new() -> Self { Self {  } }
}

impl MoveType for Rock {
    fn as_string(&self) -> &str {
        "Rock"
    }
    fn win_points(&self, other: &str) -> usize {
        match other {
            "Paper" => 0,
            "Rock" => 3,
            "Scissors" => 6,
            _ => panic!("Unknown opponent type for Rock!"),
        }
    }

    fn move_points(&self) -> usize {
        1
    }
}

struct Scissors {}

impl Scissors {
    fn new() -> Self { Self {  } }
}
impl MoveType for Scissors {
    fn as_string(&self) -> &str {
        "Scissors"
    }
    fn win_points(&self, other: &str) -> usize {
        match other {
            "Rock" => 0,
            "Scissors" => 3,
            "Paper" => 6,
            _ => panic!("Unknown opponent type for Scissors!"),
        }
    }

    fn move_points(&self) -> usize {
        3
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{Parser, Move, Rock, Paper, Scissors};
    const TEST_INPUT : &str = "A Y
B X
C Z";

    #[test]
    fn test_constructs_moves() {
        let parser = Parser::from_part1(&String::from(TEST_INPUT));
        let actual : &Vec<Move> = parser.get_moves();
        let expected : Vec<Move> = vec![
            Move::new(Box::new(Rock::new()), Box::new(Paper::new())),
            Move::new(Box::new(Paper::new()), Box::new(Rock::new())),
            Move::new(Box::new(Scissors::new()), Box::new(Scissors::new())),
        ];

        assert_eq!(actual.len(), expected.len(), "Arrays don't have the same length");
        assert!(actual.iter().zip(expected.iter()).all(|(a,b)| {
            a.opponent.as_string() == b.opponent.as_string() &&
            a.me.as_string() == b.me.as_string()
        }), "Arrays are not equal");
    }

    #[test]
    fn test_move_scores() {
        let parser = Parser::from_part1(&String::from(TEST_INPUT));
        let actual : Vec<usize> = parser.get_move_scores();
        let expected : Vec<usize> = vec![2, 6, 1, 0, 3, 3];

        assert_eq!(actual.len(), expected.len(), "Arrays don't have the same length");
        assert!(actual.iter().zip(expected.iter()).all(|(a,b)| a == b), "Arrays are not equal");
    }

    #[test]
    fn test_score_total() {
        let parser = Parser::from_part1(&String::from(TEST_INPUT));
        let actual : usize = parser.get_score_total();
        let expected : usize = 15;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_moves() {
        let parser = Parser::from_part2(&String::from(TEST_INPUT));
        let actual : Vec<usize> = parser.get_move_scores();
        let expected : Vec<usize> = vec![1, 3, 1, 0, 1, 6];

        println!("ACTUAL: {:?}", actual);
        println!("EXP: {:?}", expected);

        assert_eq!(actual.len(), expected.len(), "Arrays don't have the same length");
        assert!(actual.iter().zip(expected.iter()).all(|(a,b)| a == b), "Arrays are not equal");
    }
}
