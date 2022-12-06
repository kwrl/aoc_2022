use itertools::Itertools;

fn find_start_of_packet_marker(signal: &str) -> Option<usize> {
    for i in 3..signal.len() {
        if signal[i - 3..(i + 1)].chars().unique().count() == 4 {
            return Some(i);
        }
    }

    return None;
}

fn find_start_of_message_marker(signal: &str) -> Option<usize> {
    for i in 13..signal.len() {
        if signal[i - 13..(i + 1)].chars().unique().count() == 14 {
            return Some(i);
        }
    }

    return None;
}

fn part1(signal: &str) -> usize {
    return match find_start_of_packet_marker(signal) {
        Some(marker) => marker + 1,
        None => panic!("We dorked it all up!"),
    };
}

fn part2(signal: &str) -> usize {
    return match find_start_of_message_marker(signal) {
        Some(marker) => marker + 1,
        None => panic!("We dorked it all up!"),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io_utils::read_lines;

    #[test]
    fn part1_example_is_correct() {
        assert_eq!(7, part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(5, part1("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, part1("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(10, part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(11, part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn part1_is_correct() {
        let signal = read_lines("./data/day6.txt")
            .unwrap()
            .map(|x| x.unwrap())
            .nth(0)
            .unwrap();

        assert_eq!(1876, part1(&signal[..]));
    }

    #[test]
    fn part2_example_is_correct() {
        assert_eq!(19, part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(23, part2("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(23, part2("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(29, part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(26, part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn part2_is_correct() {
        let signal = read_lines("./data/day6.txt")
            .unwrap()
            .map(|x| x.unwrap())
            .nth(0)
            .unwrap();

        assert_eq!(2202, part2(&signal[..]));
    }
}
