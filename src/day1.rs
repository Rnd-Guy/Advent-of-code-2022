//use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use super::utils;
use super::utils::Print::*;

    /**
     * day 1:
     * Data: Groups of numbers separated by a blank line
     * Part 1: Sum the groups and return the largest sum
     * Part 2: Sum the 3 largest groups
     */
pub fn run(print: utils::Print) {

    let path = "inputs/day1.txt";

    // let file = fs::read_to_string("day1input.txt")
    //     .expect("Failed to read file D:");

    let file = fs::File::open(path)
        .expect("bah");
    let buf = io::BufReader::new(file);

    // let mut elves: Vec<&Vec<i32>> = Vec::new(); // elves hold an elf
    // let mut elf = vec![0];
    // let mut max = 0;

    // for line in buf.lines() {
    //     let unwrapped = line.unwrap();
    //     if unwrapped == "" {
    //         elves.push(&mut elf);
    //         if elf[0] > max {
    //             max = elf[0];
    //         }
    //         elf = vec![0];
    //     } else {
    //         let num: i32 = unwrapped.parse().expect("cannot parse");
    //         elf.push(num);
    //         elf[0] += num;
    //     }
    // }

    // let mut elves: Vec<Vec<&mut i32>> = Vec::new();
    // let mut max = 0;

    // for line in buf.lines() {
    //     let unwrapped = line.unwrap();
    //     if unwrapped == "" {
    //         if **elves.last().unwrap().first().unwrap() > max {
    //             max = **elves.last().unwrap().first().unwrap();
    //         }
    //         let mut newElf: Vec<&mut i32> = Vec::new();
    //         elves.push(newElf);
    //     } else {
    //         let mut num: i32 = unwrapped.parse().expect("not a num D:");
    //         let mut elf = elves.last().unwrap();

    //         elf.push(&mut num);
    //         **elf.first().unwrap() += num;

    //         //elves.last().unwrap().first_mut().unwrap() += num;
    //         //elves.last().expect("has nothing D:").push(num);
    //     }
    // }

    let mut elves: Vec<Vec<i32>> = Vec::new();
    elves.push(vec![0]);
    let mut max = 0;

    for line in buf.lines() {
        let unwrapped = line.unwrap();
        if unwrapped == "" {
            if *elves.last().unwrap().first().unwrap() > max {
                max = *elves.last().unwrap().first().unwrap();
            }
            elves.push(vec![0]);

        }
        else {
            let num: i32 = unwrapped.parse().unwrap();
            elves
                .last_mut()
                .expect("dang")
                .push(num);
            *elves.last_mut().expect("rip").first_mut().expect("unfortunate") += num;
        }
    }

    match print {
        BothParts | Part1 => println!("{}", max),
        _ => (),
    }

    let mut totals = elves.iter().map(|x| *x.first().unwrap()).collect::<Vec<i32>>();
    totals.sort();
    let sum = totals[totals.len()-1] + totals[totals.len()-2] + totals[totals.len()-3];

    match print {
        BothParts | Part2 => println!("{}", sum),
        _ => (),
    }
    
    


    
}