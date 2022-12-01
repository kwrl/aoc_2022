use crate::io_utils::read_lines;

fn sums(lines: &Vec<String>) -> Vec<u32> {
    let mut sums = vec![];

    let mut i = 0;
    while i < lines.len() {
        let mut sum: u32 = 0;

        while i < lines.len() && !lines[i].is_empty() {
            let number: u32 = lines[i].parse().unwrap();
            sum += number;

            i += 1;
        }

        sums.push(sum);
        i += 1;
    }

    return sums;
}

fn part1(filename: &str) -> u32 {
    let lines: Vec<String> = read_lines(filename)
        .unwrap()
        .into_iter()
        .map(|x| x.unwrap())
        .collect();

    let sums = sums(&lines);

    return sums.into_iter().max().unwrap();
}

fn part2(filename: &str) -> u32 {
    let lines: Vec<String> = read_lines(filename)
        .unwrap()
        .into_iter()
        .map(|x| x.unwrap())
        .collect();

    let mut sums = sums(&lines);
    sums.sort();

    let largest_to_smallest: Vec<u32> = sums.into_iter().rev().collect();

    return largest_to_smallest[0] + largest_to_smallest[1] + largest_to_smallest[2];
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_example_is_correct() {
        let result = part1("data/day1_example.txt");
        assert_eq!(24_000, result);
    }

    #[test]
    fn part1_is_correct() {
        let result = part1("data/day1.txt");
        assert_eq!(66_616, result);
    }

    #[test]
    fn part2_example_is_correct() {
        let result = part2("data/day1_example.txt");
        assert_eq!(45_000, result);
    }

    #[test]
    fn part2_is_correct() {
        let result = part2("data/day1.txt");
        assert_eq!(199172, result);
    }
}
