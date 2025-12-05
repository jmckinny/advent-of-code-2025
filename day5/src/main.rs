use std::{
    cmp::{max, min},
    num::ParseIntError,
    path::Path,
    str::FromStr,
};

fn main() {
    let input_file = Path::new("input.txt");
    let part1_answer = part1(input_file);
    println!("Part 1: {}", part1_answer);
    let part2_answer = part2(input_file);
    println!("Part 2: {}", part2_answer);
}

type IngredientID = u64;

struct KitchenState {
    fresh_ranges: Vec<IngredientRange>,
    available_ingredients: Vec<IngredientID>,
}

impl FromStr for KitchenState {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("\n\n");
        let fresh_ranges = split
            .next()
            .unwrap_or_default()
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<_>, _>>()?;
        let available_ingredients = split
            .next()
            .unwrap_or_default()
            .lines()
            .map(|line| line.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(KitchenState {
            fresh_ranges,
            available_ingredients,
        })
    }
}

#[derive(Debug, Clone)]
struct IngredientRange {
    low: IngredientID,
    high: IngredientID,
}

impl IngredientRange {
    pub fn contains_ingredient(&self, id: IngredientID) -> bool {
        (self.low..=self.high).contains(&id)
    }

    pub fn range_overlaps(&self, other: &IngredientRange) -> bool {
        let contains_low = self.contains_ingredient(other.low);
        let contains_high = self.contains_ingredient(other.high);
        contains_low || contains_high
    }

    pub fn combine_range(&self, other: &IngredientRange) -> IngredientRange {
        if !self.range_overlaps(other) && !other.range_overlaps(self) {
            panic!("tried to combine bad ranges");
        }
        IngredientRange {
            low: min(self.low, other.low),
            high: max(self.high, other.high),
        }
    }

    pub fn range_size(&self) -> u64 {
        self.high - self.low + 1
    }
}

impl FromStr for IngredientRange {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('-');
        let low = split.next().unwrap_or_default().parse()?;
        let high = split.next().unwrap_or_default().parse()?;
        Ok(IngredientRange { low, high })
    }
}

fn parse_file(filename: &Path) -> KitchenState {
    let text = std::fs::read_to_string(filename).expect("input file should exist");
    text.parse().expect("input format should be valid")
}

fn part1(filename: &Path) -> u64 {
    let kitchen_state = parse_file(filename);
    kitchen_state
        .available_ingredients
        .iter()
        .filter(|id| {
            kitchen_state
                .fresh_ranges
                .iter()
                .any(|range| range.contains_ingredient(**id))
        })
        .count() as u64
}

fn part2(filename: &Path) -> u64 {
    let mut kitchen_state = parse_file(filename);
    kitchen_state.fresh_ranges.sort_unstable_by_key(|r| r.low);
    let mut ranges: Vec<IngredientRange> = vec![];
    for i in 0..kitchen_state.fresh_ranges.len() {
        let kitchen_range = &kitchen_state.fresh_ranges[i];
        let mut already_in_ranges = false;
        for range in ranges.iter_mut() {
            if range.range_overlaps(kitchen_range) {
                already_in_ranges = true;
                *range = range.combine_range(kitchen_range);
                break;
            }
        }
        if !already_in_ranges {
            ranges.push(kitchen_state.fresh_ranges[i].clone());
        }
    }
    ranges.iter().map(|r| r.range_size()).sum()
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
        assert_eq!(part2, 14);
    }
}
