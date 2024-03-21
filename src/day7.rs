use std::collections::{HashMap, HashSet};
use std::iter::Iterator;

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
#[derive(Debug,Clone)]
struct Step {
    before:Vec<char>,
    after:Vec<char>,
    done: bool,
}

#[derive(Debug, Clone)]
struct Instructions {
    steps:HashMap<char,Step>,
    current_step: char,
    ordered_step:HashSet<char>,
}

impl Instructions {
    fn find_first(self: &Self) -> char {
        self.steps.iter().find(|(c,step)| step.before.is_empty()).map(|(&c,_)|c).unwrap()
    }
    fn is_finished(self: &Self) -> bool{
        self.steps.iter().find(|(c, s)|s.done==false).is_none()
    }
    fn new (steps: &HashMap<char, Step>) -> Self {
        Instructions {
            steps: steps.clone(),
            ordered_step: HashSet::new(),
            current_step:' ',
        }
    }

    fn set_done(self:&mut Self, step_key: char) {
        if let Some(mut step)=self.steps.get_mut(&step_key){
            step.done=true;            
        }
    }
    
    fn set_done_current(self:&mut Self) {
        self.set_done(self.current_step);
    }


    fn change_current(self: &mut Self, c: char) {

        self.set_done_current();
        self.current_step=c;
        let res= self
            .steps.iter()
            .find(|(&k, step)|k==self.current_step).unwrap();

        for &c in res.1.after.iter(){
            self.ordered_step.insert(c);
        }
    }

    fn next(self: &mut Self) -> Option<char> {

        self.ordered_step.remove(&self.current_step);
        if ! self.ordered_step.is_empty() {
            let mut v_steps:Vec<char>=self.ordered_step.clone().into_iter().collect();
            v_steps.sort();
            
            for s in v_steps.iter() {
                let mut all_previous_done=true;
                let step=self.steps.get(s).unwrap();
                // For each possible, take the first having previous all done
                for prev_step in step.before.iter() {
                    if let Some(prev_step)=self.steps.get(&prev_step) {
                        if ! prev_step.done {
                            all_previous_done=false;
                            break;
                        }
                    }
                }

                if all_previous_done {
                    self.change_current(*s);
                    return Some(*s);
                }
            }
        }
        None
    }
    
}
/*
struct InstructionsIter<'a> {
    iter:std::collections::hash_map::Values<'a, char, Step>,
}

impl<'a> Iterator for InstructionsIter<'a>  {
    type Item = &'a Step;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl Instructions {
    fn iter(&self) -> InstructionsIter {
        InstructionsIter { 
            iter: self.steps.values(),
        }
    }
}*/

#[aoc_generator(day7)]
fn input_generator(input: &str) -> HashMap<char,Step> {

    let mut steps:HashMap<char, Step>=HashMap::new();

    /*let input="Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";*/

    let re=Regex::new(r"Step (\w) must be finished before step (\w) can begin.").unwrap();

    for line in input.lines() {
        let caps=re.captures(line).unwrap();
        let before=caps[1].chars().next().unwrap();
        let after=caps[2].chars().next().unwrap();

        //Step A must be finished before step B can begin.
        // A after : B 
        // B before : A
        if let Some(step) = steps.get_mut(&before) {
            step.after.push(after);
        } else {
            steps.insert(before, Step{before: Vec::new() ,after:vec![after], done:false});
        }

        if let Some(step) = steps.get_mut(&after) {
            step.before.push(before);
        } else {
            steps.insert(after, Step{before: vec![before],after:Vec::new(),done:false });
        }
    }

//    dbg!(&steps);
    steps
}

/*#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_structure () -> HashMap<char, Step> {
        let mut steps:HashMap<char, Step>=HashMap::new();
        steps.insert('A', Step { before: Vec::new(), after: vec!['B','C'], done: false });
        steps.insert('B', Step { before: vec!['A'], after: vec!['D'], done: false });
        steps.insert('C', Step { before: vec!['A'], after: Vec::new(), done: false });
        steps.insert('D', Step { before: vec!['B'], after: Vec::new(), done: false });

        steps
    }

    #[test]
    fn test_step_creation() {
        let steps = create_test_structure();
        dbg!(steps);
        assert!(0==0)
    }
}*/

#[aoc(day7, part1)]
fn solve_part1(input: &HashMap<char, Step>) -> String {

    let mut result:String=String::new();
    let mut instructions=Instructions::new(input);

    let first=instructions.find_first();
    println!("First step is {}", first);
    instructions.change_current(first);
    result.push(first);
    loop {
        
        if instructions.is_finished(){
            break;
        }
        instructions.set_done_current();

        if let Some(ins)=instructions.next() {
            result.push(ins);
            println!("Handling step {}", ins);
        }

    }
    result
}

#[aoc(day7, part2)]
fn solve_part2(input: &HashMap<char, Step>) -> i32 {
0
}

