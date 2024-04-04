use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
#[derive(Debug,Clone)]
struct Cave{
    pots:String,
    rules:HashSet<String>,
    counter:u32,
}

#[aoc_generator(day12)]
fn input_generator(input: &str) -> Cave {

/*let input="initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";*/

    let mut line_iter=input.lines().peekable();
    let mut rules:HashSet<String>=HashSet::new();

    let first_line=line_iter
        .next()
        .unwrap()
        .chars()
        .skip(15)
        .take(99)
        .collect::<String>();
    println!("{:?}", first_line);
    line_iter.next(); // Skip blank line

    while let Some(line)=line_iter.next() {
        let ident=line.chars().take(5).collect::<String>();
        let give=line.chars().skip(9).take(1).collect::<String>();

        if give=="#" {
            println!("Rule {} gives #", ident);
            rules.insert(ident);
        }
    }

    let mut cave=Cave{pots:first_line,rules:rules, counter:0};
    dbg!(&cave);
    cave
}

#[aoc(day12, part1)]
fn solve_part1(input: &Cave) -> i32 {

    let mut count:i32=0;
    let mut cave=input.clone();

    for i in 0..20 {
        cave.pots=format!("...{}...", cave.pots);
        println!("{i}: {:?}", cave.pots);
        run_generation(&mut cave);
    }
    let end =format!("{:?}", cave.pots.chars().skip(17).collect::<String>());

    println!("Looking at {}", end);
    for (i,c) in end.chars().enumerate() {
        if c=='#' {
            count+=i as i32-4;
        }
    }
    count
}

fn run_generation (cave: &mut Cave) {

    let mut out:String=String::new();

    for i in 0..=cave.pots.len()-5 {
        let posts=cave.pots.chars().skip(i).take(5).collect::<String>();
        if cave.rules.get(&posts).is_some(){
            out.push('#')
        } else {
            out.push('.');
        }
    }
    cave.pots=out;
}