use crate::io_utils::read_lines;
use std::collections::HashSet;

#[derive(Clone, Debug)]
struct ComputerState {
    x: i32,
}

struct Computer {
    states: Vec<ComputerState>,
}

impl Computer {
    fn signal_strength_at(&self, cycle: usize) -> i32 {
        let x: i32 = self.states[cycle - 1].x;
        return cycle as i32 * x;
    }
}

trait Instruction {
    fn execute(&self, computer: &mut Computer);
}

#[derive(Debug)]
struct AddX {
    increment: i32,
}

impl Instruction for AddX {
    fn execute(&self, computer: &mut Computer) {
        let x = computer.states.last().unwrap().x;

        computer
            .states
            .push(computer.states.last().unwrap().clone());

        computer.states.push(ComputerState {
            x: x + self.increment,
        });
    }
}

#[derive(Debug)]
struct Noop {}

fn parse_instruction(s: &str) -> Box<dyn Instruction> {
    let split: Vec<&str> = s.split(" ").collect();
    return match split[0] {
        "addx" => Box::new(AddX {
            increment: split[1].parse().unwrap(),
        }),
        "noop" => Box::new(Noop {}),
        _ => panic!("Cringe"),
    };
}

impl Instruction for Noop {
    fn execute(&self, computer: &mut Computer) {
        computer
            .states
            .push(computer.states.last().unwrap().clone());
    }
}

struct CRTScreen {
    light: HashSet<(u32, u32)>,
    height: u32,
    width: u32,
}

impl CRTScreen {
    fn to_string(&self) -> String {
        let mut repr = String::from("");
        for y in 0..self.height {
            let mut line_repr = String::from("");
            for x in 0..self.width {
                if self.light.contains(&(x, y)) {
                    line_repr += "#";
                } else {
                    line_repr += ".";
                }
            }

            repr.push_str(&line_repr);
            repr.push_str("\n");
        }

        return repr;
    }

    fn draw_pixel(&mut self, x: u32, y: u32) {
        self.light.insert((x, y));
    }

    fn draw(&mut self, computer: &Computer) {
        for cycle in 0..computer.states.len() {
            let pixel_x = (cycle as u32) % self.width;
            let pixel_y = ((cycle as u32) / self.width) % self.height;

            let sprite_x = computer.states[cycle].x;

            if (pixel_x as i32 - sprite_x).abs() <= 1 {
                self.draw_pixel(pixel_x, pixel_y);
            }
        }
    }
}

fn part1(filename: &str) -> i32 {
    let lines: Vec<String> = read_lines(filename).unwrap().map(|x| x.unwrap()).collect();

    let instructions: Vec<Box<dyn Instruction>> =
        lines.iter().map(|x| parse_instruction(x)).collect();

    let mut computer: Computer = Computer {
        states: vec![ComputerState { x: 1 }],
    };

    for instruction in instructions {
        instruction.execute(&mut computer);
    }

    return computer.signal_strength_at(20)
        + computer.signal_strength_at(60)
        + computer.signal_strength_at(100)
        + computer.signal_strength_at(140)
        + computer.signal_strength_at(180)
        + computer.signal_strength_at(220);
}

fn part2(filename: &str) -> String {
    let lines: Vec<String> = read_lines(filename).unwrap().map(|x| x.unwrap()).collect();

    let instructions: Vec<Box<dyn Instruction>> =
        lines.iter().map(|x| parse_instruction(x)).collect();

    let mut computer: Computer = Computer {
        states: vec![ComputerState { x: 1 }],
    };

    for instruction in instructions {
        instruction.execute(&mut computer);
    }

    let mut screen = CRTScreen {
        height: 6,
        width: 40,
        light: HashSet::new(),
    };

    screen.draw(&computer);

    return screen.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case_is_correct() {
        let mut computer: Computer = Computer {
            states: vec![ComputerState { x: 1 }],
        };

        let instruction = parse_instruction("addx -15");

        instruction.execute(&mut computer);

        assert_eq!(3, computer.states.len());
        assert_eq!(-14, computer.states.last().unwrap().x);
    }

    #[test]
    fn part1_example_is_correct() {
        let result = part1("./data/day10_example.txt");

        assert_eq!(13140, result);
    }

    #[test]
    fn part1_is_correct() {
        let result = part1("./data/day10.txt");

        assert_eq!(13520, result);
    }

    #[test]
    fn part2_example_is_correct() {
        let result = part2("./data/day10_example.txt");
        println!("{:?}", result);

        let expected = "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....\n";

        assert_eq!(expected, result);
    }

    #[test]
    fn part2_is_correct() {
        let result = part2("./data/day10.txt");
        let expected = "###...##..###..#..#.###..####..##..###..\n#..#.#..#.#..#.#..#.#..#.#....#..#.#..#.\n#..#.#....#..#.####.###..###..#..#.###..\n###..#.##.###..#..#.#..#.#....####.#..#.\n#....#..#.#....#..#.#..#.#....#..#.#..#.\n#.....###.#....#..#.###..####.#..#.###..\n";

        assert_eq!(expected, result);
    }
}
