use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::{
    io::{self, Write},
    thread::sleep,
    time,
};

#[aoc_generator(day17)]
fn input_generator(input: &str) -> Reservoir {
    let re = Regex::new(r"^(x|y)=(\d+), (x|y)=(\d+)\.\.(\d+)$").unwrap();
    let mut reservoir = Reservoir::new();

    for line in input.lines() {
        if let Some(caps) = re.captures(line) {
            let axis1 = &caps[1];
            let coord1: usize = caps[2].parse().unwrap();
            let coord2_start: usize = caps[4].parse().unwrap();
            let coord2_end: usize = caps[5].parse().unwrap();

            match axis1 {
                "x" => {
                    for y in coord2_start..=coord2_end {
                        reservoir.grid[y][coord1] = '#';
                    }
                }
                "y" => {
                    for x in coord2_start..=coord2_end {
                        reservoir.grid[coord1][x] = '#';
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    reservoir
}
#[derive(Clone)]
struct Reservoir {
    grid: Vec<Vec<char>>,
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
}
enum Flow {
    Left,
    Right,
    Both,
}

impl Reservoir {
    fn new() -> Self {
        let mut res = Reservoir {
            grid: vec![vec![' '; 2000]; 2000],
            min_x: 0,
            min_y: 0,
            max_x: 0,
            max_y: 0,
        };
        res.grid[0][500] = '+';
        res
    }
    fn compute_size(&mut self) {
        (self.min_x, self.min_y, self.max_x, self.max_y) = self
            .grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &c)| (x, y, c)))
            .filter(|(_, _, c)| *c != ' ')
            .fold(
                (2000, 2000, 0, 0),
                |(min_x, min_y, max_x, max_y), (x, y, _)| {
                    (min_x.min(x), min_y.min(y), max_x.max(x), max_y.max(y))
                },
            );
        println!(
            "{} {} {} {}",
            self.min_x, self.min_y, self.max_x, self.max_y
        );
    }

    fn print_grid(&self, x: usize, y: usize) {
        io::stdout().write(b"\x1B[2J\x1B[1;1H").unwrap();

        let x = (x / 30) as usize * 30;
        let y = (y / 30) as usize * 30;
        println!("x: {}\ty: {}", x, y);
        let delta: usize = 50;
        for y in (0.max(y as i32 - delta as i32) as usize)..2000.min(y + delta) {
            for x in 0.max(x - delta)..2000.min(x + delta) {
                print!("{}", self.grid[y][x]);
            }
            println!();
        }
        println!();
        sleep(time::Duration::from_millis(5));
    }
    fn print_whole_grid(&self) {
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                print!("{}", self.grid[y][x]);
            }
            println!();
        }
        println!();
        sleep(time::Duration::from_millis(5));
    }
    fn is_valid_pos(&self, x: usize, y: usize) -> bool {
        if x < self.min_x || x > self.max_x || y < self.min_y || y > self.max_y {
            return false;
        }
        true
    }
    fn drop_water(&mut self, x: usize, y: usize, flow: Flow) -> Option<usize> {
        if !self.is_valid_pos(x, y) {
            return None;
        }
        //self.print_grid(x, y);
        match self.grid[y][x] {
            '#' | '~' => Some(x),
            '|' => None,
            ' ' => {
                self.grid[y][x] = '|';
                self.drop_water(x, y + 1, Flow::Both)?;
                match flow {
                    Flow::Left => self.drop_water(x - 1, y, Flow::Left),
                    Flow::Right => self.drop_water(x + 1, y, Flow::Right),
                    Flow::Both => {
                        match (
                            self.drop_water(x - 1, y, Flow::Left),
                            self.drop_water(x + 1, y, Flow::Right),
                        ) {
                            (Some(left), Some(right)) => {
                                for i in left + 1..right {
                                    self.grid[y][i] = '~';
                                }
                                Some(x)
                            }
                            _ => None,
                        }
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    fn count_water(&self) -> usize {
        self.grid
            .iter()
            .map(|row| row.iter().filter(|&&c| c == '~' || c == '|').count())
            .sum()
    }
}

#[aoc(day17, part1)]
fn solve_part1(reservoir: &Reservoir) -> usize {
    let mut reservoir = reservoir.clone();
    reservoir.compute_size();
    reservoir.drop_water(500, 1, Flow::Both);
    //reservoir.print_whole_grid();
    reservoir.count_water()
}
