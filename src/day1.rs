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