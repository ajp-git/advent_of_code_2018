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


#[aoc(day2, part2)]
fn solve_part2(input: &Vec<String>) -> String {

    for x in input.iter(){
        for y in input.iter() {
            let mut diff=0;
            let mut out:String=String::new();
            for (c1, c2) in x.chars().zip(y.chars()) {
                if c1!=c2 { 
                    diff+=1
                }
                else {
                    out.push(c1);
                } 
            }
            if diff==1 {
                println!("1 char diff between {} and {}", x, y);
                return out;
            }
        }   
    }
    "".to_string()
}