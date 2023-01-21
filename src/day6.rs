use super::utils;
use super::utils::Print::*;

/**
 * Data: One very long string of characters, with many duplicates
 */
pub fn run(print : utils::Print ) {
    let day : &str = "6";

    match print {
        Part1 => part1(day),
        Part2 => part2(day),
        BothParts => { part1(day); part2(day); }
        NoParts => (),
    }
}

/**
 * Part1 : Find the first index where the preceding 4 characters contain no duplicates
 * eg aaabbcacbacbabcabCABDabcbabcbabcbcabcabab - the CABD (caps for emphasis) is at position 21
 */
fn part1(day: &str) {
    let mut input : String = String::from("");
    let mut marker = 0;

    // only one line in input
    for line in utils::read_lines(day) {
        input = line.unwrap();
    }

    for i in 4..input.len() {
        let input_slice = &input[i-4..i];
        if !has_dupes(input_slice) {
            marker = i;
            //println!("Found dupe at marker {i}: {input_slice}");
            break;
        }
    }

    println!("Day 6 part 1: {}", marker);
}

// probably not very efficient
fn has_dupes(input: &str) -> bool {
    for j in 0..input.len()-1 {
        for k in j+1..input.len() {
            if input.as_bytes()[j] == input.as_bytes()[k] {
                return true;
            }
        }
    }
    return false;
}

/**
 * Part2 : Find the first index where the preceding 14 characters contain no duplicates
 * ie same as part 1 but 14 instead of 4
 */

fn part2(day : &str) {
    
    let mut input : String = String::from("");
    let mut marker = 0;

    // only one line in input
    for line in utils::read_lines(day) {
        input = line.unwrap();
    }

    for i in 14..input.len() {
        let input_slice = &input[i-14..i];
        if !has_dupes(input_slice) {
            marker = i;
            //println!("Found dupe at marker {i}: {input_slice}");
            break;
        }
    }

    println!("Day 6 part 2: {}", marker);
}

#[allow(dead_code)]
pub fn more_efficient() {
    let mut input : String = String::from("");

    for line in utils::read_lines("6") {
        input = line.unwrap();
    }

    let chars : Vec<char> = input.chars().collect();

    let mut num: u32 = 0;
    let size: usize = 14;
    let size_u32 = size.try_into().expect("aah");

    for i in 0..size {
        // get_char_value: a = 0, z = 25
        num += 2 << get_char_value(chars[i]);
    }
    
    let mut index: usize = size;

    while num.count_ones() != size_u32 {
        num += 2 << get_char_value(chars[index]);
        num -= 2 << get_char_value(chars[index-size]);
        index += 1;
    }

    println!("{}", index);


}

#[allow(dead_code)]
fn get_char_value(char : char) -> u32 {
    // there's probably a better way to do this
    // to_digit gives 10 for a and 35 for z (both uppercase and lowercase)
    // so we subtract to instead get a=0 and z=25
    char.to_digit(36).expect("bah") - 10
}
