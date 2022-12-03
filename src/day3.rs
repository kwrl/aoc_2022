use crate::io_utils::read_lines;
use std::collections::HashSet;
use std::str::FromStr;

struct Rucksack {
    items: String,
}

impl Rucksack {
    fn first_compartment(&self) -> String {
        return String::from(&self.items[0..self.items.len() / 2]);
    }

    fn second_compartment(&self) -> String {
        return String::from(&self.items[self.items.len() / 2..self.items.len()]);
    }

    fn items_in_both_compartments(&self) -> HashSet<char> {
        let first_compartment = self.first_compartment();
        let second_compartment = self.second_compartment();

        let mut items_in_both = HashSet::new();

        for c in self.items.chars() {
            if first_compartment.contains(c) && second_compartment.contains(c) {
                items_in_both.insert(c);
            }
        }

        return items_in_both;
    }
}
struct RucksackParseError {}
impl FromStr for Rucksack {
    type Err = RucksackParseError;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        return Ok(Rucksack {
            items: String::from(str),
        });
    }
}

fn priority(value: char) -> u32 {
    let integer = value as u32;

    if integer >= 65 && integer <= 90 {
        return integer - 64 + 26;
    }

    if integer >= 97 && integer <= 122 {
        return integer - 96;
    }

    panic!("crap");
}

fn part1(filename: &str) -> u32 {
    let rucksacks: Vec<Rucksack> = read_lines(filename)
        .unwrap()
        .map(|x| x.unwrap())
        .map(|x| Rucksack { items: x })
        .collect();

    let mut sum = 0;
    for rucksack in rucksacks {
        for c in rucksack.items_in_both_compartments() {
            sum += priority(c);
        }
    }

    return sum;
}

fn intersection(a: String, b: String, c: String) -> HashSet<char> {
    let mut intersection = HashSet::new();

    for ch in format!("{}{}{}", a, b, c).chars() {
        if a.contains(ch) && b.contains(ch) & c.contains(ch) {
            intersection.insert(ch);
        }
    }

    return intersection;
}

fn part2(filename: &str) -> u32 {
    let rucksacks: Vec<Rucksack> = read_lines(filename)
        .unwrap()
        .map(|x| x.unwrap())
        .map(|x| Rucksack { items: x })
        .collect();

    let mut sum = 0;
    for group in 0..(rucksacks.len() / 3) {
        let group_start = group * 3;

        let intersection = intersection(
            rucksacks[group_start].items.clone(),
            rucksacks[group_start + 1].items.clone(),
            rucksacks[group_start + 2].items.clone(),
        );

        for ch in intersection {
            sum += priority(ch);
        }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rucksack_compartments_are_correct() {
        let under_test = Rucksack {
            items: String::from("EPLEKAKE"),
        };

        assert_eq!("EPLE", under_test.first_compartment());
        assert_eq!("KAKE", under_test.second_compartment());
    }

    #[test]
    fn items_in_both_compartments_is_correct() {
        let under_test = Rucksack {
            items: String::from("EPLEKAKE"),
        };

        let mut expected = HashSet::new();
        expected.insert('E');

        assert_eq!(expected, under_test.items_in_both_compartments());
    }

    #[test]
    fn priority_is_correct() {
        println!("{}", 'a' as u32);
        assert_eq!(1, priority('a'));
        assert_eq!(26, priority('z'));
        assert_eq!(27, priority('A'));
        assert_eq!(52, priority('Z'));
    }

    #[test]
    fn part1_example_is_correct() {
        assert_eq!(157, part1("./data/day3_example.txt"));
    }

    #[test]
    fn part1_is_correct() {
        assert_eq!(8053, part1("./data/day3.txt"));
    }

    #[test]
    fn part2_example_is_correct() {
        assert_eq!(70, part2("./data/day3_example.txt"));
    }

    #[test]
    fn part2_is_correct() {
        assert_eq!(2425, part2("./data/day3.txt"));
    }
}
