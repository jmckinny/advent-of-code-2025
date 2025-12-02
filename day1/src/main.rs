use std::{num::ParseIntError, path::Path, str::FromStr};

fn main() {
    let input_file = Path::new("input.txt");
    let part1_answer = part1(input_file);
    println!("Part 1: {}", part1_answer);
    let part2_answer = part2(input_file);
    println!("Part 2: {}", part2_answer);
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
#[allow(unused)]
enum DirectionParseError {
    BadInput(String),
    NoInput,
    ParseIntError(ParseIntError),
}

impl From<ParseIntError> for DirectionParseError {
    fn from(value: ParseIntError) -> Self {
        DirectionParseError::ParseIntError(value)
    }
}

impl FromStr for Direction {
    type Err = DirectionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(DirectionParseError::BadInput(s.to_string())),
        }
    }
}

#[derive(Debug)]
struct RotationCommand {
    direction: Direction,
    amount: i32,
}

impl FromStr for RotationCommand {
    type Err = DirectionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut char_iter = s.chars();
        let dir_char = char_iter.next().ok_or(DirectionParseError::NoInput)?;
        let direction = dir_char.to_string().parse()?;
        let amount = char_iter.collect::<String>().parse()?;
        Ok(RotationCommand { direction, amount })
    }
}

fn parse_file(filename: &Path) -> Vec<RotationCommand> {
    let data = std::fs::read_to_string(filename).expect("input file should exist");
    data.lines()
        .map(|line| line.parse().expect("line should be valid"))
        .collect()
}

const DIAL_SIZE: i32 = 100;
const START_POS: i32 = 50;

fn part1(filename: &Path) -> i32 {
    let rotations = parse_file(filename);
    let mut zeros = 0;
    let mut current_pos = START_POS;
    for rotation in rotations {
        let move_ammount = match rotation.direction {
            Direction::Left => -rotation.amount,
            Direction::Right => rotation.amount,
        };
        current_pos += move_ammount % DIAL_SIZE;
        if current_pos >= DIAL_SIZE {
            current_pos -= DIAL_SIZE;
        } else if current_pos < 0 {
            current_pos += DIAL_SIZE;
        }
        if current_pos == 0 {
            zeros += 1;
        }
    }
    zeros
}

fn part2(filename: &Path) -> i32 {
    let rotations = parse_file(filename);
    let mut zeros = 0;
    let mut current_pos = START_POS;
    for rotation in rotations {
        match rotation.direction {
            Direction::Right => {
                zeros += (current_pos + rotation.amount) / DIAL_SIZE;
                current_pos = (current_pos + rotation.amount).rem_euclid(DIAL_SIZE);
            }
            Direction::Left => {
                if current_pos == 0 {
                    zeros += rotation.amount / DIAL_SIZE;
                } else if rotation.amount >= current_pos {
                    zeros += ((rotation.amount - current_pos) / DIAL_SIZE) + 1;
                }
                current_pos = (current_pos - rotation.amount).rem_euclid(DIAL_SIZE);
            }
        };
    }
    zeros
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = Path::new("test_input.txt");
        let part1 = part1(test_input);
        assert_eq!(part1, 3);
    }

    #[test]
    fn test_part2() {
        let test_input = Path::new("test_input.txt");
        let part2 = part2(test_input);
        assert_eq!(part2, 6);
    }

    #[test]
    fn test_part2_edgecase() {
        let test_input = Path::new("test_edgecase.txt");
        let part2 = part2(test_input);
        assert_eq!(part2, 20);
    }

    #[test]
    fn test_part2_edgecase2() {
        let test_input = Path::new("test_edgecase2.txt");
        let part2 = part2(test_input);
        assert_eq!(part2, 2);
    }
}
