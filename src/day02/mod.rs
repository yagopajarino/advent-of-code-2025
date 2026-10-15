use std::ops::Add;

use super::utils;

pub fn solve(part: u8) -> u64 {
    match part {
        1 => part_one("./src/day02/input_0.txt"),
        2 => part_two("./src/day02/input_0.txt"),
        _ => 0,
    }
}

#[derive(Debug)]
struct IdRange {
    from: u64,
    to: u64,
}

impl From<&str> for IdRange {
    fn from(value: &str) -> Self {
        let (from, to) = value.split_once("-").expect("error");
        IdRange {
            from: from.parse().unwrap(),
            to: to.parse().unwrap(),
        }
    }
}

fn part_one(path: &str) -> u64 {
    let lines: Vec<String> = utils::read_input(path);
    let ranges: Vec<IdRange> = lines
        .first()
        .unwrap()
        .split(",")
        .map(|s| s.into())
        .collect();

    let mut res: u64 = 0;
    for range in ranges {
        for n in range.from..=range.to {
            let s = n.to_string();
            let (prefix, sufix) = s.split_at(s.len() / 2);
            if prefix == sufix {
                res = res.add(n);
            }
        }
    }
    res
}

fn part_two(path: &str) -> u64 {
    let lines = utils::read_input(path);

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_0() {
        let input = "./src/day02/test_input_0.txt";
        let expected = 1227775554;
        let result = part_one(input);
        assert_eq!(result, expected);
    }

    #[test]
    #[ignore]
    fn test_case_1() {
        let input = "./src/day01/test_input_0.txt";
        let expected = 6;
        let result = part_two(input);
        assert_eq!(result, expected);
    }
}
