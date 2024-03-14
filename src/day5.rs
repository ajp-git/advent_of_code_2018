use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Vec<u8> {
    //let input="dabAcCaCBAcCcaDA";
    input.to_string().chars().map(|c| c as u8).collect()
}

#[aoc(day5, part1)]
fn solve_part1(input: &Vec<u8>) -> u32 {

    let mut input = input.clone();
    let mut input_tmp = input.clone();
    let mut last_len=0;
    println!();
    loop {
        input=input_tmp.clone();
        input_tmp.clear();

        let curr_len=input.len();
        if last_len==curr_len {
            println!("\nNo change in len, breaking");
            break;
        }
        print!("\rlen :{}", curr_len);
        last_len=curr_len;

        let mut iter = input.iter().peekable();
        
        loop {
            let c1 = iter.next();
            let c2=iter.peek();
            if c2.is_none() { 
                if c1.is_some() {
                    input_tmp.push(*c1.unwrap());
                }
                break;
            }
            let c1=*c1.unwrap();
            let c2=**c2.unwrap();
//            println!("Comparing {} and {}", c1 as char, c2 as char);
            if !(c1==c2+32 || c2==c1+32) {
                input_tmp.push(c1);
            } else {
//                println!("Removing {} and {}", c1 as char, c2 as char);
                iter.next();
            }
        }
//        println!("New String :{:?}",input.iter().map(|c|*c as char).collect::<Vec<char>>());
    }
    println!();
    last_len as u32
}



#[aoc(day5, part2)]
fn solve_part2(input: &Vec<u8>) -> u32 {

    let mut min_polymer=usize::MAX;

    let mut input_original = input.clone();
    for x in 0..26 {
        let mut input=input_original
            .iter()
            .filter(|&&c|c!=b'a'+x && c!=b'A'+x)
            .map(|c| *c)
            .collect::<Vec<u8>>();

        let mut input_tmp = input.clone();
        let mut last_len=0;
        println!();
        loop {
            input=input_tmp.clone();
            input_tmp.clear();

            let curr_len=input.len();
            if last_len==curr_len {
                println!("\nNo change in len, breaking");
                break;
            }
            print!("\rlen :{}", curr_len);
            last_len=curr_len;

            let mut iter = input.iter().peekable();
            
            loop {
                let c1 = iter.next();
                let c2=iter.peek();
                if c2.is_none() { 
                    if c1.is_some() {
                        input_tmp.push(*c1.unwrap());
                    }
                    break;
                }
                let c1=*c1.unwrap();
                let c2=**c2.unwrap();
    //            println!("Comparing {} and {}", c1 as char, c2 as char);
                if !(c1==c2+32 || c2==c1+32) {
                    input_tmp.push(c1);
                } else {
    //                println!("Removing {} and {}", c1 as char, c2 as char);
                    iter.next();
                }
            }
    //        println!("New String :{:?}",input.iter().map(|c|*c as char).collect::<Vec<char>>());
        }
        if last_len < min_polymer {
            min_polymer=last_len;
        }
    }
    println!();
    min_polymer as u32
}
