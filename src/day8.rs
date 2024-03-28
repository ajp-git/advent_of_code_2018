use std::fs::Metadata;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day8)]
fn input_generator(input: &str) -> Vec<u8> {

//   let input="2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    let numbers=input.split_whitespace();
    
    numbers.into_iter().map(|n| n.parse::<u8>().unwrap() ).collect::<Vec<u8>>()
}

#[aoc(day8, part1)]
fn solve_part1(input: &Vec<u8>) -> u32 {
    get_metadata_sum_part1(input, 0).0
}

fn get_metadata_sum_part1(input: &Vec<u8>, index:usize ) -> (u32,usize) {
    
    let mut index=index;
    let mut metadata_sum:u32=0;

    let children_numbers=input[index];
    index+=1;

    let metadata_numbers= input[index];
    index+=1;
    for _ in 0..children_numbers {
        let result= get_metadata_sum_part1(input, index);
        metadata_sum+=result.0;
        index=result.1;
    }
    for _ in 0..metadata_numbers{
        metadata_sum+=input[index] as u32;
        index+=1;
    }
    (metadata_sum,index)
}


#[aoc(day8, part2)]
fn solve_part2(input: &Vec<u8>) -> u32 {
    get_metadata_sum_part2(input, 0,0).0
}

fn get_metadata_sum_part2(input: &Vec<u8>, index:usize, level:usize ) -> (u32,usize) {
    
    let mut index=index;
    let mut metadata_sum:u32=0;
    let mut child_value:Vec<u32>=Vec::new();
    
    let mut ident="-- ".repeat(level);
    ident=format!("{} > ", ident);
    println!("{} Entering",ident);
    let children_numbers=input[index];
    index+=1;

    let metadata_numbers= input[index];
    index+=1;

    println!("{}Found {} children and {} meta", ident, children_numbers, metadata_numbers);
        
    for _ in 0..children_numbers {
        let (sum, new_index) = get_metadata_sum_part2(input, index, level+1);
        index=new_index;
        child_value.push(sum);            
    }
    println!("{ident} children values:{child_value:?}");
    for i in 0..metadata_numbers {
        let meta_val=input[index];
        println!("{ident} metadata {i} : {meta_val}");
        if children_numbers>0 {
            if meta_val<= children_numbers {
                println!("{ident} Adding child_value[{}] {}", meta_val-1, child_value[meta_val as usize -1]);
                metadata_sum+=child_value[meta_val as usize -1];
            } else {
                println!("{ident} child_value[{}] out of scope", meta_val-1);
            }
        } else {
            metadata_sum+=meta_val as u32;
        }
        index+=1;
    }

    println!("{ident} Returning {}",metadata_sum);
    (metadata_sum,index)
}
