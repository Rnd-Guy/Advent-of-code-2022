use std::collections::HashMap;

use super::utils;
use super::utils::Print::*;

/**
 * Day 9:
 * Data: (U|D|L|R) [1-9][0-9]
 * T follows H on a 2d grid, being a max of 1 space behind
 * if same row or col, follows in that same row or col, otherwise moves diagonally closer
 * initial position is H on T
 */
pub fn run(print : utils::Print ) {
    let day : &str = "9";

    if day == "0" {
        panic!("CHANGE THE DAY");
    }

    match print {
        Part1 => part1(day),
        Part2 => part2(day),
        BothParts => { part1(day); part2(day); }
        NoParts => (),
    }
}

/**
 * Part 1: Follow the H and keep track of how many distinct spaces T travelled
 */
fn part1(day: &str) {

    let mut t = [0,0]; // x, y, up is positive
    let mut h = [0,0];
    let mut trail: HashMap<[i32;2], u32> = HashMap::new();
    trail.entry(h).and_modify(|v| *v += 1).or_insert(1);
    for line in utils::read_lines(day) {

        let line = line.unwrap();
        
        let direction = line.chars().nth(0).unwrap();
        let distance: i32 = line.chars().as_str()[2..].parse().unwrap();

        match direction {
            'U' => h[1] += distance,
            'D' => h[1] -= distance,
            'L' => h[0] -= distance,
            'R' => h[0] += distance,
            _ => (),
        }

        follow_head(&mut trail, h, &mut t, true);
    }

    println!("Day 9 part 1: {}", trail.len());
}

fn follow_head(trail: &mut HashMap<[i32;2], u32>, head: [i32; 2], tail: &mut [i32;2], store: bool) {
    // summary is that the point is at least 2 in one direction
    // note with part 2: you can have parts that are 2 diagonal from each other, so 
    if (head[0] - tail[0]).abs() >= 2 {
        // diagonal movement always happens first
        if head[1] > tail[1] {
            tail[1] += 1;
        } else if head[1] < tail[1] {
            tail[1] -= 1;
        }
        if head[0] > tail[0] {
            while tail[0] < head[0] - 1 {
                tail[0] += 1;
                if store {
                    store_trail(trail, tail);
                }
            }
        } else {
            while tail[0] > head[0] + 1 {
                tail[0] -= 1;
                if store {
                    store_trail(trail, tail);
                }
            }
        }

    } else if (head[1] - tail[1]).abs() >= 2 {
        if head[0] > tail[0] {
            tail[0] += 1;
        } else if head[0] < tail[0] {
            tail[0] -= 1;
        }
        if head[1] > tail[1] {
            while tail[1] < head[1] - 1 {
                tail[1] += 1;
                if store {
                    store_trail(trail, tail);
                }
            }
        } else {
            while tail[1] > head[1] + 1 {
                tail[1] -= 1;
                if store {
                    store_trail(trail, tail);
                }
            }
        }
    }
}

fn store_trail(trail: &mut HashMap<[i32;2], u32>, key: &[i32;2]) {
    (*trail).entry(*key).and_modify(|v| *v += 1).or_insert(1);
    //println!("{:?}, {}", key, trail.get(key).unwrap());
}

fn part2(day : &str) {

    let mut ropes: Vec<[i32;2]> = vec!([0;2], [0;2], [0;2], [0;2], [0;2], [0;2], [0;2], [0;2], [0;2], [0;2]); // 0 is head, 9 is tail
    let mut trail: HashMap<[i32;2], u32> = HashMap::new();

    store_trail(&mut trail, &ropes[9]);

    for line in utils::read_lines(day) {

        let line = line.unwrap();
        
        let direction = line.chars().nth(0).unwrap();
        let distance: i32 = line.chars().as_str()[2..].parse().unwrap();

        for _ in 0..distance {
            match direction {
                'U' => ropes[0][1] += 1,
                'D' => ropes[0][1] -= 1,
                'L' => ropes[0][0] -= 1,
                'R' => ropes[0][0] += 1,
                _ => (),
            }

            for i in 0..8 {
                follow_head(&mut trail, ropes[i], &mut ropes[i+1], false);
            }
            follow_head(&mut trail, ropes[8], &mut ropes[9], true);
            //println!("{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}", ropes[0], ropes[1], ropes[2], ropes[3], ropes[4], ropes[5], ropes[6], ropes[7], ropes[8], ropes[9])
        }

        println!("above is end");
        //follow_head(&mut trail, h, &mut t);
    }

    println!("Day 9 part 2: {}", trail.len());
}