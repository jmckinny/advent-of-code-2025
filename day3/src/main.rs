use std::path::Path;

fn main() {
    let input_file = Path::new("input.txt");
    let part1_answer = part1(input_file);
    println!("Part 1: {}", part1_answer);
    let part2_answer = part2(input_file);
    println!("Part 2: {}", part2_answer);
}

fn parse_file(filename: &Path) -> Vec<Vec<u8>> {
    let text = std::fs::read_to_string(filename).expect("input file should exist");
    text.lines()
        .map(|line| {
            line.chars()
                .map(|digit| digit.to_digit(10).expect("should be digit") as u8)
                .collect()
        })
        .collect()
}

fn get_bank_joltage_part1(bank: &[u8]) -> u64 {
    let (max, max_index) = get_max_joltage(&bank[0..bank.len() - 1]);
    let (second_max, _) = get_max_joltage(&bank[max_index + 1..]);

    format!("{max}{second_max}")
        .parse()
        .expect("should be number")
}

const MAX_JOLTAGE_DIGITS: usize = 12;

fn get_bank_joltage_part2(bank: &[u8]) -> u64 {
    let mut total_joltage = String::with_capacity(MAX_JOLTAGE_DIGITS);
    let mut start = 0;
    let mut end = bank.len() - MAX_JOLTAGE_DIGITS;
    while total_joltage.len() != MAX_JOLTAGE_DIGITS {
        let (joltage, index) = get_max_joltage(&bank[start..=end]);
        total_joltage.push_str(&joltage.to_string());
        start = (index + start) + 1;
        end = bank.len() - (MAX_JOLTAGE_DIGITS - total_joltage.len());
    }
    total_joltage.parse().expect("should be number")
}

fn get_max_joltage(bank: &[u8]) -> (u8, usize) {
    let mut max = bank[0];
    let mut max_index = 0;
    (0..bank.len()).for_each(|i| {
        if bank[i] > max {
            max = bank[i];
            max_index = i;
        }
    });
    (max, max_index)
}

fn part1(filename: &Path) -> u64 {
    let battery_banks = parse_file(filename);
    battery_banks
        .iter()
        .map(|bank| get_bank_joltage_part1(bank))
        .sum()
}

fn part2(filename: &Path) -> u64 {
    let battery_banks = parse_file(filename);
    battery_banks
        .iter()
        .map(|bank| get_bank_joltage_part2(bank))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = Path::new("test_input.txt");
        let part1 = part1(test_input);
        assert_eq!(part1, 357);
    }

    #[test]
    fn test_part2() {
        let test_input = Path::new("test_input.txt");
        let part2 = part2(test_input);
        assert_eq!(part2, 3121910778619);
    }

    #[test]
    fn test_get_bank_joltage_part2() {
        assert_eq!(
            get_bank_joltage_part2(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]),
            987654321111
        );
        assert_eq!(
            get_bank_joltage_part2(&[8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]),
            811111111119
        );
        assert_eq!(
            get_bank_joltage_part2(&[2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]),
            434234234278
        );
        assert_eq!(
            get_bank_joltage_part2(&[8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]),
            888911112111
        );
    }
}
