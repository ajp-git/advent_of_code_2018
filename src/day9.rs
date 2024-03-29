use std::collections::{HashMap, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
fn input_generator(input: &str) -> (u32,u32) {
 //(9,25) // 32 high score
 //(10,1618) // 8317 
// (411, 71058)
 (411, 7105800)
}

#[aoc(day9, part1)]
fn solve_part1(input: &(u32, u32)) -> u32 {

    let players=input.0;
    let marbles=input.1;

    let mut scores = HashMap::new();
    let mut circle = VecDeque::new();

    circle.push_back(0);

    for marble in 1..=marbles {
        if marble%23==0{
            circle.rotate_right(7);
            let player=marble%players;
            *scores.entry(player).or_insert(0)+=marble+circle.pop_back().unwrap();
            circle.rotate_left(1);
        } else {
            circle.rotate_left(1);
            circle.push_back(marble);
        }
    }
    *scores.values().max().unwrap_or(&0)
}

