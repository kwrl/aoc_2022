#![allow(dead_code)]

use crate::io_utils::read_lines;
#[derive(Debug)]
struct Vec2 {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Grid {
    content: Vec<u32>,
    width: usize,
}

impl Grid {
    fn get(&self, coordinate: &Vec2) -> Option<u32> {
        return self.get_by_index(coordinate.y * self.width + coordinate.x);
    }

    fn get_by_index(&self, i: usize) -> Option<u32> {
        if i < self.content.len() {
            return Some(self.content[i]);
        }
        return None;
    }

    fn width(&self) -> usize {
        return self.width;
    }

    fn height(&self) -> usize {
        return self.content.len() / self.width;
    }

    fn scenic_score(&self, coordinate: &Vec2) -> usize {
        let value = self.get(coordinate).unwrap();

        let mut up_score = 0;

        for v in self.above(coordinate) {
            up_score += 1;

            if value <= v {
                break;
            }
        }

        let mut right_score = 0;

        for v in self.right_of(coordinate) {
            right_score += 1;

            if value <= v {
                break;
            }
        }

        let mut under_score = 0;

        for v in self.under(coordinate) {
            under_score += 1;

            if value <= v {
                break;
            }
        }

        let mut left_score = 0;

        for v in self.left_of(coordinate) {
            left_score += 1;

            if value <= v {
                break;
            }
        }

        return up_score * right_score * under_score * left_score;
    }

    fn is_visible(&self, coordinate: &Vec2) -> bool {
        let value = self.get(coordinate).unwrap();

        if self.above(coordinate).iter().all(|x| x < &value) {
            return true;
        }

        if self.right_of(coordinate).iter().all(|x| x < &value) {
            return true;
        }

        if self.under(coordinate).iter().all(|x| x < &value) {
            return true;
        }

        if self.left_of(coordinate).iter().all(|x| x < &value) {
            return true;
        }

        return false;
    }

    fn left_of(&self, coordinate: &Vec2) -> Vec<u32> {
        let mut values = vec![];

        for x in (0..coordinate.x).rev() {
            values.push(self.get(&Vec2 { x, y: coordinate.y }).unwrap());
        }

        return values;
    }

    fn right_of(&self, coordinate: &Vec2) -> Vec<u32> {
        let mut values = vec![];

        for x in (coordinate.x + 1)..self.width() {
            values.push(self.get(&Vec2 { x, y: coordinate.y }).unwrap());
        }

        return values;
    }

    fn under(&self, coordinate: &Vec2) -> Vec<u32> {
        let mut values = vec![];

        for y in (coordinate.y + 1)..self.height() {
            values.push(self.get(&Vec2 { x: coordinate.x, y }).unwrap());
        }

        return values;
    }

    fn above(&self, coordinate: &Vec2) -> Vec<u32> {
        let mut values = vec![];

        for y in (0..coordinate.y).rev() {
            values.push(self.get(&Vec2 { x: coordinate.x, y }).unwrap());
        }

        return values;
    }

    fn coordinates(&self) -> Vec<Vec2> {
        let mut coordinates = vec![];
        for y in 0..self.height() {
            for x in 0..self.width() {
                coordinates.push(Vec2 { x, y });
            }
        }

        return coordinates;
    }
}

fn part1(filename: &str) -> usize {
    let lines: Vec<String> = read_lines(filename).unwrap().map(|x| x.unwrap()).collect();
    let width = lines[0].len();

    let numbers: Vec<u32> = lines
        .iter()
        .flat_map(|line| line.chars())
        .map(|x| x.to_digit(10).unwrap())
        .collect();

    let grid = Grid {
        content: numbers,
        width,
    };

    let visible_count = grid
        .coordinates()
        .iter()
        .filter(|x| grid.is_visible(x))
        .count();

    return visible_count;
}

fn part2(filename: &str) -> usize {
    let lines: Vec<String> = read_lines(filename).unwrap().map(|x| x.unwrap()).collect();
    let width = lines[0].len();

    let numbers: Vec<u32> = lines
        .iter()
        .flat_map(|line| line.chars())
        .map(|x| x.to_digit(10).unwrap())
        .collect();

    let grid = Grid {
        content: numbers,
        width,
    };

    let largest_scenic_score = grid
        .coordinates()
        .iter()
        .map(|x| grid.scenic_score(x))
        .max()
        .unwrap();

    return largest_scenic_score;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_is_correct() {
        let result = part1("./data/day8_example.txt");

        assert_eq!(21, result);
    }

    #[test]
    fn part1_is_correct() {
        let result = part1("./data/day8.txt");

        assert_eq!(1672, result);
    }

    #[test]
    fn part2_example_is_correct() {
        let result = part2("./data/day8_example.txt");

        assert_eq!(8, result);
    }

    #[test]
    fn part2_is_correct() {
        let result = part2("./data/day8.txt");

        assert_eq!(327180, result);
    }
}
