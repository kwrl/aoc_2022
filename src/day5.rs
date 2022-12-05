use crate::io_utils::read_lines;
use std::str::FromStr;

#[derive(Debug)]
struct CargoBay {
    stacks: Vec<Vec<char>>,
}

impl CargoBay {
    pub fn move_item(&mut self, from: usize, to: usize) {
        let item = self.stacks[from].pop().unwrap();

        self.stacks[to].push(item);
    }

    pub fn handle_move(&mut self, cargo_move: &CargoMove) {
        for i in 0..cargo_move.quantity {
            self.move_item(cargo_move.from - 1, cargo_move.to - 1);
        }
    }

    pub fn handle_move_part2(&mut self, cargo_move: &CargoMove) {
        let mut items: Vec<char> = vec![];

        for _i in 0..cargo_move.quantity {
            items.push(self.stacks[cargo_move.from - 1].pop().unwrap());
        }

        for item in items.into_iter().rev() {
            self.stacks[cargo_move.to - 1].push(item);
        }
    }

    fn to_str(&self) -> String {
        let mut str = String::new();

        let cloned_stacks = self.stacks.clone();

        for stack in cloned_stacks {
            if stack.len() != 0 {
                str.push(stack.last().unwrap().clone());
            }
        }

        return str;
    }
}

#[derive(Debug)]
struct CargoBayParseError;

impl FromStr for CargoBay {
    type Err = CargoBayParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut stacks: Vec<Vec<char>> = vec![vec![]; 10];

        for line in str.lines().rev() {
            if !line.trim().starts_with("[") {
                continue;
            }

            for stack_index in 0..(line.len() / 4) {
                let stack = &mut stacks[stack_index];

                let value: Option<char> = line[stack_index * 4..((stack_index * 4) + 4)]
                    .trim()
                    .strip_prefix("[")
                    .and_then(|x| x.strip_suffix("]"))
                    .and_then(|x| x.chars().nth(0));

                match value {
                    Some(some_value) => stack.push(some_value),
                    None => {}
                }
            }
        }

        return Ok(CargoBay { stacks });
    }
}

#[derive(Debug)]
struct CargoMove {
    quantity: usize,
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct CargoMoveParseError;
impl FromStr for CargoMove {
    type Err = CargoMoveParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<usize> = str
            .split(" ")
            .map(|x| x.parse())
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .collect();

        return Ok(CargoMove {
            quantity: numbers[0],
            from: numbers[1],
            to: numbers[2],
        });
    }
}

fn part1(filename: &str) -> String {
    let lines: Vec<String> = read_lines(filename).unwrap().map(|x| x.unwrap()).collect();
    let content = lines.join("\n");

    let split: Vec<&str> = content.split("\n\n").collect();

    let mut cargo_bay: CargoBay = split[0].parse().unwrap();

    println!("{:?}", cargo_bay);

    let cargo_moves: Vec<CargoMove> = split[1]
        .split("\n")
        .map(|x| x.parse())
        .map(|x| x.unwrap())
        .collect();

    for cargo_move in cargo_moves {
        cargo_bay.handle_move(&cargo_move);
        println!("DONE {:?}", cargo_move);
    }

    println!("{:?}", cargo_bay);

    return cargo_bay.to_str();
}

fn part2(filename: &str) -> String {
    let lines: Vec<String> = read_lines(filename).unwrap().map(|x| x.unwrap()).collect();
    let content = lines.join("\n");

    let split: Vec<&str> = content.split("\n\n").collect();

    let mut cargo_bay: CargoBay = split[0].parse().unwrap();

    println!("{:?}", cargo_bay);

    let cargo_moves: Vec<CargoMove> = split[1]
        .split("\n")
        .map(|x| x.parse())
        .map(|x| x.unwrap())
        .collect();

    for cargo_move in cargo_moves {
        println!("START {:?}", cargo_move);
        cargo_bay.handle_move_part2(&cargo_move);
        println!("DONE {:?}", cargo_move);
    }

    println!("{:?}", cargo_bay);

    return cargo_bay.to_str();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_move_is_correct() {
        let cargo_move: CargoMove = "move 1 from 2 to 1".parse().unwrap();

        assert_eq!(1, cargo_move.quantity);
        assert_eq!(2, cargo_move.from);
        assert_eq!(1, cargo_move.to);
    }

    #[test]
    fn part1_example_is_correct() {
        let result = part1("./data/day5_example.txt");

        assert_eq!("CMZ", result);
    }

    #[test]
    fn part1_is_correct() {
        let result = part1("./data/day5.txt");

        assert_eq!("RTGWZTHLD", result);
    }

    #[test]
    fn part2_example_is_correct() {
        let result = part2("./data/day5_example.txt");

        assert_eq!("MCD", result);
    }

    #[test]
    fn part2_is_correct() {
        let result = part2("./data/day5.txt");

        assert_eq!("STHGRZZFR", result);
    }
}
