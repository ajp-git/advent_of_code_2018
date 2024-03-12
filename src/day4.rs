use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
/*
ins shift
[1518-03-24 00:50] wakes up
[1518-11-12 00:47] wakes up
[1518-08-18 00:42] falls asleep
[1518-04-24 00:49] wakes up
*/
#[derive(Debug, PartialEq)]
enum Action {
    WakesUp,
    FallAsleep,
    BeginShift,
}
#[derive(Debug)]
struct GuardAction{
    date:String,
    guard_id:u32,
    action:Action,
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<GuardAction> {
/*
let input="[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up"; */

    let mut guard_actions:Vec<GuardAction> = Vec::new();
    let wakesup=    Regex::new(r"\[(.*)\] wakes up").unwrap();
    let fallasleep= Regex::new(r"\[(.*)\] fall asleep").unwrap();
    let beginshift= Regex::new(r"\[(.*)\] Guard #(\d+) begins shift").unwrap();

    for line in input.lines() {
        
        if let Some(caps) = wakesup.captures(line) {
            let mut ga = GuardAction { date:caps[1].to_string(), guard_id:0, action: Action::WakesUp };
            guard_actions.push(ga);
        } else if  let Some(caps) = fallasleep.captures(line) {
            let mut ga = GuardAction { date:caps[1].to_string(), guard_id:0, action: Action::FallAsleep };
            guard_actions.push(ga);            
        } else if  let Some(caps) = beginshift.captures(line) {
            let mut ga = GuardAction { date:caps[1].to_string(), guard_id:caps[2].parse::<u32>().unwrap(), action: Action::BeginShift };
            guard_actions.push(ga);            
        }
    }
    guard_actions.sort_by(|a,b| a.date.cmp(&b.date));
    let mut last_guard=0;
    for ga in guard_actions.iter_mut() {
        if ga.action==Action::BeginShift {
            last_guard=ga.guard_id
        } else {
            ga.guard_id=last_guard;
        }
    }
//    println!("{:?}", guard_actions);
    guard_actions
}

#[aoc(day4, part1)]
fn solve_part1(input: &[GuardAction]) -> usize {
    0
}