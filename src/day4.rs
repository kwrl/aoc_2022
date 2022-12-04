use crate::io_utils::read_lines;
use std::cmp;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        return self.start <= other.start && self.end >= other.end;
    }

    fn overlaps(&self, other: &Range) -> bool {
        return cmp::max(self.start, other.start) <= cmp::min(self.end, other.end);
    }
}

struct InputLine {
    first_range: Range,
    second_range: Range,
}

#[derive(Debug)]
struct InputLineParseError {}
impl FromStr for InputLine {
    type Err = InputLineParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = str.trim().split(",").collect();

        return Ok(InputLine {
            first_range: split[0].parse().unwrap(),
            second_range: split[1].parse().unwrap(),
        });
    }
}

#[derive(Debug)]
struct RangeParseError {}
impl FromStr for Range {
    type Err = RangeParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = str.trim().split("-").collect();

        let start: u32 = split[0].parse().unwrap();
        let end: u32 = split[1].parse().unwrap();

        return Ok(Range { start, end });
    }
}

fn part1(filename: &str) -> u32 {
    let input_lines: Vec<InputLine> = read_lines(filename)
        .unwrap()
        .map(|x| x.unwrap())
        .map(|x| x.parse().unwrap())
        .collect();

    let contain_count: usize = input_lines
        .into_iter()
        .filter(|x| {
            x.first_range.contains(&x.second_range) || x.second_range.contains(&x.first_range)
        })
        .count();

    return contain_count as u32;
}

fn part2(filename: &str) -> u32 {
    let input_lines: Vec<InputLine> = read_lines(filename)
        .unwrap()
        .map(|x| x.unwrap())
        .map(|x| x.parse().unwrap())
        .collect();

    let overlap_count: usize = input_lines
        .into_iter()
        .filter(|x| x.first_range.overlaps(&x.second_range))
        .count();

    return overlap_count as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_parsing_is_correct() {
        let range: Range = "2-4".parse().unwrap();

        assert_eq!(Range { start: 2, end: 4 }, range);
    }

    #[test]
    fn input_line_parsing_is_correct() {
        let line: InputLine = "2-4, 6-8".parse().unwrap();

        assert_eq!(Range { start: 2, end: 4 }, line.first_range);
        assert_eq!(Range { start: 6, end: 8 }, line.second_range);
    }

    #[test]
    fn part1_example_is_correct() {
        let result = part1("./data/day4_example.txt");

        assert_eq!(2, result);
    }

    #[test]
    fn part1_is_correct() {
        let result = part1("./data/day4.txt");

        assert_eq!(588, result);
    }

    #[test]
    fn part2_example_is_correct() {
        let result = part2("./data/day4_example.txt");

        assert_eq!(4, result);
    }

    #[test]
    fn part2_is_correct() {
        let result = part2("./data/day4.txt");

        assert_eq!(911, result);
    }
}
