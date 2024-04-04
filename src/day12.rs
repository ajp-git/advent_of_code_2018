use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
#[derive(Debug,Clone)]
struct Cave{
    pots:String,
    rules:HashSet<String>,
    generations:usize,

}

impl Cave {
    
    fn new(pots:String, rules:HashSet<String>) -> Self {
        Cave { pots, rules, generations:0 }
    }

    fn run_generations(&mut self, generations:usize ) -> isize {

        const CONVERGE:u32=10;

        let mut n = String::from("...");
        n.push_str(&self.pots);
        n.push_str("...");

        let mut last = 0;
        let mut diffs: HashMap<isize, u32> = HashMap::new();

        for gen in 1..=generations {
            let mut out = String::from("...");
            for i in 2..n.len() - 2 {
                let slice = &n[i - 2..=i + 2];
                if self.rules.get(slice).is_some(){
                    out.push('#')
                } else {
                    out.push('.');
                }
                    }
            out.push_str("...");
            n = out;

            // Our string grows by one '.' at both the beginning and end each generation
            let score = n
                .chars()
                .enumerate()
                .filter(|(_, c)| c == &'#')
                .map(|(i, _)| i as isize - (3 + gen as isize))
                .sum::<isize>();
            let e = diffs.entry(score as isize - last as isize).or_insert(0);
            if *e > CONVERGE {
                return (generations - gen) as isize * (score - last) + score;
            } else {
                *e += 1;
            }
            last = score;
        }
        last    
    }
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

    let mut cave=Cave::new(first_line,rules);
    dbg!(&cave);
    cave
}

#[aoc(day12, part1)]
fn solve_part1(input: &Cave) -> isize {

    let mut cave=input.clone();
    cave.run_generations(20)
}

#[aoc(day12, part2)]
fn solve_part2(input: &Cave) -> isize {
    let mut cave=input.clone();
    cave.run_generations(50_000_000_000)

}
