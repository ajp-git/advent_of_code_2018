use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Clone)]
struct Fabric {
    claims: HashSet<u32>,
}


#[derive(Clone)]
struct Claim {
    id: u32,
    x: u32,
    y:u32,
    dx: u32,
    dy: u32,
    overlaps: bool,
}

impl Claim {
    fn new (input: &str) -> Self {

        let r_line=Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        let caps=r_line.captures(&input).unwrap();

        Claim { id: caps[1].parse::<u32>().unwrap(),
             x: caps[2].parse::<u32>().unwrap(),
             y: caps[3].parse::<u32>().unwrap(),
             dx: caps[4].parse::<u32>().unwrap(),
             dy: caps[5].parse::<u32>().unwrap(),
             overlaps: false,
        }
    }
    fn fill_grid(&mut self, mut grid:&mut HashMap<(u32,u32), Fabric>) {
        for i in self.x..(self.x+self.dx) {
            for j in self.y..(self.y+self.dy) {
                if let Some(fabric) = grid.get_mut(&(i,j)) {
                    if fabric.claims.iter().find(|&&v|v!=self.id ).is_some() {
                        self.overlaps=true;
                    }
                    fabric.claims.insert(self.id);
                } else {
                    let mut claims=HashSet::new();
                    claims.insert(self.id);
                    grid.insert((i,j),Fabric{claims});
                }
            }    
        }
     }

}


#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<Claim> {


    let mut claims:Vec<Claim>=Vec::new();
   
    for line in input.lines() {
        claims.push(Claim::new(line));
    }
    claims
}

#[aoc(day3, part1)]
fn solve_part1(input: &Vec<Claim>) -> usize {
    let mut input=input.to_vec();

    let mut grid:HashMap<(u32,u32), Fabric> = HashMap::new();

    input.iter_mut().for_each(|c| c.fill_grid(&mut grid));
    display_grid(&grid);
    grid.iter().filter(|(_, &ref fab)| fab.claims.len()>1).count()
}

fn display_grid(grid: &HashMap<(u32,u32), Fabric>) {
    println!();
    for y in 0..10{
        for x in 0..10{
            let c=grid.get(&(x,y));
            match c {
                None => print!("."),
                Some(fab) => print!("{}", fab.claims.len()),               
            }
        }
        println!();
        
    }
}

#[aoc(day3, part2)]
fn solve_part2(input: &Vec<Claim>) -> u32 {

    let mut input=input.to_vec();
    let mut grid:HashMap<(u32,u32), Fabric> = HashMap::new();

    input.iter_mut().for_each(|c| c.fill_grid(&mut grid));
    input.iter_mut().for_each(|c| c.fill_grid(&mut grid)); //twice to fill the first found

    input.iter().find(|c| !c.overlaps).map(|claim| claim.id).unwrap()
}
