use super::utils;
use super::utils::Print::*;
use regex::Regex;

/**
 * Data example:
 * 
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2

 */

pub fn run(print : utils::Print ) {
    let day : &str = "5";

    match print {
        Part1 => part1(day),
        Part2 => part2(day),
        BothParts => { part1(day); part2(day); }
        NoParts => (),
    }
}

/**
 * part 1:
 * After all the movements, what's the letters that end up at the top of each stack?
 */

fn part1(day: &str) {
    let mut line_count = 1;
    let crate_regex = Regex::new(r"[A-Z]").expect("crate regex failure D:");
    let move_regex = Regex::new(r"move (\d+) from (\d) to (\d)").expect("move regex failure :c");

    // surely there's a better way
    let mut stacks: Vec<Vec<String>> = [
        Vec::new(), Vec::new(), 
        Vec::new(), Vec::new(), 
        Vec::new(), Vec::new(), 
        Vec::new(), Vec::new(), 
        Vec::new(), Vec::new()
    ].to_vec();

    for line in utils::read_lines(day) {
        let line = line.unwrap();

        if line_count <= 8 {

            // one regex can have multiple capture groups, eg ([0-3][0-9])/([0-1][0-9])/([0-9][0-9]) for dd/mm/yy
            // one text can match multiple times, eg "their birthdays were 11/11/11, 03/03/03 and 02/02/02"

            // the first loop is for full regex matches (ie 11/11/11, 03/03/03, 02/02/02)
            // if we had multiple capture groups, we could have a second loop for each group inside one match (ie 11, 11, 11)

            let cargos = crate_regex.captures_iter(&line); // captures_iter gives us all the matches in that line
            for cargo in cargos {
                let cargo = cargo.get(0).unwrap();
                let index = cargo.start();
                let letter = cargo.as_str();

                let stack_index = index/4 + 1;

                stacks.get_mut(stack_index).expect("ahh").push(letter.to_string());

                // println!("match is {}, start index is {}, letter is {}", 
                //          cargo.as_str(), 
                //          index, 
                //          letter);

            }
        }

        // 9 or 10 works here, just needs to be after we store the data and before we manipulate it and only done once
        if line_count == 9 {
            // cannot use for in loop or stacks will move and we cannot reuse it
            for i in 1..10 {
                let stack = stacks.get_mut(i).unwrap();

                // since we read the stacks from the end to the beginning, reversing them will put the end at the end
                // so we can push and pop the end of the stack
                stack.reverse();
            }
        }

        if line_count >= 11 {

            // this time we have one match and multiple capture groups
            let instruction = move_regex.captures(&line).expect("couldn't get instruction D:");
            let number_to_move: i32 = instruction.get(1).expect("bah").as_str().parse().unwrap();
            let from: usize = instruction.get(2).expect("hamburger").as_str().parse().unwrap();
            let to: usize = instruction.get(3).expect("chocolate").as_str().parse().unwrap();
            
            for _ in 0..number_to_move {
                let letter: String;
                match stacks.get_mut(from).expect("couldn't get stack").pop() {
                    Some(l) => letter = l,
                    None => continue,
                }
                stacks.get_mut(to).unwrap().push(letter);
            }
            
        }

        line_count += 1;

    }

    for i in 1..10 {
        println!("{}", stacks.get(i).unwrap().join(""));
    }
}

fn part2(day : &str) {
    
    let mut line_count = 1;
    let crate_regex = Regex::new(r"[A-Z]").expect("crate regex failure D:");
    let move_regex = Regex::new(r"move (\d+) from (\d) to (\d)").expect("move regex failure :c");

    // surely there's a better way
    let mut stacks: Vec<Vec<String>> = [
        Vec::new(), Vec::new(), 
        Vec::new(), Vec::new(), 
        Vec::new(), Vec::new(), 
        Vec::new(), Vec::new(), 
        Vec::new(), Vec::new()
    ].to_vec();

    for line in utils::read_lines(day) {
        let line = line.unwrap();

        if line_count <= 8 {

            // one regex can have multiple capture groups, eg ([0-3][0-9])/([0-1][0-9])/([0-9][0-9]) for dd/mm/yy
            // one text can match multiple times, eg "their birthdays were 11/11/11, 03/03/03 and 02/02/02"

            // the first loop is for full regex matches (ie 11/11/11, 03/03/03, 02/02/02)
            // if we had multiple capture groups, we could have a second loop for each group inside one match (ie 11, 11, 11)

            let cargos = crate_regex.captures_iter(&line); // captures_iter gives us all the matches in that line
            for cargo in cargos {
                let cargo = cargo.get(0).unwrap();
                let index = cargo.start();
                let letter = cargo.as_str();

                let stack_index = index/4 + 1;

                stacks.get_mut(stack_index).expect("ahh").push(letter.to_string());

                // println!("match is {}, start index is {}, letter is {}", 
                //          cargo.as_str(), 
                //          index, 
                //          letter);

            }
        }

        // 9 or 10 works here, just needs to be after we store the data and before we manipulate it and only done once
        if line_count == 9 {
            // cannot use for in loop or stacks will move and we cannot reuse it
            for i in 1..10 {
                let stack = stacks.get_mut(i).unwrap();

                // since we read the stacks from the end to the beginning, reversing them will put the end at the end
                // so we can push and pop the end of the stack
                stack.reverse();
            }
        }

        if line_count >= 11 {

            // this time we have one match and multiple capture groups
            let instruction = move_regex.captures(&line).expect("couldn't get instruction D:");
            let number_to_move: i32 = instruction.get(1).expect("bah").as_str().parse().unwrap();
            let from: usize = instruction.get(2).expect("hamburger").as_str().parse().unwrap();
            let to: usize = instruction.get(3).expect("chocolate").as_str().parse().unwrap();
            
            let mut popped: Vec<String> = Vec::new();
            for _ in 0..number_to_move {
                let letter: String;
                match stacks.get_mut(from).expect("couldn't get stack").pop() {
                    Some(l) => letter = l,
                    None => continue,
                }
                popped.push(letter);
            }
            popped.reverse();
            for i in popped {
                stacks.get_mut(to).unwrap().push(i);
            }
            
        }

        line_count += 1;

    }

    for i in 1..10 {
        println!("{}", stacks.get(i).unwrap().join(""));
    }
}