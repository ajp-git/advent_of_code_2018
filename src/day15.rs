use std::{cmp::max, cmp::min, collections::HashMap};

use aoc_runner_derive::{aoc, aoc_generator};

enum Grid {
    Wall,
    Open,
    Goblin(u32),
    Elf(u32),
}
struct Cave {
    grid:HashMap<(usize,usize),Grid>,
    max_x:usize,
    max_y:usize,
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

        let max_x=*grid.keys().map(|(x,_)|x).max().unwrap();
        let max_y=*grid.keys().map(|(_,y)|y).max().unwrap();
       
        Cave { grid, max_x, max_y }
    }

    fn print(&self) {

        for y in 0..=self.max_y {
            for x in 0..=self.max_x{
                print!("{}",
                match self.grid.get(&(x,y)).unwrap() {
                    Grid::Wall => '#',
                    Grid::Elf(v) => 'E',
                    Grid::Goblin(v) => 'G',
                    Grid::Open => '.',
                });
            }
            println!();
        }
    }

    fn turn(self: &mut Self){

        // starting from top left
        // for each G ot E
        // find nearest target
        // step to it

        for y in 0..=self.max_y{
            for x in 0..=self.max_x{
                let current=self.grid.get(&(x,y)).unwrap();
                match current {
                    Grid::Elf(_) => {},
                    Grid::Goblin(_) => {},
                    _ => {},
                }
            }
        }
    }

    fn get_nearest_target(&self, x:usize, y:usize) -> Option<(usize,usize)> {

        let mut x_dist=usize::MAX;
        let mut y_dist=usize::MAX;

        let current=self.grid.get(&(x,y)).unwrap().clone();
        let target:Grid;

        match current {
            Grid::Elf(_) => target=Grid::Goblin(0),
            Grid::Goblin(_) => target=Grid::Elf(0),
            _ => panic!("Impossible current"),
            
        }

        let circle_size=max(
            x.max(self.max_x-x),
            y.max(self.max_y-y));


        // turn around 
        // (-1,-1) (0,-1)  (1,-1)
        // (-1, 0) (x,y)   (1,0)
        // (-1, 1) (0,1)   (1,1)

        for i in 1..circle_size{
            for nx in 0.max(x-i)..self.max_x.min(x+i) {
                for ny in 0.max(x-i)..self.max_x.min(x+i) {
                    match (current,self.grid.get(&(nx,ny)).unwrap()) {
                        (Grid::Elf(_), Grid::Goblin(_)) => if self.is_accessible_target(x, y, nx, ny) { return Some((nx, ny)); }, 
                        (Grid::Goblin(_), Grid::Elf(_)) => if self.is_accessible_target(x, y, nx, ny) { return Some((nx, ny)); }, 
                        _ =>{},
                    }
                }   
            }
        }
        None
    }

    fn is_accessible_target(&self, src_x:usize, src_y:usize, target_x:usize, target_y:usize) -> bool {
        // is there an open path ?
        true
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

