use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day8)]
fn input_generator(input: &str) -> Vec<u8> {

    let input="2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    let numbers=input.split_whitespace();
    
    numbers.into_iter().map(|n| n.parse::<u8>().unwrap() ).collect::<Vec<u8>>()
}

#[aoc(day8, part1)]
fn solve_part1(input: &Vec<u8>) -> u32 {
    get_metadata_sum(input, 0).0
}

fn get_metadata_sum(input: &Vec<u8>, index:usize ) -> (u32,usize) {
    
    let mut index=index;
    let mut metadata_sum:u32=0;

    let children_numbers=input[index];
    index+=1;

    let metadata_numbers= input[index];
    index+=1;
    for _ in 0..children_numbers {
        let result= get_metadata_sum(input, index);
        metadata_sum+=result.0;
        index=result.1;
    }
    for _ in 0..metadata_numbers{
        metadata_sum+=input[index] as u32;
        index+=1;
    }
    (metadata_sum,index)
}