use std::{
    cmp::{max, min, Ordering},
    collections::{HashMap, HashSet, VecDeque},
};

use aoc_runner_derive::{aoc, aoc_generator};
#[derive(Debug, PartialEq)]
enum Tile {
    Wall,
    Open,
    Unit(UnitType),
}
type Position = (usize, usize);

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
enum UnitType {
    Goblin,
    Elf,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Unit {
    id: u32,
    unittype: UnitType,
    health: u32,
    x: usize,
    y: usize,
    attack: u32,
}
struct Cave {
    grid: Vec<Vec<Tile>>,
    units: Vec<Unit>,
    rounds: usize,
}

impl Cave {
    fn new(input: &str) -> Self {
        let mut grid = Vec::new();
        let mut units: Vec<Unit> = Vec::new();

        for line in input.trim().lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                let tile = match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Open,
                    'G' => Tile::Unit(UnitType::Goblin),
                    'E' => Tile::Unit(UnitType::Elf),
                    _ => panic!("unknown char {} in grid", c),
                };
                row.push(tile);
            }
            grid.push(row);
        }
        let mut id = 0;

        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                match &grid[y][x] {
                    Tile::Unit(v) => {
                        units.push(Unit {
                            id,
                            unittype: *v,
                            health: 200,
                            x: x,
                            y: y,
                            attack: 3,
                        });
                        id += 1;
                    }
                    _ => {}
                }
            }
        }
        Cave {
            grid,
            units,
            rounds: 0,
        }
    }

    fn print(&self) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                print!(
                    "{}",
                    match self.grid[y][x] {
                        Tile::Wall => '#',
                        Tile::Unit(UnitType::Elf) => 'E',
                        Tile::Unit(UnitType::Goblin) => 'G',
                        Tile::Open => '.',
                    }
                );
            }
            println!();
        }
    }

    fn sort_units(self: &mut Self) {
        self.units.sort_by(|a, b| {
            if a.y < b.y {
                Ordering::Less
            } else if a.y == b.y && a.x < b.x {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
    }

    fn find_target_for_unit(&self, unit: &Unit) -> Option<(u32, usize)> {
        // for all targets, get shortest path
        let possible_targets: Vec<Unit> = self
            .units
            .iter()
            .filter(|u| u.unittype != unit.unittype)
            .copied()
            .collect();

        let target = possible_targets
            .iter()
            .map(|u| (u.id, self.get_target_distance(unit, u)))
            .min_by_key(|(_, d)| d.clone());

        if target.is_some() {
            return target.unwrap().0;
        }
        None
    }

    fn get_target_distance(&self, src_unit: &Unit, dst_unit: &Unit) -> Option<usize> {
        if let Some(path) = get_path(
            &self.grid,
            (src_unit.x, src_unit.y),
            (dst_unit.x, dst_unit.y),
        ) {
            return Some(path.0);
        } else {
            None
        }
    }

    fn get_target_1st_move(&self, src_unit: &Unit, dst_unit: &Unit) -> Option<Position> {
        if let Some(path) = get_path(
            &self.grid,
            (src_unit.x, src_unit.y),
            (dst_unit.x, dst_unit.y),
        ) {
            let steps=path.1;
            while prev=steps.
            return Some(path.0,path.1.get(k));
        } else {
            None
        }
    }

    fn run(&mut self) -> u32 {
        while self
            .units
            .iter()
            .filter(|&&u| u.unittype == UnitType::Elf && u.health > 0)
            .count()
            > 0
        {
            self.round();
        }
        self.units
            .iter()
            .filter(|u| u.health > 0)
            .map(|u| u.health)
            .sum()
    }

    fn round(&mut self) {
        self.units.sort();

        for unit in self.units.iter() {
            if let target = self.find_target_for_unit(unit) {
                // Handle targer
                todo!();
            } else {
                // No target means blocked or alone
                todo!();
            }
        }
    }
}

fn get_path(
    grid: &Vec<Vec<Tile>>,
    src: Position,
    destination: Position,
) -> Option<(usize, HashMap<Position, Position>)> {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    let mut prev: HashMap<Position, Position> = HashMap::new();
    queue.push_back((src, 0));

    while queue.len() > 0 {
        if let Some(current) = queue.pop_front() {
            seen.insert(current.0);
            if current.0 == destination {
                return Some((current.1, prev));
            }
            let next_moves = get_available_neighbors(grid, current.0);
            next_moves.iter().for_each(|n| {
                if !seen.contains(n) {
                    queue.push_back((*n, current.1 + 1));
                    prev.insert(*n, current.0);
                }
            });
        }
    }
    None
}
fn get_available_neighbors(grid: &Vec<Vec<Tile>>, pos: Position) -> Vec<Position> {
    let mut positions: Vec<Position> = Vec::new();

    let (x, y) = pos;
    if x > 0 {
        if grid[y][x - 1] == Tile::Open {
            positions.push((x - 1, y));
        }
    }
    if y > 0 {
        if grid[y - 1][x] == Tile::Open {
            positions.push((x, y - 1));
        }
    }
    if x < grid.len() - 1 {
        if grid[y][x + 1] == Tile::Open {
            positions.push((x + 1, y));
        }
    }
    if y < grid[0].len() - 1 {
        if grid[y + 1][x] == Tile::Open {
            positions.push((x, y + 1));
        }
    }
    positions
}

#[aoc_generator(day15)]
fn input_generator(input: &str) -> Cave {
    let input = "#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";

    Cave::new(input)
}

#[aoc(day15, part1)]
fn solve_part1(input: &Cave) -> u32 {
    input.print();
    0
}

#[aoc(day15, part2)]
fn solve_part2(input: &Cave) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    fn create_cave_tiles() -> Cave {
        let input = "\
        #######\n\
        #.G.E.#\n\
        #E.G.E#\n\
        #.G.E.#\n\
        #######";

        Cave::new(input)
    }

    #[test]
    fn test_cave_creation() {
        let cave = create_cave_tiles();

        // Check specific tiles
        assert_eq!(cave.grid[0][0], Tile::Wall); // Top-left corner should be a wall
        assert_eq!(cave.grid[1][2], Tile::Unit(UnitType::Goblin)); // There should be a Goblin at (2, 1)
        assert_eq!(cave.grid[1][4], Tile::Unit(UnitType::Elf)); // There should be an Elf at (4, 1)
        assert_eq!(cave.grid[1][1], Tile::Open); // There should be an open space at (1, 1)
    }
    #[test]
    fn test_unit_count() {
        let cave = create_cave_tiles();

        // Check specific tiles
        assert_eq!(
            cave.grid
                .iter()
                .flatten()
                .filter(|&t| *t == Tile::Unit(UnitType::Elf) || *t == Tile::Unit(UnitType::Goblin))
                .count(),
            7
        ); // Top-left corner should be a wall
    }

    #[test]
    fn test_sort_units() {
        let mut cave = create_cave_tiles();
        cave.sort_units();

        let mut units = cave.units.iter();
        units.next();
        units.next();
        let u = units.next().unwrap();
        assert_eq!(u.x, 1)
    }

    #[test]
    fn test_find_next_move() {
        let mut cave = create_cave_tiles();
        cave.sort_units();
    }

    #[test]
    fn test_available_positions() {
        let cave = create_cave_tiles();
        assert_eq!(get_available_neighbors(&cave.grid, (0, 0)).len(), 0);
        assert_eq!(get_available_neighbors(&cave.grid, (1, 1)).len(), 0);
        assert_eq!(get_available_neighbors(&cave.grid, (2, 1)).len(), 3);
    }
}
