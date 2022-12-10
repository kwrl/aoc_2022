use crate::io_utils::read_lines;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Vec2 {
    x: i32,
    y: i32,
}

struct Knot<'a> {
    position: Vec2,
    next: &'a mut Option<&'a mut Knot<'a>>,
}

impl<'a> Knot<'a> {
    fn tail(&mut self) -> &Knot<'a> {
        return match self.next {
            Some(n) => n.tail(),
            None => self,
        };
    }

    fn step(&mut self, step: &Vec2) {
        self.position = self.position.plus(step);

        match self.next {
            Some(n) => {
                let difference = self.position.minus(&n.position);

                if difference.x.abs() >= 2 || difference.y.abs() >= 2 {
                    let direction = &difference.direction();

                    n.step(direction);
                }
            }
            None => {}
        }
    }
}

static UP: Vec2 = Vec2 { x: 0, y: -1 };
static RIGHT: Vec2 = Vec2 { x: 1, y: 0 };
static DOWN: Vec2 = Vec2 { x: 0, y: 1 };
static LEFT: Vec2 = Vec2 { x: -1, y: 0 };

impl Vec2 {
    fn plus(&self, other: &Vec2) -> Vec2 {
        return Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }

    fn minus(&self, other: &Vec2) -> Vec2 {
        return Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }

    fn scale(&self, scale: i32) -> Vec2 {
        return Vec2 {
            x: self.x * scale,
            y: self.y * scale,
        };
    }

    fn direction(&self) -> Vec2 {
        let mut x = 0;

        if self.x != 0 {
            x = self.x / self.x.abs();
        }

        let mut y = 0;

        if self.y != 0 {
            y = self.y / self.y.abs();
        }

        return Vec2 { x, y };
    }
}

fn parse_steps(str: &str) -> Vec<Vec2> {
    let split: Vec<&str> = str.split(" ").collect();
    let direction = split[0];
    let scale: i32 = split[1].parse().unwrap();

    let step = match direction {
        "U" => UP,
        "R" => RIGHT,
        "D" => DOWN,
        "L" => LEFT,
        _ => panic!("Skrrrrt"),
    };

    let mut steps = vec![];

    for _ in 0..scale {
        steps.push(step.clone())
    }

    return steps;
}

fn next_position(head: &Vec2, tail: &Vec2) -> Vec2 {
    let difference = head.minus(tail);

    if difference.x.abs() >= 2 || difference.y.abs() >= 2 {
        let step = difference.direction();

        return tail.plus(&step);
    }

    return tail.clone();
}

fn part1(filename: &str) -> usize {
    let steps: Vec<Vec2> = read_lines(filename)
        .unwrap()
        .map(|x| x.unwrap())
        .flat_map(|x| parse_steps(&x[..]))
        .collect();

    let mut head_positions = vec![Vec2 { x: 0, y: 0 }];
    let mut tail_positions = vec![Vec2 { x: 0, y: 0 }];

    for head_step in steps {
        let head_position = head_positions.last().unwrap();
        let new_head_position = head_position.plus(&head_step);
        head_positions.push(new_head_position);

        let tail_position = tail_positions.last().unwrap();

        tail_positions.push(next_position(&new_head_position, tail_position));
    }

    return tail_positions.iter().unique().count();
}

fn part2(filename: &str) -> usize {
    let steps: Vec<Vec2> = read_lines(filename)
        .unwrap()
        .map(|x| x.unwrap())
        .flat_map(|x| parse_steps(&x[..]))
        .collect();

    let mut rope: Vec<Vec<Vec2>> = (0..10).map(|i| vec![Vec2 { x: 0, y: 0 }]).collect();

    for head_step in steps {
        // Updates the head
        let head_position = rope[0].last().unwrap();
        let new_head_position = head_position.plus(&head_step);
        rope[0].push(new_head_position);

        for i in 1..rope.len() {
            let previous_knot_position = rope[i - 1].last().unwrap();
            let current_knot_position = rope[i].last().unwrap();

            let new_current_knot_position =
                next_position(previous_knot_position, current_knot_position);

            //if &new_current_knot_position != current_knot_position {
            rope[i].push(new_current_knot_position);
            //}
        }
    }

    return rope.last().unwrap().iter().unique().count();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_right_move_is_correct() {
        let result = parse_steps("R 4");

        assert_eq!(vec![RIGHT, RIGHT, RIGHT, RIGHT], result);
    }

    #[test]
    fn parse_left_move_is_correct() {
        let result = parse_steps("L 5");

        assert_eq!(vec![LEFT, LEFT, LEFT, LEFT, LEFT], result);
    }

    #[test]
    fn parse_up_move_is_correct() {
        let result = parse_steps("U 2");

        assert_eq!(vec![UP, UP], result);
    }

    #[test]
    fn parse_down_move_is_correct() {
        let result = parse_steps("D 7");

        assert_eq!(vec![DOWN, DOWN, DOWN, DOWN, DOWN, DOWN, DOWN], result);
    }

    #[test]
    fn part1_example_is_correct() {
        let result = part1("./data/day9_example.txt");

        assert_eq!(13, result);
    }

    #[test]
    fn part1_is_correct() {
        let result = part1("./data/day9.txt");

        assert_eq!(6314, result);
    }

    #[test]
    fn part2_example_is_correct() {
        let result = part2("./data/day9_example.txt");

        assert_eq!(1, result);
    }

    #[test]
    fn part2_is_correct() {
        let result = part2("./data/day9.txt");

        assert_eq!(2504, result);
    }
}
