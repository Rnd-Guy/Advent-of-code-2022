use super::utils;
use super::utils::Print::*;

pub fn run(print : utils::Print ) {
    const DAY : &str = "3";

    match print {
        Part1 => part1(DAY),
        Part2 => part2(DAY),
        BothParts => { part1(DAY); part2(DAY); }
        NoParts => (),
    }
}

/*
    Day 3:
    Data: ([a-zA-Z](2))(+)
    Each line of text is a-zA-Z of even length. Left half is one set of data, right half is another.
    There is one shared letter in both halves for each line.
*/


/**
 * Part 1:
 * a-z value 1-26
 * A-Z value 27-52
 * Sum all the values for each letter that appears more than once in both halves on each line
 */
fn part1(day: &str) {
    let mut value = 0;

    for line in utils::read_lines(day) {
        let line = line.unwrap();
        
        let (left, right) = line.split_at(line.len()/2);
        
        let mut dupe_char : char = 'a';
        for char in left.chars() {
            if right.contains(char) {
                dupe_char = char;
                break;
            }
        }
        value += get_char_value(dupe_char);

    }

    println!("day3 part 1: {}", value);
}

/**
 * Part 2:
 * Letter values the same, but now we group lines into groups of 3. Each 3 lines will have one shared letter in all three.
 * Sum all of these shared letters
 */
fn part2(day: &str) {
    let mut value = 0;
    let mut group_index = 0; // 0, 1, 2
    let mut group = ["".to_string(), "".to_string(), "".to_string()];

    for line in utils::read_lines(day) {
        let line = line.unwrap();

        group[group_index] = line.clone();
        if group_index < 2 {
            group_index += 1;
        } else {
            group_index = 0;
            let mut dupe_chars : Vec<char> = Vec::new();
            for char in group[0].chars() {
                if group[1].contains(char) && !dupe_chars.contains(&char) {
                    dupe_chars.push(char.clone());
                }
            }

            for char in dupe_chars {
                if group[2].contains(char) {
                    value += get_char_value(char);
                    break;
                }
            }
        }

        
    }
    println!("day3 part 2: {}", value);
}

fn get_char_value(char : char) -> u32 {
    // there's probably a better way to do this
    // to_digit gives 10 for a and 35 for z (both uppercase and lowercase)

    if char.is_lowercase() {
        return char.to_digit(36).expect("bah") - 9;
    } else if char.is_uppercase() {
        return char.to_digit(36).expect("bruh") - 9 + 26
    }
    0
}