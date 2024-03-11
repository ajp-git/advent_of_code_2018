use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<String> {

    let mut occ:Vec<String> = Vec::new();
    for line in input.lines() {
        occ.push(line.to_string());
    }
    occ
}

#[aoc(day2, part1)]
fn solve_part1(input: &Vec<String>) -> usize {
    
    let mut count2=0;
    let mut count3=0;

    for line in input.iter() {
        let mut occ:HashMap<char,u32>=HashMap::new();
        for c in line.chars() {
            *occ.entry(c).or_insert(0)+=1;
        }
        count2+=if occ.iter().filter(|(_,&v)| v==2).count() >0 {1} else {0};
        count3+=if occ.iter().filter(|(_,&v)| v==3).count() >0 {1} else {0};
    }
    count2*count3
}