use std::{collections::HashMap, path::Path, str::FromStr};

fn main() {
    let input_file = Path::new("input.txt");
    let part1_answer = part1(input_file);
    println!("Part 1: {}", part1_answer);
    let part2_answer = part2(input_file);
    println!("Part 2: {}", part2_answer);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RoomItem {
    Empty,
    Start,
    Beam,
    Splitter,
}

impl TryFrom<char> for RoomItem {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(RoomItem::Empty),
            'S' => Ok(RoomItem::Start),
            '|' => Ok(RoomItem::Beam),
            '^' => Ok(RoomItem::Splitter),
            _ => Err(format!("got invalid room item char: {}", value)),
        }
    }
}

impl std::fmt::Display for RoomItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RoomItem::Empty => write!(f, "."),
            RoomItem::Start => write!(f, "S"),
            RoomItem::Beam => write!(f, "|"),
            RoomItem::Splitter => write!(f, "^"),
        }
    }
}

#[derive(Debug, Clone)]
struct Room {
    grid: Vec<Vec<RoomItem>>,
}

impl std::fmt::Display for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter() {
            for item in row {
                write!(f, "{}", item)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for Room {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| char.try_into().expect("should be valid RoomItem"))
                    .collect()
            })
            .collect();
        Ok(Room { grid })
    }
}

#[derive(Debug)]
struct Simulator {
    room: Room,
    tick: usize,
    splits: u64,
}

impl Simulator {
    pub fn with_room(room: Room) -> Self {
        Simulator {
            room,
            tick: 0,
            splits: 0,
        }
    }

    pub fn get_room_item(&self, row: usize, col: usize) -> Option<RoomItem> {
        self.room.grid.get(row)?.get(col).copied()
    }

    pub fn set_room_item(&mut self, item: RoomItem, row: usize, col: usize) {
        if row > self.room.grid.len() || col > self.room.grid[row].len() {
            return;
        }
        self.room.grid[row][col] = item;
    }

    pub fn is_item(&self, item: RoomItem, row: usize, col: usize) -> bool {
        if let Some(room_item) = self.get_room_item(row, col) {
            return room_item == item;
        }
        false
    }

    pub fn get_start(&self) -> (usize, usize) {
        for row in 0..self.room.grid.len() {
            for col in 0..self.room.grid[row].len() {
                if self.is_item(RoomItem::Start, row, col) {
                    return (row, col);
                }
            }
        }
        unreachable!("room must have start")
    }

    pub fn beam_can_go(&self, row: usize, col: usize) -> bool {
        if let Some(item) = self.get_room_item(row, col) {
            return match item {
                RoomItem::Empty => true,
                RoomItem::Start => false,
                RoomItem::Beam => false,
                RoomItem::Splitter => true,
            };
        }
        false
    }

    pub fn sim_over(&self) -> bool {
        if self.tick == 0 {
            return false;
        }
        for row in self.tick..self.room.grid.len() {
            for col in 0..self.room.grid[row].len() {
                if self.is_item(RoomItem::Beam, row, col) && self.beam_can_go(row + 1, col) {
                    return false;
                }
            }
        }
        true
    }

    pub fn simulate_tick_pt1(&mut self) {
        for row in (self.tick..self.room.grid.len()).rev() {
            for col in 0..self.room.grid[row].len() {
                let Some(item) = self.get_room_item(row, col) else {
                    continue;
                };
                if matches!(item, RoomItem::Beam) || matches!(item, RoomItem::Start) {
                    self.expand_beam_pt1(row, col);
                }
            }
        }
        self.tick += 1;
    }

    fn expand_beam_pt1(&mut self, row: usize, col: usize) {
        if let Some(item) = self.get_room_item(row + 1, col) {
            match item {
                RoomItem::Empty => self.set_room_item(RoomItem::Beam, row + 1, col),
                RoomItem::Start => {
                    self.set_room_item(RoomItem::Beam, row + 1, col);
                }
                RoomItem::Beam => {}
                RoomItem::Splitter => {
                    self.set_room_item(RoomItem::Beam, row + 1, col + 1);
                    self.set_room_item(RoomItem::Beam, row + 1, col - 1);
                    self.splits += 1;
                }
            }
        }
    }

    pub fn simulate_timelines(&mut self) -> u64 {
        let mut cache: HashMap<(usize, usize), u64> = HashMap::new();
        let (row, col) = self.get_start();
        self.calculate_timeline_count(&mut cache, row, col)
    }

    pub fn calculate_timeline_count(
        &mut self,
        cache: &mut HashMap<(usize, usize), u64>,
        row: usize,
        col: usize,
    ) -> u64 {
        if let Some(timelines) = cache.get(&(row, col)) {
            return *timelines;
        }
        let item = match self.get_room_item(row + 1, col) {
            None => return 1,
            Some(item) => item,
        };
        match item {
            RoomItem::Empty | RoomItem::Start => {
                let down = self.calculate_timeline_count(cache, row + 1, col);
                cache.insert((row + 1, col), down);
                down
            }
            RoomItem::Splitter => {
                let left = self.calculate_timeline_count(cache, row + 1, col - 1);
                cache.insert((row + 1, col - 1), left);
                let right = self.calculate_timeline_count(cache, row + 1, col + 1);
                cache.insert((row + 1, col + 1), right);
                left + right
            }
            RoomItem::Beam => unreachable!("should be no beams in part2"),
        }
    }
}

fn parse_file(filename: &Path) -> Room {
    let text = std::fs::read_to_string(filename).expect("input should exist");
    text.parse().expect("should be valid room")
}

fn part1(filename: &Path) -> u64 {
    let room = parse_file(filename);
    let mut sim = Simulator::with_room(room);
    while !sim.sim_over() {
        sim.simulate_tick_pt1();
    }
    sim.splits
}

fn part2(filename: &Path) -> u64 {
    let room = parse_file(filename);
    let mut sim = Simulator::with_room(room);
    sim.simulate_timelines()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = Path::new("test_input.txt");
        let part1 = part1(test_input);
        assert_eq!(part1, 21);
    }

    #[test]
    fn test_part2() {
        let test_input = Path::new("test_input.txt");
        let part2 = part2(test_input);
        assert_eq!(part2, 40);
    }
}
