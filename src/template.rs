use super::utils;
use super::utils::Print::*;

pub fn run(print : utils::Print ) {
    let day : &str = "0";

    assert!(day != "0", "CHANGE THE DAY");

    test(); // for any assertion tests

    match print {
        Part1 => part1(day),
        Part2 => part2(day),
        BothParts => { part1(day); part2(day); }
        NoParts => (),
    }
}

fn part1(day: &str) {
    
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        


    }
}

fn part2(day : &str) {
    
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        


    }
}

fn test() {

}