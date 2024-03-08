use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<i32> {

    let mut shifts:Vec<i32>=Vec::new();

    for line in input.lines() {
        shifts.push(line.parse::<i32>().unwrap());
    }
    shifts
}

#[aoc(day1, part1)]
fn solve_part1(input: &Vec<i32>) -> i32 {
    input.iter().sum()
}

#[aoc(day1, part2)]
fn solve_part2(input: &Vec<i32>) -> i32 {
    let mut freq:HashSet<i32>=HashSet::new();

    let mut current_value = 0;
    loop {
        for val in input.iter() {
            current_value+=val;
            if ! freq.insert(current_value) {
                return current_value;
            }
        }        
    }
}

