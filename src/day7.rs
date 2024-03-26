use std::collections::{HashMap, HashSet};
use std::iter::Iterator;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;
#[derive(Debug,Clone)]
struct Step {
    before:Vec<char>,
    after:Vec<char>,
    done: bool,
    assigned:bool,
    counter:u32,
}

#[derive(Debug, Clone)]
struct Instructions {
    steps:HashMap<char,Step>,
    current_step: char,
    ordered_step:HashSet<char>,
    elves:HashMap<u32,char>,
    delay:u32,
    global_counter:u32
}

impl Instructions {
    fn find_first(self: &Self) -> char {
        self.steps.iter().find(|(c,step)| step.before.is_empty()).map(|(&c,_)|c).unwrap()
    }
    fn is_finished(self: &Self) -> bool{
        self.steps.iter().find(|(c, s)|s.done==false).is_none()
    }
    fn new (steps: &HashMap<char, Step>, elves_number:u32, delay: u32) -> Self {

        let mut elves:HashMap<u32,char>=HashMap::new();
        for i in 0..elves_number{
            elves.insert(i, ' ');
        }
        Instructions {
            steps: steps.clone(),
            ordered_step: HashSet::new(),
            current_step:' ',
            elves:elves,
            delay:delay,
            global_counter:0,
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
            .find(|(&k, _)|k==self.current_step).unwrap();

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

    fn get_available_instructions (self: &Self) -> Option<Vec<char>> {

        let mut avail_ins:Vec<char> = Vec::new();
        
        for &step_id in self.ordered_step.iter() {
            let step = self.steps.get(&step_id).unwrap();
            if ! step.assigned {
                avail_ins.push(step_id);
            }
        }

        if avail_ins.is_empty() {
            return None;
        }
        
        avail_ins.sort();
        Some(avail_ins)
    }

    fn get_available_elfe (self: &Self) -> Option<Vec<u32>>{
        let avail_elves:Vec<u32> = self.elves.iter()
            .filter(|(_,&step)| step==' ')
            .map(|(&id,_)|id)
            .collect();

        if avail_elves.is_empty() {
            return None;
        }
        Some(avail_elves)
    }

    fn free_elfe_for_step(&mut self, step_id:char){

        for value in self.elves.values_mut(){
            if *value==step_id {
                *value=' ';
            }
        }
    }
    /// advance one step all instructions
    fn one_turn(self:&mut Self) -> Option<Vec<char>> {

        let mut added_steps=Vec::new();

        print!("\n\t{}", 
            self.global_counter);
        for (id, c) in &self.elves{
            print!("\t{}", c);
        }
        let mut new_steps:Vec<char>=Vec::new();
        let mut remove_steps:Vec<char>=Vec::new();

        for &s in self.ordered_step.iter() {
            if let Some(step) = self.steps.get_mut(&s) {
                if step.assigned {
                    print!(":{}",step.counter);
                    step.counter-=1;
                    if step.counter==0{
                        added_steps.push(s);
                        for &i in step.after.iter(){
                            new_steps.push(i);
                        }
                        remove_steps.push(s);
                    }    
                }
            } else {
                panic!("In one_turn, step {s} not found");
            }

        }
        for &i in new_steps.iter(){self.ordered_step.insert(i);}
        for i in remove_steps.iter(){
            self.ordered_step.remove(i);
            self.free_elfe_for_step(*i);
        }

        if added_steps.is_empty(){return None;}
        Some(added_steps)
    }

    fn attribute_instructions_to_elves(&mut self) {
        
        if let Some(steps) = self.get_available_instructions(){
            let mut changes = Vec::new();
        
            for (step, (elfe, _)) in steps.iter().zip(self.elves.iter_mut().filter(|(_, c)| **c == ' ')) {
                // `elfe` is a reference to the key, so we clone the key itself
                changes.push((*elfe, *step));
            }
        
            for (elfe, step_id) in changes {
                // Insert the cloned key and the step into the HashMap
                self.elves.insert(elfe, step_id);
                
                let mut step=self.steps.get_mut(&step_id).unwrap();
//                step.counter=(step_id as u8) as u32 + self.delay;
                step.assigned=true;
            }          
        }
    }

    fn run_part2 (self: &mut Self)-> String {
        println!("Running part 2");
        let mut result:String=String::new();

        let first=self.find_first();
        println!("First step is {}", first);
        self.ordered_step.insert(first);
//        result.push(first);
        loop {
            if self.ordered_step.is_empty(){
                break;
            }
            if self.get_available_instructions().is_some() && self.get_available_elfe().is_some() {
                self.attribute_instructions_to_elves();
            }
            let added_steps=self.one_turn();
            if let Some(added_steps) = added_steps {
                for c in added_steps{
                    result.push(c)
                }                        
            }
            self.global_counter+=1;
            if self.global_counter==20 {
                panic!();
            }
        }
        result
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
    let elves:u32=2;
    let add_seconds:u32=0;
    let input="Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

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
            steps.insert(before,
                Step{
                    before: Vec::new(),
                    after:vec![after],
                    done:false,
                    counter:before as u32-b'A' as u32+add_seconds+1,
                    assigned:false,
                }
            );
        }

        if let Some(step) = steps.get_mut(&after) {
            step.before.push(before);
        } else {
            steps.insert(after, Step{
                before: vec![before],
                after:Vec::new(),
                done:false,
                counter:after as u32-b'A' as u32+add_seconds+1,
                assigned:false
            });
        }
    }

    dbg!(&steps);
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

    return "BGJCNLQUYIFMOEZTADKSPVXRHW".to_string();
    let mut result:String=String::new();
    let mut instructions=Instructions::new(input, 1, 0);

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
fn solve_part2(input: &HashMap<char, Step>) -> String {
    let elves:u32=2;

    let mut instructions=Instructions::new(input, elves, 0);

    instructions.run_part2()
}

