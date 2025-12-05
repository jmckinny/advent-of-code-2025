use std::path::Path;

fn main() {
    let input_file = Path::new("input.txt");
    let part1_answer = part1(input_file);
    println!("Part 1: {}", part1_answer);
    let part2_answer = part2(input_file);
    println!("Part 2: {}", part2_answer);
}

type SlotGrid = Vec<Vec<Slot>>;

#[derive(Debug)]
enum Slot {
    Roll,
    Empty,
}

impl From<char> for Slot {
    fn from(value: char) -> Self {
        match value {
            '@' => Slot::Roll,
            _ => Slot::Empty,
        }
    }
}

impl Slot {
    pub fn is_roll(&self) -> bool {
        matches!(self, Slot::Roll)
    }
}

fn count_surrounding_rolls(grid: &SlotGrid, row: usize, col: usize) -> u64 {
    let row_offsets = [-1isize, 0, 1];
    let col_offsets = [-1isize, 0, 1];
    let mut count = 0;
    for row_offset in row_offsets {
        for col_offset in col_offsets {
            if row_offset == 0 && col_offset == 0 {
                // don't count the current roll
                continue;
            }
            let nearby_row = match row.checked_add_signed(row_offset) {
                Some(index) => index,
                None => continue,
            };
            let nearby_col = match col.checked_add_signed(col_offset) {
                Some(index) => index,
                None => continue,
            };
            let Some(slot) = get_slot(grid, nearby_row, nearby_col) else {
                continue;
            };
            if slot.is_roll() {
                count += 1;
            }
        }
    }
    count
}

fn get_slot(grid: &SlotGrid, row: usize, col: usize) -> Option<&Slot> {
    let row = grid.get(row)?;
    row.get(col)
}

fn parse_file(filename: &Path) -> SlotGrid {
    let text = std::fs::read_to_string(filename).expect("input file should exist");
    text.lines()
        .map(|line| line.chars().map(|char| char.into()).collect())
        .collect()
}

fn attempt_remove_roll(grid: &mut SlotGrid) -> bool {
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if let Some(slot) = get_slot(grid, row, col)
                && !slot.is_roll()
            {
                continue;
            }
            let nearby_rolls = count_surrounding_rolls(grid, row, col);
            if nearby_rolls < 4 {
                grid[row][col] = Slot::Empty;
                return true;
            }
        }
    }
    false
}

fn part1(filename: &Path) -> u64 {
    let grid = parse_file(filename);
    let mut open_rolls = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if let Some(slot) = get_slot(&grid, row, col)
                && !slot.is_roll()
            {
                continue;
            }

            let nearby_rolls = count_surrounding_rolls(&grid, row, col);
            if nearby_rolls < 4 {
                open_rolls += 1;
            }
        }
    }
    open_rolls
}

fn part2(filename: &Path) -> u64 {
    let mut grid = parse_file(filename);
    let mut total_removed = 0;
    while attempt_remove_roll(&mut grid) {
        total_removed += 1;
    }
    total_removed
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = Path::new("test_input.txt");
        let part1 = part1(test_input);
        assert_eq!(part1, 13);
    }

    #[test]
    fn test_part2() {
        let test_input = Path::new("test_input.txt");
        let part2 = part2(test_input);
        assert_eq!(part2, 43);
    }
}
