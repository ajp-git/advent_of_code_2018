use std::{cmp::{max, min, Ordering}, collections::HashMap};

use aoc_runner_derive::{aoc, aoc_generator};
#[derive(Debug,PartialEq)]
enum Tile {
    Wall,
    Open,
    Unit(UnitType),
}

#[derive(Debug,PartialEq, Clone, Copy)]
enum UnitType {
    Goblin,
    Elf,
}
#[derive(Debug,PartialEq)]
struct Unit{
    unittype:UnitType,
    health:u32,
    x:usize,
    y:usize,
    attack:u32,
}
struct Cave {
    grid:Vec<Vec<Tile>>,
    units:Vec<Unit>,
}

impl Cave {
    fn new (input: &str) -> Self {

        let mut grid=Vec::new();
        let mut units:Vec<Unit>=Vec::new();

        for line in input.trim().lines(){
            let mut row=Vec::new();
            for c in line.chars() {
                let tile=
                    match c {
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

        for y in 0..grid.len(){
            for x in 0..grid[0].len() {
                match &grid[y][x] {
                    Tile::Unit(v) => {
                        units.push(Unit { 
                            unittype: *v,
                            health: 200,
                            x:x, y:y,
                            attack: 3 } );
                    },
                    _ => {},
                }
            }
        }
        Cave { grid, units }
    }

    fn print(&self) {

        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len(){
                print!("{}",
                match self.grid[y][x]{
                    Tile::Wall => '#',
                    Tile::Unit(UnitType::Elf) => 'E',
                    Tile::Unit(UnitType::Goblin) => 'G',
                    Tile::Open => '.',
                });
            }
            println!();
        }
    }

    fn sort_units(self: &mut Self){

        self.units.sort_by(|a,b| 
            if a.y < b.y {
                Ordering::Less
            } else if a.y==b.y && a.x < b.x {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        );
    }

}

#[aoc_generator(day15)]
fn input_generator(input: &str) -> Cave {
    let input="#######
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

        let cave=create_cave_tiles();

        // Check specific tiles
        assert_eq!(cave.grid[0][0], Tile::Wall); // Top-left corner should be a wall
        assert_eq!(cave.grid[1][2], Tile::Unit(UnitType::Goblin)); // There should be a Goblin at (2, 1)
        assert_eq!(cave.grid[1][4], Tile::Unit(UnitType::Elf)); // There should be an Elf at (4, 1)
        assert_eq!(cave.grid[1][1], Tile::Open); // There should be an open space at (1, 1)
    }
    #[test]
    fn test_unit_count() {

        let cave=create_cave_tiles();

        // Check specific tiles
        assert_eq!(cave.
            grid
            .iter()
            .flatten()
            .filter(
                |&t|
                *t==Tile::Unit(UnitType::Elf) || *t==Tile::Unit(UnitType::Goblin))
            .count(), 7); // Top-left corner should be a wall
    }

    #[test]
    fn test_sort_units() {
        let mut cave=create_cave_tiles();
        cave.sort_units();

        let mut units=cave.units.iter();
        units.next();
        units.next();
        let u=units.next().unwrap();
        assert_eq!(u.x, 1)
    }

    #[test]
    fn test_find_next_move() {
        let mut cave=create_cave_tiles();
        cave.sort_units();

        
    }
}