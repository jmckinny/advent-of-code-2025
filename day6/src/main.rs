use std::{path::Path, str::FromStr};

fn main() {
    let input_file = Path::new("input.txt");
    let part1_answer = part1(input_file);
    println!("Part 1: {}", part1_answer);
    let part2_answer = part2(input_file);
    println!("Part 2: {}", part2_answer);
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
}

impl FromStr for Operator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Add),
            "*" => Ok(Operator::Multiply),
            _ => Err(format!("invalid operator found: '{s}'")),
        }
    }
}

#[derive(Debug)]
struct Problem {
    numbers: Vec<u64>,
    operator: Operator,
}

impl Problem {
    pub fn solve(&self) -> u64 {
        match self.operator {
            Operator::Add => self.numbers.iter().sum(),
            Operator::Multiply => self.numbers.iter().product(),
        }
    }
}

fn parse_file(filename: &Path) -> Vec<Problem> {
    let text = std::fs::read_to_string(filename).expect("input file should exist");
    let number_rows: Vec<Vec<u64>> = text
        .lines()
        .take(text.lines().count() - 1)
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse().expect("should be number"))
                .collect()
        })
        .collect();
    let operators: Vec<Operator> = text
        .lines()
        .last()
        .expect("Should have last line")
        .split_whitespace()
        .map(|op_str| op_str.parse().expect("should be operator"))
        .collect();
    let mut problems = Vec::new();
    for i in 0..operators.len() {
        let numbers = number_rows.iter().map(|row| row[i]).collect();
        let operator = operators[i];
        problems.push(Problem { numbers, operator });
    }
    problems
}

fn parse_file_pt2(filename: &Path) -> Vec<Problem> {
    let text = std::fs::read_to_string(filename).expect("input file should exist");
    let operators: Vec<Operator> = text
        .lines()
        .last()
        .expect("Should have last line")
        .split_whitespace()
        .map(|op_str| op_str.parse().expect("should be operator"))
        .rev()
        .collect();

    let number_matrix: Vec<Vec<char>> = text
        .lines()
        .take(text.lines().count() - 1)
        .map(|line| line.chars().collect())
        .collect();

    let mut number_rows: Vec<Vec<u64>> = Vec::new();
    let mut current_problem: Vec<u64> = Vec::new();
    for col in (0..number_matrix[0].len()).rev() {
        let new_problem = (0..number_matrix.len()).all(|i| number_matrix[i][col].is_whitespace());
        if new_problem {
            number_rows.push(current_problem.clone());
            current_problem.clear();
            continue;
        }
        let mut number_str = String::new();
        (0..number_matrix.len()).for_each(|row| {
            if number_matrix[row][col].is_whitespace() {
                return;
            }
            number_str.push(number_matrix[row][col]);
        });
        current_problem.push(number_str.parse().expect("should be number"));
    }
    number_rows.push(current_problem);

    operators
        .iter()
        .enumerate()
        .map(|(i, op)| Problem {
            operator: *op,
            numbers: number_rows[i].clone(),
        })
        .collect()
}

fn part1(filename: &Path) -> u64 {
    let problems = parse_file(filename);
    problems.iter().map(|p| p.solve()).sum()
}

fn part2(filename: &Path) -> u64 {
    let problems = parse_file_pt2(filename);
    problems.iter().map(|p| p.solve()).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = Path::new("test_input.txt");
        let part1 = part1(test_input);
        assert_eq!(part1, 4277556);
    }

    #[test]
    fn test_part2() {
        let test_input = Path::new("test_input.txt");
        let part2 = part2(test_input);
        assert_eq!(part2, 3263827);
    }
}
