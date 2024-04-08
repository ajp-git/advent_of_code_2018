use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};


enum Grid {
    Wall,
    Open,
    Goblin(u32),
    Elf(u32),
}
struct Cave {
    grid:HashMap<(usize,usize),Grid>,
}

impl Cave {
    fn new (input: &str) -> Self {

        let mut grid:HashMap<(usize,usize), Grid>=HashMap::new();
    
        for (y, line) in input.lines().enumerate(){
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => grid.insert((x,y), Grid::Wall),
                    '.' => grid.insert((x,y), Grid::Open),
                    'G' => grid.insert((x,y), Grid::Goblin(200)),
                    'E' => grid.insert((x,y), Grid::Elf(200)),
                    _ => panic!("unknown char {} in grid", c),
                };
            }
        }
        Cave { grid:grid }
    }

    fn print(&self) {
        let max_x=self.grid.keys().map(|(x,_)|x).max().unwrap();
        let max_y=self.grid.keys().map(|(_,y)|y).max().unwrap();
        
    }
}

#[aoc_generator(day15)]
fn input_generator(input: &str) -> Cave {

    Cave::new(input)
}

#[aoc(day15, part1)]
fn solve_part1(input: &Cave) -> u32 {
    0
}

#[aoc(day15, part2)]
fn solve_part2(input: &Cave) -> u32 {
    
    0
}

