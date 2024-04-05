use std::{collections::{HashMap, HashSet}, fmt::Debug};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug,Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn apply (&self, pos:(usize,usize)) -> (usize, usize) {
        match *self {
            Direction::East => (pos.0+1, pos.1),
            Direction::West => (pos.0.checked_sub(1).unwrap_or(pos.0),pos.1),
            Direction::North => (pos.0, pos.1.checked_sub(1).unwrap_or(pos.1)),
            Direction::South => (pos.0, pos.1+1),
        }
    }
}

#[derive(Debug, Clone)]
struct Cart {
    id:u32,
    x:usize,
    y:usize,
    dir:Direction,
    turn:u8,
}

impl Cart {

    fn get_pos(&self)->(usize,usize) {
        (self.x, self.y)
    }

    fn go_next_pos (&mut self, c:char) {

        match c {
            '+' => {
                match self.turn {
                    0 => {
                        match self.dir {
                            Direction::East => self.dir=Direction::North,
                            Direction::North => self.dir=Direction::West,
                            Direction::West => self.dir=Direction::South,
                            Direction::South => self.dir=Direction::East,
                        }
                    },
                    1 => {},
                    2 => {
                        match self.dir {
                            Direction::East => self.dir=Direction::South,
                            Direction::North => self.dir=Direction::East,
                            Direction::West => self.dir=Direction::North,
                            Direction::South => self.dir=Direction::West,
                        }

                    },
                    _ => panic!("Cart::go_next_pos : impossible turn value"),
                };
                self.turn=(self.turn+1)%3;
            },
            '/' => {
                match self.dir {
                    Direction::West=>self.dir=Direction::South,
                    Direction::South=>self.dir=Direction::West,
                    Direction::North=>self.dir=Direction::East,
                    Direction::East=>self.dir=Direction::North,
                }
            },
            '\\' => {
                match self.dir {
                    Direction::East=>self.dir=Direction::South,
                    Direction::South=>self.dir=Direction::East,
                    Direction::North=>self.dir=Direction::West,
                    Direction::West=>self.dir=Direction::North,
                }

            },
            '-' => {},
            '|' => {},
            _ => panic!("go_next_pos match ({}) unhandled",c), 
            
        }

        (self.x,self.y) = self.dir.apply(self.get_pos());

    }
}
#[derive(Clone)]
struct Crop {
    grid:HashMap<(usize,usize),char>,
    carts:Vec<Cart>,
}

impl Crop {

    fn get_ordered_carts(&self) -> Vec<Cart> {

        let mut carts:Vec<Cart>=self.carts.iter().cloned().collect();
        carts.sort_by(|a,b| a.x.cmp(&b.x).then_with(||a.y.cmp(&b.y)));
        carts
    }

    fn step(&mut self) -> Option<(usize,usize)> {
        // on parse les carts
        // on regarde la direction de chacun et la case
        self.print();
        println!();
        
        let mut new_carts:Vec<Cart>=Vec::new();

        let mut carts=self.get_ordered_carts();
        for cart in carts.iter_mut() {
            cart.go_next_pos(self.get_cart_path(cart.id));
            new_carts.push(cart.clone());
        }
        self.carts=new_carts;
        self.is_crash()
    }

    fn is_crash(&self) -> Option<(usize, usize)> {
        
        let mut positions:HashSet<(usize,usize)>=HashSet::new();

        for cart in self.carts.iter(){
            if ! positions.insert((cart.x, cart.y)){
                return Some((cart.x,cart.y));
            }
        }
        None
    }

    fn get_cart_path(&self, id:u32) -> char{
        let cart=self.carts.iter().find(|c|c.id==id).unwrap();
        let path=self.grid.get(&(cart.x, cart.y)).unwrap();
        *path
    }

    fn print(&self) {
        let max_y = self.grid.keys().map(|g| g.1 ).max().unwrap();
        let max_x = self.grid.keys().map(|g| g.0 ).max().unwrap();
        for y in 0..=max_y{
            for x in 0..=max_x {
                let mut char_to_print=' ';

                if let Some(c) = self.grid.get(&(x,y)) {
                    char_to_print=*c;

                    if let Some(cart) = self.carts.iter().find(|c|c.x==x && c.y==y) {
                        match cart.dir {
                            Direction::East=>char_to_print='>',
                            Direction::North=>char_to_print='^',
                            Direction::South=>char_to_print='v',
                            Direction::West=>char_to_print='<',
                        }
                    }
                    print!("{}",char_to_print);
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

}

#[aoc_generator(day13)]
fn input_generator(input: &str) -> Crop {

/*
    let input="/->-\\        
|   |  /----\\
| /-+--+-\\  |
| | |  | v  |
\\-+-/  \\-+--/
  \\------/  ";
 */
    let mut grid:HashMap<(usize,usize), char>=HashMap::new();
    let mut carts:Vec<Cart>=Vec::new();
    let mut card_id=0;

    for (y,line) in input.lines().enumerate() {
        for (x,c) in line.chars().enumerate() {
            let mut c = c;
            if "<>^v".contains(c) {
                let mut dir:Direction=Direction::South;
                match c {
                    'v' => {dir=Direction::South;c='|';},
                    '^' => {dir=Direction::North;c='|';},
                    '<' => {dir=Direction::West;c='-';},
                    '>' => {dir=Direction::East;c='-';},
                    _ => {},                
                }
                carts.push(Cart { id: card_id, x: x, y: y, dir: dir, turn: 0 });
                card_id+=1;
            }
            grid.insert((x,y), c);
        }
    }
    
    Crop { grid: grid, carts: carts }
}

#[aoc(day13, part1)]
fn solve_part1(input: &Crop) -> String {
    let mut crop=input.clone();
    
    loop{
        if let Some(pos)=crop.step(){
            return format!("{},{}", pos.0, pos.1);
        }
    }
    "".to_string()
}

#[aoc(day13, part2)]
fn solve_part2(input: &Crop) -> String {
    "".to_string()
}

