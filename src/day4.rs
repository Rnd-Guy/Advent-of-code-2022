use super::utils;
use super::utils::Print::*;

/**
 * Data:
 * a-b,y-z where a,b,y,z are numbers with a<b and y<z
 */
pub fn run(print : utils::Print ) {
    let day : &str = "4";

    match print {
        Part1 => part1(day),
        Part2 => part2(day),
        BothParts => { part1(day); part2(day); }
        NoParts => (),
    }
}

/**
 * Count number of rows where a-b is completely inside y-z or vice versa
 */
fn part1(day: &str) {
    
    let mut sum = 0;
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        
        let split = line.split(",").collect::<Vec<&str>>();
        let left = split[0];
        let right = split[1];

        let left_split = left.split("-").collect::<Vec<&str>>();
        let l1: i32 = left_split[0].parse().expect("");
        let l2: i32 = left_split[1].parse().expect("");
        
        let right_split = right.split("-").collect::<Vec<&str>>();
        let r1: i32 = right_split[0].parse().expect("");
        let r2: i32 = right_split[1].parse().expect("");

        if (l1 <= r1 && l2 >= r2) || (r1 <= l1 && r2 >= l2) {
            sum += 1;
        }

    }

    println!("Day 4 part 1: {}", sum);
}

fn part2(day: &str) {
    
    let mut sum = 0;
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        
        let split = line.split(",").collect::<Vec<&str>>();
        let left = split[0];
        let right = split[1];

        let left_split = left.split("-").collect::<Vec<&str>>();
        let l1: i32 = left_split[0].parse().expect("");
        let l2: i32 = left_split[1].parse().expect("");
        
        let right_split = right.split("-").collect::<Vec<&str>>();
        let r1: i32 = right_split[0].parse().expect("");
        let r2: i32 = right_split[1].parse().expect("");

        if (l1 <= r2 && l2 >= r1) || (r1 <= l2 && r2 >= l1) {
            sum += 1;
        }

    }

    println!("Day 4 part 2: {}", sum);
}