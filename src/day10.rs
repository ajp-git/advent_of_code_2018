use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Clone)]
struct Star {
    position_x:i32,
    position_y:i32,
    velocity_x:i32,
    velocity_y:i32,
}

#[aoc_generator(day10)]
fn input_generator(input: &str) -> Vec<Star> {

    /*let input="position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";*/

    let mut stars:Vec<Star>=Vec::new();

    let re=Regex::new(r"position=<\s*(-?\d+),\s*(-?\d+)>\s*velocity=<\s*(-?\d+),\s*(-?\d+)>").unwrap();
    for line in input.lines() {
        //println!("{line}");
        let caps=re.captures(line).unwrap();
        stars.push(
            Star { 
                position_x: caps[1].parse::<i32>().unwrap(),
                position_y: caps[2].parse::<i32>().unwrap(), 
                velocity_x: caps[3].parse::<i32>().unwrap(), 
                velocity_y: caps[4].parse::<i32>().unwrap() }
        )
    }
    stars
 
}

#[aoc(day10, part1)]
fn solve_part1(input: &Vec<Star>) -> String {

    let mut min_x_size=i32::MAX;
    let mut min_y_size=i32::MAX;
    let mut min_x=0;
    let mut min_y=0;
    let mut max_x=0;
    let mut max_y=0;

    let mut min_step=0;
    let mut counter=0;

    let mut stars:Vec<Star> = input.clone();

    loop {
        stars.iter_mut().for_each(|star|
            {
                star.position_x+=star.velocity_x;
                star.position_y+=star.velocity_y;
            }
        );
        let current_min_x= stars.iter().map(|star| star.position_x).min().unwrap();
        let current_min_y= stars.iter().map(|star| star.position_y).min().unwrap();
        let current_max_x= stars.iter().map(|star| star.position_x).max().unwrap();
        let current_max_y= stars.iter().map(|star| star.position_y).max().unwrap();

        if current_max_x-current_min_x <= min_x_size && current_max_y-current_min_y <= min_y_size {
            min_x_size=current_max_x-current_min_x;
            min_y_size=current_max_y-current_min_y;
            min_step=counter;
            min_x=current_min_x;
            min_y=current_min_y;
            max_x=current_max_x;
            max_y=current_max_y;
        }
        counter+=1;

        print!("\r{} - {}", counter, min_step);
        if counter>10910{
            break;

        }
        println!("Min is {}", min_step);
    }
    println!();
    let mut stars:Vec<Star> = input.clone();

    for _ in 0..=min_step {
        stars.iter_mut().for_each(|star|
            {
                star.position_x+=star.velocity_x;
                star.position_y+=star.velocity_y;
            }
        );
    }
    let lights:Vec<(i32,i32)>=stars.iter().map(|star|(star.position_x,star.position_y)).collect();
    for y in min_y..=max_y {
        for x in min_x..max_x {
            if lights.iter().any(|l|l.0==x && l.1==y) {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }    

    "0".to_string()
}

