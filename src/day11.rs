use std::num;

use super::utils;
use super::utils::Print::*;

/**
 * Data:
 * Monkey business
 * Best to just see the test data file
 * Monkey inspects item and performs operation on that item
 * Then we divide by 3 and round down
 * Then monkey tests
 * then monkey passes item
 * then next monkey is considered (new monkeys consider new items too)
 */
pub fn run(print : utils::Print ) {
    let day : &str = "11";

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

// struct Monkey {
//     items: Vec::<u64>,
//     inspects: u64, // number of items inspected
//     operation: Option<Box<dyn Fn()>>, // performs the test then divides by three, 
//     test: Option<Box<dyn Fn()>>, // returns the monkey to pass to
// }
struct Monkey {
    items: Vec::<u64>,
    inspects: u64, // number of items inspected
    operation: Box<dyn Fn(u64) -> u64>, // performs the test then divides by three, 
    test: u64,
    if_true: usize,
    if_false: usize,
} 

impl Monkey {
    fn new() -> Self {
        Self{items: Vec::new(),
                inspects: 0,
                operation: Box::new(|_| 0),
                test: 1,
                if_true: 0,
                if_false: 0
        }
    }

    fn handle_items(&mut self) -> Vec<(usize,u64)> {
        let mut thrown_items = Vec::new();
        for item in &self.items {
            let mut new_item = (&self.operation)(*item);
            new_item /= 3;

            if new_item % self.test == 0 {
                //monkeys[self.if_true].items.push(new_item);
                thrown_items.push((self.if_true, new_item));
            } else {
                //monkeys[self.if_false].items.push(new_item);
                thrown_items.push((self.if_false, new_item));
            }

            self.inspects += 1;
        }
        self.items.clear();

        thrown_items
    }

    // in part 2 we don't divide by 3 but instead modulo with the common divider
    fn handle_items2(&mut self, divider: u64) -> Vec<(usize,u64)> {
        let mut thrown_items = Vec::new();
        for item in &self.items {
            let mut new_item = item % divider;
            new_item = (&self.operation)(new_item);
            if new_item % self.test == 0 {
                //monkeys[self.if_true].items.push(new_item);
                thrown_items.push((self.if_true, new_item));
            } else {
                //monkeys[self.if_false].items.push(new_item);
                thrown_items.push((self.if_false, new_item));
            }

            self.inspects += 1;
        }
        self.items.clear();

        thrown_items
    }
}

/**
 * Part 1: Count number of items a monkey inspects in total after 20 rounds and multiply the two largest inspects
 */
fn part1(day: &str) {

    // first read the file and learn the rules
    let mut monkeys: Vec<Monkey> = Vec::new();
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        
        parse_monkey(&mut monkeys, line);
    }

    let rounds = 20;
    for i in 0..rounds {
        for monkey in 0..monkeys.len() {
            let thrown_items = monkeys[monkey].handle_items();
            for (to_monkey, item) in thrown_items {
                monkeys[to_monkey].items.push(item);
            }
        }
    }

    // output
    let mut max_inspected1 = 0;
    let mut max_inspected2 = 0;
    for monkey in 0..monkeys.len() {
        //println!("Monkey {monkey} has items: {:?} and inspected {} times", monkeys[monkey].items, monkeys[monkey].inspects);
        let inspected = monkeys[monkey].inspects;
        if inspected >= max_inspected1 {
            max_inspected2 = max_inspected1;
            max_inspected1 = inspected;
        } else if inspected > max_inspected2 {
            max_inspected2 = inspected
        }
    }
    println!("Day 11 part 1: {}", max_inspected1 * max_inspected2);

}

fn parse_monkey(monkeys: &mut Vec<Monkey>, line: String) -> u64{
    // the return value is specifically for the test

    let mut current_monkey = monkeys.len();
    if current_monkey > 0 {
        current_monkey -= 1;
    }

    if line == "" {
        // noop
    } else if line[0..6] == "Monkey"[..] {
        monkeys.push(Monkey::new())

    } else if line[2..7] == "Start"[..] {
        let items = line[18..].split(", ");
        for item in items {
            
            monkeys[current_monkey].items.push(item.parse().unwrap());
        }

    } else if line[2..7] == "Opera"[..] {
        let operation = line.chars().nth(23).unwrap();
        let value = line[25..].to_string();

        //println!("line is {}, operation is {}, value is {}", line, operation, value);

        monkeys[current_monkey].operation = parse_operation(operation, value);

    } else if line[2..6] == "Test"[..] {
        let value = line[21..].parse::<u64>().unwrap();
        monkeys[current_monkey].test = value;

        return value

    } else if line[4..11] == "If true"[..] {
        let value = line[29..30].parse::<usize>().unwrap();
        monkeys[current_monkey].if_true = value;

    } else if line[4..12] == "If false"[..] {
        let value = line[30..31].parse::<usize>().unwrap();
        monkeys[current_monkey].if_false = value;

    } else {
        panic!("Couldn't parse monkey D: operation was {}", line)
    }

    0
}

// operation is always "new = old (+|*) (old|[0-9]+)"
fn parse_operation(operation: char, value: String) -> Box<dyn Fn(u64) -> u64> {
    if operation == '+' {
        if value == "old" {
            Box::new(|i| i + i)
        } else {
            Box::new(move |i| i + value.parse::<u64>().unwrap())
        }
    } else if operation == '*' {
        if value == "old" {
            Box::new(|i| i * i)
        } else {
            Box::new(move |i| i * value.parse::<u64>().unwrap())
        }
    } else {
        panic!("couldn't parse operation D: operation was {} {}", operation, value);
    }
}

fn part2(day : &str) {
    
    // first read the file and learn the rules
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut divider = 1;
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        
        let new_divider = parse_monkey(&mut monkeys, line);
        if new_divider > 0 {
            divider *= new_divider;
        }
    }

    let rounds = 10000;
    for i in 0..rounds {
        for monkey in 0..monkeys.len() {
            let thrown_items = monkeys[monkey].handle_items2(divider);
            for (to_monkey, item) in thrown_items {
                monkeys[to_monkey].items.push(item);
            }
        }
    }

    // output
    let mut max_inspected1 = 0;
    let mut max_inspected2 = 0;
    for monkey in 0..monkeys.len() {
        //println!("Monkey {monkey} has items: {:?} and inspected {} times", monkeys[monkey].items, monkeys[monkey].inspects);
        let inspected = monkeys[monkey].inspects;
        if inspected >= max_inspected1 {
            max_inspected2 = max_inspected1;
            max_inspected1 = inspected;
        } else if inspected > max_inspected2 {
            max_inspected2 = inspected
        }
    }
    println!("Day 11 part 2: {}", max_inspected1 * max_inspected2);
}