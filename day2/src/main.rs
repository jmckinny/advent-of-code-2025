use std::path::Path;

fn main() {
    let input_file = Path::new("input.txt");
    let part1_answer = part1(input_file);
    println!("Part 1: {}", part1_answer);
    let part2_answer = part2(input_file);
    println!("Part 2: {}", part2_answer);
}

struct IdRange {
    low: u64,
    high: u64,
}

impl IdRange {
    pub fn get_invalid_ids(&self, is_invalid_id: &dyn Fn(u64) -> bool) -> Vec<u64> {
        let mut invalid_ids = Vec::new();
        for i in self.low..=self.high {
            if is_invalid_id(i) {
                invalid_ids.push(i);
            }
        }
        invalid_ids
    }
}

fn is_invalid_id_part1(id: u64) -> bool {
    let digit_string = id.to_string();
    if !digit_string.len().is_multiple_of(2) {
        return false;
    }
    let half = digit_string.len() / 2;
    digit_string[0..half] == digit_string[half..digit_string.len()]
}

fn is_invalid_id_part2(id: u64) -> bool {
    let digit_string = id.to_string();
    let half_len = digit_string.len() / 2;

    for chunk_size in (1..=half_len).rev() {
        let mut chunks = digit_string.as_bytes().chunks(chunk_size);
        let first = chunks.next().expect("first chunk should exist");
        if chunks.all(|chunk| first == chunk) {
            return true;
        }
    }
    false
}

fn parse_file(filename: &Path) -> Vec<IdRange> {
    let text = std::fs::read_to_string(filename).expect("input file should exist");
    text.split(',')
        .map(|range| {
            let mut split_hiphen = range.split('-');
            let low = split_hiphen
                .next()
                .expect("low should exist")
                .trim_end()
                .parse()
                .expect("should be number");
            let high = split_hiphen
                .next()
                .expect("high should exist")
                .trim_end()
                .parse()
                .expect("should be number");
            IdRange { low, high }
        })
        .collect()
}

fn part1(filename: &Path) -> u64 {
    let ranges = parse_file(filename);
    ranges
        .iter()
        .flat_map(|r| r.get_invalid_ids(&is_invalid_id_part1))
        .sum()
}

fn part2(filename: &Path) -> u64 {
    let ranges = parse_file(filename);
    ranges
        .iter()
        .flat_map(|r| r.get_invalid_ids(&is_invalid_id_part2))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = Path::new("test_input.txt");
        let part1 = part1(test_input);
        assert_eq!(part1, 1227775554);
    }

    #[test]
    fn test_part2() {
        let test_input = Path::new("test_input.txt");
        let part2 = part2(test_input);
        assert_eq!(part2, 4174379265);
    }

    #[test]
    fn test_is_invalid_id_part1() {
        assert!(!is_invalid_id_part1(10));
        assert!(is_invalid_id_part1(11));
        assert!(is_invalid_id_part1(99));
    }

    #[test]
    fn test_is_invalid_id_part2() {
        assert!(is_invalid_id_part2(1010));
        assert!(!is_invalid_id_part2(10));
        assert!(is_invalid_id_part2(11));
        assert!(is_invalid_id_part2(22));
        assert!(is_invalid_id_part2(99));
        assert!(is_invalid_id_part2(999));
        assert!(is_invalid_id_part2(1010));
        assert!(is_invalid_id_part2(2121212121));
        assert!(is_invalid_id_part2(38593859));
        assert!(is_invalid_id_part2(1188511885));
    }
}
