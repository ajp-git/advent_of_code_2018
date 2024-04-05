use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day14)]
fn input_generator(input: &str) -> u32 {
    input.parse::<u32>().unwrap()
}

#[aoc(day14, part1)]
fn solve_part1(input: &u32) -> String {

    let input=*input;

    let mut scores:Vec<u8>=Vec::new();
    scores.push(3);
    scores.push(7);
    
    let mut elfe_1:usize=0;
    let mut elfe_2:usize=1;
    print_recipes(0,scores.clone(), elfe_1, elfe_2);
   
    let mut count=0;
    loop {
        let total=scores[elfe_1 as usize]+scores[elfe_2 as usize];

        let digit_1=total/10;
        if digit_1 >0 {
            scores.push(digit_1);
        }
        let digit_2=total%10;
        scores.push(digit_2);
        //println!("1:[{}]\t2:[{}]",elfe_1,elfe_2);
        
        //println!("1={}\t2={}",scores[elfe_1],scores[elfe_2]);
        elfe_1=(elfe_1+1+scores[elfe_1] as usize)%scores.len();
        elfe_2=(elfe_2+1+scores[elfe_2] as usize)%scores.len();
        //println!("1={}\t2={}",scores[elfe_1],scores[elfe_2]);



//        print_recipes(count, scores.clone(), elfe_1, elfe_2);
        count+=1;
        if count==(input+1)*2 {
            let mut s:String=String::new();
            for i in input..input+10 {
                s.push_str(format!("{}",scores[i as usize]).as_str());
            }
            return s;
        }
    }
    input.to_string()
}

fn print_recipes(index: u32, scores:Vec<u8>, elfe_1:usize, elfe_2:usize) {
    print!("{index}\t");
    for (i, score) in scores.iter().enumerate() {
        match i as u32{
            _ if i as usize == elfe_1 => print!("({score})"),
            _ if i as usize == elfe_2 => print!("[{score}]"),
            _ => print!(" {score} "),
        }
    }
    println!();
}

#[aoc(day14, part2)]
fn solve_part2(input: &u32) -> String {

    "".to_string()
}

