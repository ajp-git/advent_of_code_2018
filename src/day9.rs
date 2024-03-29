use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

struct Circle {
    current_pos:usize,
    current_user:u32,
    marbles:HashMap<u32,u32>, // place, value
    users:HashMap<u32,u32>, // user_id, points
    users_number:u32,
    play_rounds:u32,
}

impl Circle {
    fn new(users_number:u32, play_rounds:u32 )-> Self{
        Circle {
            current_pos: 0,
            current_user: 0,
            marbles: HashMap::new(), 
            users: HashMap::new(),
            users_number,
            play_rounds,

        }
    }

    fn add (&mut self) {

        // play_round contains the value of the new marble
        let pos=self.get_next_pos();
        let mut marble=self.marbles.entry(pos).or_insert(0);
        *marble=self.play_rounds;
        self.current_user=(self.current_user+1%self.users_number);
    }

    fn get_winner(&self) -> u32 {
        0
    }

    fn insert_at_pos(&mut self, pos:u32, value:u32) {

        let shifted_keys:Vec<u32>=self.marbles
            .keys()
            .filter(|&&k| k>= pos)
            .cloned()
            .collect();

        for key in shifted_keys.iter().rev() {
            if let Some(&val) = self.marbles.get(key) {
                self.marbles.insert(key+1, val);
            }
        }
        self.marbles.insert(pos, value);
    }

    fn get_next_pos(&self) -> u32 {

        let len=self.marbles.len();
        if self.current_pos+2 > len {

        }
        

        (self.current_pos as u32 +1)%((self.marbles.len() as u32 +1).max(1))
    }

    fn print(&self) {
        print!("[{}]", self.current_user);
        for i in 0..self.marbles.len() {
            print!("{}",
                if i==self.current_pos {
                    format!("({})",i)
                }else{
                    format!(" {i} ")
                }
            )
        }
        println!();
    }
}

#[aoc_generator(day9)]
fn input_generator(input: &str) -> (u32,u32) {
 (9,25) // 32 high score
 //(10,1618) // 8317 
 
}

#[aoc(day9, part1)]
fn solve_part1(input: &(u32, u32)) -> u32 {
 let mut circle = Circle::new(input.0,input.1);

 for i in 0..input.0 {
    circle.print();
    circle.add();
 }
 
 circle.get_winner()
}

