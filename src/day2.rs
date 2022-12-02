use crate::io_utils::read_lines;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(&self) -> u32 {
        return match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };
    }

    fn loses_to(&self) -> Shape {
        return match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        };
    }

    fn wins_over(&self) -> Shape {
        return match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        };
    }
}

#[derive(Debug, PartialEq)]
enum GameResult {
    Win,
    Draw,
    Loss,
}

struct Part1Instruction {
    opponent_shape: Shape,
    my_shape: Shape,
}

impl Part1Instruction {
    fn score(&self) -> u32 {
        return self.my_shape.score() + self.outcome_score();
    }

    fn outcome_score(&self) -> u32 {
        return match self.my_shape {
            Shape::Rock => match self.opponent_shape {
                Shape::Rock => 3,
                Shape::Paper => 0,
                Shape::Scissors => 6,
            },
            Shape::Paper => match self.opponent_shape {
                Shape::Rock => 6,
                Shape::Paper => 3,
                Shape::Scissors => 0,
            },
            Shape::Scissors => match self.opponent_shape {
                Shape::Rock => 0,
                Shape::Paper => 6,
                Shape::Scissors => 3,
            },
        };
    }
}

impl FromStr for Part1Instruction {
    type Err = ParseError;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = str.split(" ").collect();

        let opponent_shape: Shape = match split[0] {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            &_ => panic!("DING"),
        };

        let my_shape: Shape = match split[1] {
            "X" => Shape::Rock,
            "Y" => Shape::Paper,
            "Z" => Shape::Scissors,
            &_ => panic!("DONG"),
        };

        return Ok(Part1Instruction {
            opponent_shape,
            my_shape,
        });
    }
}

fn part1(filename: &str) -> u32 {
    let input_lines: Vec<Part1Instruction> = read_lines(filename)
        .unwrap()
        .map(|x| x.unwrap())
        .map(|x| x.parse().unwrap())
        .collect();

    let scores: Vec<u32> = input_lines.into_iter().map(|x| x.score()).collect();

    println!("{:?}", scores);

    return scores.into_iter().sum();
}

struct Part2Instruction {
    result: GameResult,
    opponent_shape: Shape,
}

impl FromStr for Part2Instruction {
    type Err = ParseError;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = str.split(" ").collect();

        let opponent_shape: Shape = match split[0] {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            &_ => panic!("DING"),
        };

        let result: GameResult = match split[1] {
            "X" => GameResult::Loss,
            "Y" => GameResult::Draw,
            "Z" => GameResult::Win,
            &_ => panic!("DONG"),
        };

        return Ok(Part2Instruction {
            opponent_shape,
            result,
        });
    }
}

impl Part2Instruction {
    fn score(&self) -> u32 {
        return match self.result {
            GameResult::Win => 6 + self.opponent_shape.loses_to().score(),
            GameResult::Loss => 0 + self.opponent_shape.wins_over().score(),
            GameResult::Draw => 3 + self.opponent_shape.score(),
        };
    }
}

fn part2(filename: &str) -> u32 {
    let input_lines: Vec<Part2Instruction> = read_lines(filename)
        .unwrap()
        .map(|x| x.unwrap())
        .map(|x| x.parse().unwrap())
        .collect();

    let scores: Vec<u32> = input_lines.into_iter().map(|x| x.score()).collect();

    return scores.into_iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_line_is_correct() {
        let input_line: Part1Instruction = "A Y".parse().unwrap();

        assert_eq!(Shape::Rock, input_line.opponent_shape);
        assert_eq!(Shape::Paper, input_line.my_shape);
    }

    #[test]
    fn part1_example_is_correct() {
        let result = part1("./data/day2_example.txt");

        assert_eq!(15, result);
    }

    #[test]
    fn part1_is_correct() {
        let result = part1("./data/day2.txt");

        assert_eq!(12276, result);
    }

    #[test]
    fn part2_example_is_correct() {
        let result = part2("./data/day2_example.txt");

        assert_eq!(12, result);
    }

    #[test]
    fn part2_is_correct() {
        let result = part2("./data/day2.txt");

        assert_eq!(12, result);
    }
}
