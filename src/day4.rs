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
impl GuardAction {
    fn get_minute(&self) -> u8 {
        self.date[14..=15].parse::<u8>().unwrap()
    }
    fn get_hour(&self) -> u8 {
        self.date[11..=12].parse::<u8>().unwrap()
    }
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
[1518-11-05 00:55] wakes up"; 
 */
    let mut guard_actions:Vec<GuardAction> = Vec::new();
    let wakesup=    Regex::new(r"\[(.*)\] wakes up").unwrap();
    let fallasleep= Regex::new(r"\[(.*)\] falls asleep").unwrap();
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
        } else  {
            panic!("Bad line {}", line);
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
fn solve_part1(input: &[GuardAction]) -> u32 {
    
    for ga in input.iter() {
        println!("Debug at {:?}", ga);
    }
    let mut guards:HashSet<u32>=HashSet::new();
    let mut sleeps:HashMap<(u32,u8),u32>=HashMap::new();
    let mut is_sleeping:bool=false;
    let mut sleeping_started_minute:u8=0;
    let mut sleeping_started_hour:u8=0;
    let mut current_guard=0;

    for ga in input.iter() {
        guards.insert(ga.guard_id);
        println!("\nLooking at {:?}", ga);
        if ga.action==Action::BeginShift {
            current_guard=ga.guard_id;
            if is_sleeping {
                panic!("No wake up for guard {}", current_guard);
            }
        } else if ga.action==Action::FallAsleep {
            sleeping_started_minute = ga.get_minute();
            sleeping_started_hour = ga.get_hour();
            is_sleeping=true;

        } else { // Action::WakeUp
            if is_sleeping {
                is_sleeping=false;
                println!("For guard {} started sleep time {}:{}", current_guard, sleeping_started_hour, sleeping_started_minute);
                let mut end_minute=ga.get_minute();
                let mut end_hour=ga.get_hour();
                println!("\tend sleep time {}:{}", end_hour, end_minute);
            
                for m in sleeping_started_minute..end_minute {
                    *sleeps.entry((ga.guard_id, m)).or_insert(0)+=1;
                    print!("zzz : {}\t", m);                    
                }
            } else {
                panic!("Wake up without sleep for guard {}", current_guard);
            }
        }
    }
    println!("{:?}", sleeps);
    println!();
    println!("{:?}", guards);

    let mut max_slept_minutes=0;
    let mut max_sleeping_guard=0;
    
    for g in guards.iter(){
        let m = sleeps.iter()
            .filter(|((guard_id,_),_)| guard_id==g)
            .map(|((guard_id, mn), nb)|nb)
            .sum();
        if m > max_slept_minutes{
            max_sleeping_guard=*g;
            max_slept_minutes=m;
        }
    }
    println!("Guard {} slept {}mn", max_sleeping_guard, max_slept_minutes);

    let mut max_mn_sleeping_time=0;
    let mut mn_max=0;
    for i in 0..60 {
        let m = sleeps
            .iter()
            .filter(|((guard_id, mn),count)| guard_id==&max_sleeping_guard && mn==&i)
            .map(|((_,_),count)|count)
            .sum();
        if m>max_mn_sleeping_time {
            max_mn_sleeping_time=m;
            mn_max=i;
        }
    }
    println!("Guard slept the most at mn {}", mn_max);
    mn_max as u32*max_sleeping_guard
}


#[aoc(day4, part2)]
fn solve_part2(input: &[GuardAction]) -> u32 {
    
    for ga in input.iter() {
        println!("Debug at {:?}", ga);
    }
    let mut guards:HashSet<u32>=HashSet::new();
    let mut sleeps:HashMap<(u32,u8),u32>=HashMap::new();
    let mut is_sleeping:bool=false;
    let mut sleeping_started_minute:u8=0;
    let mut sleeping_started_hour:u8=0;
    let mut current_guard=0;

    for ga in input.iter() {
        guards.insert(ga.guard_id);
        println!("\nLooking at {:?}", ga);
        if ga.action==Action::BeginShift {
            current_guard=ga.guard_id;
            if is_sleeping {
                panic!("No wake up for guard {}", current_guard);
            }
        } else if ga.action==Action::FallAsleep {
            sleeping_started_minute = ga.get_minute();
            sleeping_started_hour = ga.get_hour();
            is_sleeping=true;

        } else { // Action::WakeUp
            if is_sleeping {
                is_sleeping=false;
                println!("For guard {} started sleep time {}:{}", current_guard, sleeping_started_hour, sleeping_started_minute);
                let mut end_minute=ga.get_minute();
                let mut end_hour=ga.get_hour();
                println!("\tend sleep time {}:{}", end_hour, end_minute);
            
                for m in sleeping_started_minute..end_minute {
                    *sleeps.entry((ga.guard_id, m)).or_insert(0)+=1;
                    print!("zzz : {}\t", m);                    
                }
            } else {
                panic!("Wake up without sleep for guard {}", current_guard);
            }
        }
    }
    println!("{:?}", sleeps);
    println!();
    println!("{:?}", guards);

    let mut max_mn_sleeping_time=0;
    let mut guard_max=0;
    let mut mn_max=0;
    for i in 0..60 {
        for guard in guards.iter(){
            let m = sleeps
            .iter()
            .filter(|((guard_id, mn),count)| guard_id==guard && mn==&i)
            .map(|((_,_),count)|count)
            .sum();

            if m>max_mn_sleeping_time {
                max_mn_sleeping_time=m;
                guard_max=*guard;
                mn_max=i;
            }

        }
    }
    println!("Guard {} slept the most at mn {}", guard_max, mn_max);
    guard_max as u32*mn_max as u32
}