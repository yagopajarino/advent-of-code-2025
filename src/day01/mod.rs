use super::utils;

#[derive(Debug)]
struct Rotation {
    pub direction: Direction,
    pub distance: i32,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

pub fn solve(part: u8) -> i32 {
    match part {
        1 => part_one("./src/day01/input_0.txt"),
        2 => part_two("./src/day01/input_0.txt"),
        _ => -1,
    }
}

fn part_one(path: &str) -> i32 {
    let lines = utils::read_input(path);

    let rotations = to_rotations(lines);

    let mut res = 0;
    let mut dial_pointer = 50;
    for rotation in rotations {
        for _ in 0..rotation.distance {
            dial_pointer = match rotation.direction {
                Direction::Left => dial_pointer - 1,
                Direction::Right => dial_pointer + 1,
            };
        }
        if dial_pointer % 100 == 0 {
            res += 1
        }
    }
    res
}

fn part_two(path: &str) -> i32 {
    let lines = utils::read_input(path);

    let rotations = to_rotations(lines);

    let mut res = 0;
    let mut dial_pointer = 50;
    for rotation in rotations {
        for _ in 0..rotation.distance {
            dial_pointer = match rotation.direction {
                Direction::Left => dial_pointer - 1,
                Direction::Right => dial_pointer + 1,
            };
            if dial_pointer % 100 == 0 {
                res += 1
            }
        }
    }
    res
}

fn to_rotations(lines: Vec<String>) -> Vec<Rotation> {
    lines
        .iter()
        .filter(|line| line.len() > 1)
        .map(|line| {
            let mut direction = line.clone();
            let distance: i32 = direction.split_off(1).parse().expect("Invalid distance");
            let direction = match direction.as_str() {
                "L" => Ok(Direction::Left),
                "R" => Ok(Direction::Right),
                _ => Err("Invalid direction"),
            }
            .unwrap();
            Rotation {
                direction,
                distance,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_case_0() {
        let input = "./src/day01/test_input_0.txt";
        let expected = 3;
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

    #[test]
    fn solve_it_0() {
        let input = "./src/day01/input_0.txt";
        let expected = 1105;
        let result = part_one(input);
        println!();
        println!("-----------------------------------------");
        println!("Result for the part one: {:?}", expected);
        println!("------------------------------------------");
        println!();
        assert_eq!(result, expected);
    }

    #[test]
    fn solve_it_1() {
        let input = "./src/day01/input_1.txt";
        let expected = 6599;
        let result = part_two(input);
        println!();
        println!("------------------------------------------");
        println!("Result for the part two: {:?}             ", expected);
        println!("------------------------------------------");
        println!();
        assert_eq!(result, expected);
    }
}
