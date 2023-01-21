use super::utils;
use super::utils::Print::*;

/**
 * Day 10:
 * Data: looks like assembly
 * [command] [value]
 * commands: 
 *   - noop takes one cycle
 *   - addx takes two cycles, then adds a value to the x register (which starts at 1), x remains the same during the first of the two cycles
 */
pub fn run(print : utils::Print ) {
    let day : &str = "10";

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
 * Part 1:
 * Signal strength = cycle number * x, where cycle number is 20 + 40n
 * note that signal strength for addx takes the old value of x
 * Sum the signal strength for cycles 20, 60, 100, 140, 180, 220
 */
fn part1(day: &str) {
    
    let mut cycle = 0;
    let mut x = 1;
    let mut signal = 0;
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        
        let mut cycle_to_add = 0;
        let mut addx = 0;
        if line == "noop".to_string() {
            cycle_to_add+=1;
        } else if line[0..4] == "addx"[..] {
            addx = line[5..].chars().as_str().parse().unwrap();
            cycle_to_add+=2;
        } else {
            panic!("couldn't read line: {}", line);
        }

        cycle += 1;
        match cycle {
            20 | 60 | 100 | 140 | 180 | 220 => {
                signal += cycle * x
            },
            _ => ()
        }

        if cycle_to_add == 2 {
            cycle += 1;
            match cycle {
                20 | 60 | 100 | 140 | 180 | 220 => {
                    signal += cycle * x
                },
                _ => ()
            }
            x += addx;
        }
        if cycle >= 220 {
            break;
        }

    }

    println!("Day 10 part 1: {}", signal);
}

/**
 * the value of x is a 3x1 sprite centered on x
 * 40 cycles per row, if the sprite is on the cycle position, then we draw the lit value.
 * note once again addx is considered at the very endof the loop
 */
fn part2(day : &str) {
    
    let mut cycle: i32 = 0;
    let mut x: i32 = 1;
    let mut output = "".to_string();
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        
        let (command, addx) = parse_line(line);
        handle_output(&mut output, cycle, x);
        cycle += 1;
        if command == "addx" {
            handle_output(&mut output, cycle, x);
            cycle += 1;
            x += addx;
        }
    }
}

fn parse_line(line: String) -> (String, i32) {
    if line == "noop".to_string() {
        ("noop".to_string(), 0)
    } else if line[0..4] == "addx"[..] {
        let addx = line[5..].chars().as_str().parse().unwrap();
        ("addx".to_string(), addx)
    } else {
        panic!("couldn't read line: {}", line);
    }
}

fn print_pixels(output: &String) {
    println!("Day 10 part 2: {}", output);
}

fn handle_output(output: &mut String, cycle: i32, x: i32) {
    if (cycle % 40).abs_diff(x) <= 1 {
        *output += "#";
    } else {
        *output += "-";
    }
    if cycle % 40 == 39 {
        print_pixels(output);
        (*output).clear();
    }
}