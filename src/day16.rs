use std::{
    collections::{HashMap, HashSet},
    process::id,
};

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
struct Operation {
    name: String,
    id: u8,
}

#[derive(Debug)]
struct TrainData {
    /*
       Before: [0, 2, 2, 2]
       11 3 3 3
       After:  [0, 2, 2, 0]
    */
    before: Vec<u8>,
    values: Vec<u8>,
    after: Vec<u8>,
}
#[derive(Debug)]
struct RealData {
    values: Vec<u8>,
}
#[derive(Debug, Clone, PartialEq)]
struct Possibilities {
    id: u8,
    ids: HashSet<String>,
}

#[aoc_generator(day16)]
fn input_generator(input: &str) -> (Vec<TrainData>, Vec<RealData>) {
    let mut train: Vec<TrainData> = Vec::new();
    let mut data: Vec<RealData> = Vec::new();

    let mut lines = input.lines();
    let re_values = Regex::new(r"(\d+) (\d+) (\d+) (\d+)").unwrap();

    while let Some(line) = lines.next() {
        if line.starts_with(&"Before") {
            let re_before = Regex::new(r"Before: \[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
            let values_before = re_before.captures(&line);
            if values_before.is_none() {
                println!("{} not regexed", line);
            }
            let values_before = values_before.unwrap();
            let line = lines.next().unwrap();
            let values = re_values.captures(&line).unwrap();

            let line = lines.next().unwrap();
            let re_after = Regex::new(r"After:  \[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
            let values_after = re_after.captures(&line).unwrap();
            lines.next();

            train.push(TrainData {
                before: (1..=4)
                    .map(|i| values_before[i].parse::<u8>().unwrap())
                    .collect(),
                values: (1..=4).map(|i| values[i].parse::<u8>().unwrap()).collect(),
                after: (1..=4)
                    .map(|i| values_after[i].parse::<u8>().unwrap())
                    .collect(),
            });
        } else {
            if line.to_string().len() > 0 {
                let values = re_values.captures(&line).unwrap();
                data.push(RealData {
                    values: (1..=4).map(|i| values[i].parse::<u8>().unwrap()).collect(),
                });
            }
        }
    }
    (train, data)
}

fn get_operation_name(operations: &Vec<Operation>, id: u8) -> String {
    let op = operations.iter().find(|op| op.id == id).unwrap();
    op.name.clone()
}

fn execute_op(regs: &Vec<u32>, operations: &Vec<Operation>, command: &Vec<u8>) -> Vec<u32> {
    let mut new_regs = regs.clone();
    let command: Vec<usize> = command.iter().map(|v| *v as usize).collect();
    let operation = operations
        .iter()
        .find(|operation| operation.id == command[0] as u8)
        .expect("msg: operation not found {command}");

    println!("Executing {}", operation.name);

    match operation.name.as_str() {
        "addr" => {
            new_regs[command[3]] = new_regs[command[1]] + new_regs[command[2]];
        }
        "addi" => {
            new_regs[command[3]] = new_regs[command[1]] + command[2] as u32;
        }
        "mulr" => {
            new_regs[command[3]] = new_regs[command[1]] * new_regs[command[2]];
        }
        "muli" => {
            new_regs[command[3]] = new_regs[command[1]] * command[2] as u32;
        }
        "banr" => {
            new_regs[command[3]] = new_regs[command[1]] & new_regs[command[2]];
        }
        "bani" => {
            new_regs[command[3]] = new_regs[command[1]] & command[2] as u32;
        }
        "borr" => {
            new_regs[command[3]] = new_regs[command[1]] | new_regs[command[2]];
        }
        "bori" => {
            new_regs[command[3]] = new_regs[command[1]] | command[2] as u32;
        }
        "setr" => {
            new_regs[command[3]] = new_regs[command[1]];
        }
        "seti" => {
            new_regs[command[3]] = command[1] as u32;
        }
        "gtir" => {
            new_regs[command[3]] = if command[1] as u32 > new_regs[command[2]] {
                1
            } else {
                0
            };
        }
        "gtri" => {
            new_regs[command[3]] = if new_regs[command[1]] > command[2] as u32 {
                1
            } else {
                0
            };
        }
        "gtrr" => {
            new_regs[command[3]] = if new_regs[command[1]] > new_regs[command[2]] {
                1
            } else {
                0
            };
        }
        "eqir" => {
            new_regs[command[3]] = if command[1] as u32 == new_regs[command[2]] {
                1
            } else {
                0
            };
        }
        "eqri" => {
            new_regs[command[3]] = if new_regs[command[1]] == command[2] as u32 {
                1
            } else {
                0
            };
        }
        "eqrr" => {
            new_regs[command[3]] = if new_regs[command[1]] == new_regs[command[2]] {
                1
            } else {
                0
            };
        }

        _ => panic!("Unknown command {:?}", operation.name),
    }
    new_regs
}

fn simulate(train: &Vec<TrainData>) -> (usize, Vec<Operation>, Vec<Possibilities>) {
    let mut operations: Vec<Operation> = vec![
        ("addr", 0u8),
        ("addi", 1),
        ("mulr", 2),
        ("muli", 3),
        ("banr", 4),
        ("bani", 5),
        ("borr", 6),
        ("bori", 7),
        ("setr", 8),
        ("seti", 9),
        ("gtir", 10),
        ("gtri", 11),
        ("gtrr", 12),
        ("eqir", 13),
        ("eqri", 14),
        ("eqrr", 15),
    ]
    .iter()
    .map(|(name, id)| Operation {
        name: name.to_string(),
        id: *id,
    })
    .collect();

    let mut regs: Vec<u32> = vec![0; 4];
    let mut posssibilities: Vec<Possibilities> = (0..16)
        .map(|i| Possibilities {
            id: i,
            ids: HashSet::new(),
        })
        .collect();

    for (index, train_line) in train.iter().enumerate() {
        // assign registers
        train_line
            .before
            .iter()
            .enumerate()
            .for_each(|(i, &v)| regs[i] = v as u32);

        for op in 0..16 {
            let mut train_line_updated = TrainData {
                before: train_line.before.clone(),
                values: vec![
                    op,
                    train_line.values[1],
                    train_line.values[2],
                    train_line.values[3],
                ],
                after: train_line.after.clone(),
            };
            train_line_updated.values[0] = op;
            let new_regs = execute_op(&regs, &operations, &train_line_updated.values);
            if train_line
                .after
                .iter()
                .enumerate()
                .filter(|(i, &v)| new_regs[*i] == v as u32)
                .count()
                == 4
            {
                let mut pos: &mut Possibilities = posssibilities
                    .get_mut(train_line.values[0] as usize)
                    .unwrap();
                pos.ids.insert(get_operation_name(&operations, op));
            }
        }
    }

    let l = train
        .iter()
        .filter(|line| posssibilities[line.values[0] as usize].ids.len() >= 3)
        .count();

    (l, operations, posssibilities)
}

#[aoc(day16, part1)]
fn solve_part1((train, values): &(Vec<TrainData>, Vec<RealData>)) -> usize {
    simulate(train).0
    //println!("Training : {:?}", train);
    //println!("Data : {:?}", values);
}
#[aoc(day16, part2)]
fn solve_part2((train, values): &(Vec<TrainData>, Vec<RealData>)) -> u32 {
    let (_, operations, possibilities) = simulate(train);

    let mut possibilities = possibilities.clone();
    for pos in possibilities.iter() {
        println!("Pos id : {} : {:?}", pos.id, pos.ids);
    }
    loop {
        let to_remove: Vec<String> = possibilities
            .iter()
            .filter(|p| p.ids.len() == 1)
            .map(|f| f.ids.iter().last().unwrap().clone())
            .collect();
        println!("Removing {:?}", to_remove);
        if to_remove.len() == 16 {
            break;
        }
        for remove in to_remove.iter() {
            for pos in possibilities.iter_mut() {
                if pos.ids.len() > 1 && pos.ids.contains(remove) {
                    println!("Removing {remove} from {pos:?}");
                    pos.ids.retain(|id| id != remove);
                }
            }
        }
    }
    let operations: Vec<Operation> = operations
        .iter()
        .zip(possibilities.iter())
        .map(|(op, pos)| {
            let name = pos.ids.iter().next().unwrap();
            Operation {
                id: op.id,
                name: name.clone(),
            }
        })
        .collect();

    for op in operations.iter() {
        println!("Name : {}, id = {}", op.name, op.id);
    }
    println!("Operations : {:?}", operations);

    let mut regs = vec![0u32; 4];
    values.iter().for_each(|v| {
        regs = execute_op(&regs, &operations, &v.values);
        println!("After operation regs are {:?}", regs);
        /*if let Some(op) = operations.iter().find(|x| x.id == v.values[0]) {
            println!("Op : {:?}\tline : {:?}\tregs {:?}", op.name, v.values, regs);
        }*/
    });
    //println!("Training : {:?}", train);
    //println!("Data : {:?}", values);

    regs[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addr() {
        let regs = vec![1, 2, 3, 4];
        let operations = vec![Operation {
            name: "addr".to_string(),
            id: 1,
        }];
        let command = vec![1, 1, 2, 3];
        let expected = vec![1, 2, 3, 5];
        assert_eq!(execute_op(&regs, &operations, &command), expected);
    }

    #[test]
    fn test_addi() {
        let regs = vec![1, 2, 3, 4];
        let operations = vec![Operation {
            name: "addi".to_string(),
            id: 1,
        }];
        let command = vec![1, 1, 2, 3];
        let expected = vec![1, 2, 5, 4];
        assert_eq!(execute_op(&regs, &operations, &command), expected);
    }

    #[test]
    fn test_mulr() {
        let regs = vec![1, 2, 3, 4];
        let operations = vec![Operation {
            name: "mulr".to_string(),
            id: 1,
        }];
        let command = vec![1, 1, 2, 3];
        let expected = vec![1, 2, 3, 6];
        assert_eq!(execute_op(&regs, &operations, &command), expected);
    }

    #[test]
    fn test_muli() {
        let regs = vec![1, 2, 3, 4];
        let operations = vec![Operation {
            name: "muli".to_string(),
            id: 1,
        }];
        let command = vec![1, 1, 2, 3];
        let expected = vec![1, 2, 6, 4];
        assert_eq!(execute_op(&regs, &operations, &command), expected);
    }

    #[test]
    fn test_banr() {
        let regs = vec![1, 2, 3, 4];
        let operations = vec![Operation {
            name: "banr".to_string(),
            id: 1,
        }];
        let command = vec![1, 1, 2, 3];
        let expected = vec![1, 2, 3, 2];
        assert_eq!(execute_op(&regs, &operations, &command), expected);
    }

    #[test]
    fn test_bani() {
        let regs = vec![1, 2, 3, 4];
        let operations = vec![Operation {
            name: "bani".to_string(),
            id: 1,
        }];
        let command = vec![1, 1, 2, 3];
        let expected = vec![1, 2, 3, 0];
        assert_eq!(execute_op(&regs, &operations, &command), expected);
    }

    #[test]
    fn test_borr() {
        let regs = vec![1, 2, 3, 4];
        let operations = vec![Operation {
            name: "borr".to_string(),
            id: 1,
        }];
        let command = vec![1, 1, 2, 3];
        let expected = vec![1, 2, 3, 3];
        assert_eq!(execute_op(&regs, &operations, &command), expected);
    }

    #[test]
    fn test_bori() {
        let regs = vec![1, 2, 3, 4];
        let operations = vec![Operation {
            name: "bori".to_string(),
            id: 1,
        }];
        let command = vec![1, 1, 2, 3];
        let expected = vec![1, 2, 3, 3];
        assert_eq!(execute_op(&regs, &operations, &command), expected);
    }

    #[test]
    fn test_setr() {
        let regs = vec![1, 2, 3, 4];
        let operations = vec![Operation {
            name: "setr".to_string(),
            id: 1,
        }];
        let command = vec![1, 1, 0, 3];
        let expected = vec![1, 2, 3, 2];
        assert_eq!(execute_op(&regs, &operations, &command), expected);
    }

    #[test]
    fn test_seti() {
        let regs = vec![1, 2, 3, 4];
        let operations = vec![Operation {
            name: "seti".to_string(),
            id: 1,
        }];
        let command = vec![1, 1, 0, 3];
        let expected = vec![1, 2, 3, 1];
        assert_eq!(execute_op(&regs, &operations, &command), expected);
    }

    #[test]
    fn test_gtir() {
        let regs = vec![1, 2, 3, 4];
        let operations = vec![Operation {
            name: "gtir".to_string(),
            id: 1,
        }];
        let command = vec![1, 3, 2, 2];
        let expected = vec![1, 2, 3, 1];
        assert_eq!(execute_op(&regs, &operations, &command), expected);
    }

    #[test]
    fn test_gtri() {
        let regs = vec![1, 2, 3, 4];
        let operations = vec![Operation {
            name: "gtri".to_string(),
            id: 1,
        }];
        let command = vec![1, 1, 2, 2];
        let expected = vec![1, 2, 3, 1];
        assert_eq!(execute_op(&regs, &operations, &command), expected);
    }

    #[test]
    fn test_gtrr() {
        let regs = vec![1, 2, 3, 4];
        let operations = vec![Operation {
            name: "gtrr".to_string(),
            id: 1,
        }];
        let command = vec![1, 1, 2, 2];
        let expected = vec![1, 2, 3, 1];
        assert_eq!(execute_op(&regs, &operations, &command), expected);
    }

    #[test]
    fn test_eqir() {
        let regs = vec![1, 2, 3, 4];
        let operations = vec![Operation {
            name: "eqir".to_string(),
            id: 1,
        }];
        let command = vec![1, 2, 2, 2];
        let expected = vec![1, 2, 3, 1];
        assert_eq!(execute_op(&regs, &operations, &command), expected);
    }

    #[test]
    fn test_eqri() {
        let regs = vec![1, 2, 3, 4];
        let operations = vec![Operation {
            name: "eqri".to_string(),
            id: 1,
        }];
        let command = vec![1, 1, 2, 2];
        let expected = vec![1, 2, 3, 1];
        assert_eq!(execute_op(&regs, &operations, &command), expected);
    }

    #[test]
    fn test_eqrr() {
        let regs = vec![1, 2, 3, 4];
        let operations = vec![Operation {
            name: "eqrr".to_string(),
            id: 1,
        }];
        let command = vec![1, 1, 2, 2];
        let expected = vec![1, 2, 3, 1];
        assert_eq!(execute_op(&regs, &operations, &command), expected);
    }
}
