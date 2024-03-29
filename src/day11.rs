use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day11)]
fn input_generator(input: &str) -> u32 {
 6042
}


#[aoc(day11, part1)]
fn solve_part1(input: &u32) -> String {

 let mut max_found=i32::MIN;
 let mut max_x=0;
 let mut max_y=0;

 for x in 0..297 {
    for y in 0..297 {

        let mut sub_total=0;
        for i in 0..3 {
            for j in 0..3 {
                sub_total+=get_power(x+i, y+j, *input);
            }
        }
        if sub_total>max_found{
            max_found=sub_total;
            max_x=x;
            max_y=y;

            print!("\rFound new max : {},{} = {}", max_x, max_y, max_found);
        }
    }
 } 
 println!();
 format!("{} {}", max_x, max_y)
}

fn get_power(x:u32,y:u32,serial:u32) -> i32 {
    let rack_id: i32 = x as i32+10;
    let mut power:i32=rack_id as i32*y as i32;
    power+=serial as i32;
    power*=rack_id;
    power = (power/100)%10;
    power-=5;
    
    power
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_power_known_values() {
        // Test cases based on known values
        assert_eq!(get_power(122, 79, 57), -5);
        assert_eq!(get_power(217, 196, 39), 0);
        assert_eq!(get_power(101, 153, 71), 4);
    }

}