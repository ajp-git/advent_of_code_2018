#[warn(missing_debug_implementations, missing_docs)]
use std::cmp::min;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, PartialEq)]
enum Acre {
    Ground,
    Tree,
    LumberYard,
}
#[derive(Debug, Clone)]
struct Lumber {
    grid: Vec<Vec<Acre>>,
}
impl Lumber {
    fn new() -> Self {
        Lumber { grid: Vec::new() }
    }
    fn step(&mut self) {
        let mut new_grid = self.grid.clone();
        for x in 0..self.grid.len() {
            for y in 0..self.grid[0].len() {
                let (ground, lumberyard, tree) = self.get_surroudings(x, y);
                match self.grid[y][x] {
                    Acre::Ground => {
                        if tree >= 3 {
                            new_grid[y][x] = Acre::Tree;
                        }
                    }
                    Acre::Tree => {
                        if lumberyard >= 3 {
                            new_grid[y][x] = Acre::LumberYard;
                        }
                    }
                    Acre::LumberYard => {
                        if lumberyard >= 1 && tree >= 1 {
                            new_grid[y][x] = Acre::LumberYard;
                        } else {
                            new_grid[y][x] = Acre::Ground;
                        }
                    }
                }
            }
        }
        self.grid = new_grid;
    }

    fn get_surroudings(&self, x: usize, y: usize) -> (u32, u32, u32) {
        let mut surround = (0, 0, 0);
        for xs in 0.max(x as isize - 1) as usize..=min(self.grid.len() - 1, x + 1) {
            for ys in 0.max(y as isize - 1) as usize..=min(self.grid[0].len() - 1, y + 1) {
                if xs == x && ys == y {
                    continue;
                }
                match self.grid[ys][xs] {
                    Acre::Ground => surround.0 += 1,
                    Acre::LumberYard => surround.1 += 1,
                    Acre::Tree => surround.2 += 1,
                }
            }
        }
        surround
    }

    fn print(&self) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                match self.grid[y][x] {
                    Acre::Ground => print!("."),
                    Acre::LumberYard => print!("#"),
                    Acre::Tree => print!("|"),
                }
            }
            println!();
        }
        println!();
    }
}

#[aoc_generator(day18)]
fn input_generator(input: &str) -> Lumber {
    let mut lumber = Lumber::new();
    for line in input.lines() {
        let mut acres = Vec::new();
        for c in line.chars() {
            match c {
                '.' => acres.push(Acre::Ground),
                '#' => acres.push(Acre::LumberYard),
                '|' => acres.push(Acre::Tree),
                _ => unreachable!(),
            }
        }
        lumber.grid.push(acres);
    }
    lumber
}

#[aoc(day18, part1)]
fn solve_part1(lumber: &Lumber) -> usize {
    let mut lumber = lumber.clone();
    println!("Original");
    lumber.print();

    for _ in 0..10 {
        lumber.step();
        lumber.print();
    }

    lumber
        .grid
        .iter()
        .flatten()
        .filter(|&w| *w == Acre::Tree)
        .count()
        * lumber
            .grid
            .iter()
            .flatten()
            .filter(|&w| *w == Acre::LumberYard)
            .count()
}
