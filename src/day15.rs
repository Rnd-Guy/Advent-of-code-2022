#![feature(mixed_integer_ops)]

use super::utils;
use super::utils::Print::*;
use std::cmp;
use regex::Regex;
use lazy_static::lazy_static;

// start 9:25
pub fn run(print : utils::Print ) {
    let day : &str = "15";

    assert!(day != "0", "CHANGE THE DAY");

    test(); // for any assertion tests

    match print {
        Part1 => part1(day),
        Part2 => part2(day),
        BothParts => { part1(day); part2(day); }
        NoParts => (),
    }
}

// number of items in row that are not able to contain a beacon cause another is closer
// we can look at each beacon and determine the range inaccessible for a certain y value

struct Sensor {
    position: (i32,i32),
    closest_beacon: (i32, i32),
    distance: u32,
}

impl Sensor {
    fn new(x: i32, y: i32, bx: i32, by: i32) -> Sensor {
        Sensor{
            position: (x,y), 
            closest_beacon: (bx, by), 
            distance: x.abs_diff(bx) + y.abs_diff(by),
        }
    }

    fn count_invalid_for_y(&self, y: i32) -> Option<(i32, i32)> {
        let dist = y.abs_diff(self.position.1);
        if dist > self.distance {
            return None;
        }

        return Some((self.position.0 - ((&self.distance-dist) as i32), self.position.0 + ((&self.distance-dist) as i32)));

    }

    fn count_invalid_for_y_bounded(&self, y: i32, lower_bound_x: i32, upper_bound_x: i32) -> Option<(i32, i32)> {
        let dist = y.abs_diff(self.position.1);
        if dist > self.distance {
            return None;
        }

        let mut range = (self.position.0 - ((&self.distance-dist) as i32), self.position.0 + ((&self.distance-dist) as i32));
        
        // no overlap with desired range
        if range.1 < lower_bound_x || range.0 > upper_bound_x {
            return None;
        }

        if range.0 < lower_bound_x {
            range.0 = lower_bound_x;
        }

        if range.1 > upper_bound_x {
            range.1 = upper_bound_x;
        }

        Some(range)
    }
}

fn part1(day: &str) {

    let mut sensors: Vec<Sensor> = Vec::new();
    let y = 2000000; // use 10 for test case, 2000000 if actual case
    
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        
        if line != "" {
            sensors.push(parse_line(line));
        }
    }

    let mut ranges : Vec<(i32, i32)> = Vec::new();
    for sensor in &sensors {
        let range = sensor.count_invalid_for_y(y);
        match range {
            Some(r) => {
                let mut overlapped = false;
                for existing_range in 0..ranges.len() {
                    // check if ranges overlap

                    if !(r.1 < ranges[existing_range].0 || r.0 > ranges[existing_range].1) {
                        println!("range overlap: ({}, {}) and ({}, {})", r.0, r.1, ranges[existing_range].0, ranges[existing_range].1);
                        overlapped = true;
                        ranges[existing_range] = (cmp::min(r.0, ranges[existing_range].0), cmp::max(r.1, ranges[existing_range].1));
                        println!("new range: ({}, {})", cmp::min(r.0, ranges[existing_range].0), cmp::max(r.1, ranges[existing_range].1));
                        break;
                    }
                }

                if !overlapped {
                    ranges.push(r);
                }
            },
            None => continue,
        }
    }

    // check if now any existing ranges overlap with each other
    let mut no_overlaps = false;
    'w: while !no_overlaps {
        if ranges.len() == 0 {
            panic!("No ranges D:")
        }
        for r1 in 0..ranges.len()-1 {
            for r2 in r1+1..ranges.len() {
                if !(ranges[r1].1 < ranges[r2].0 || ranges[r1].0 > ranges[r2].1) {
                    println!("range overlap: ({}, {}) and ({}, {})", ranges[r1].0, ranges[r1].1, ranges[r2].0, ranges[r2].1);
                    println!("new range: ({}, {})", cmp::min(ranges[r1].0, ranges[r2].0), cmp::max(ranges[r1].1, ranges[r2].1));
                    ranges[r1] = (cmp::min(ranges[r1].0, ranges[r2].0), cmp::max(ranges[r1].1, ranges[r2].1));
                    ranges.remove(r2);
                    continue 'w;
                }
            }
        }
        no_overlaps = true;
    }

    let mut sum = 0;
    for range in ranges {
        sum += (range.1 - range.0)+1;
    }

    // it doesn't count beacons already in said y value
    let mut beacons_on_y : Vec<i32> = Vec::new();
    for sensor in &sensors {
        if sensor.closest_beacon.1 == y && !beacons_on_y.contains(&sensor.closest_beacon.0) {
            beacons_on_y.push(sensor.closest_beacon.0);
        }
    }

    sum -= beacons_on_y.len() as i32;

    println!("Day 15 part 1: {sum}");

}

fn parse_line(line: String) -> Sensor {
    lazy_static! {
        static ref READ_SENSOR : Regex = Regex::new(r"x=([-]?[0-9]+)[^0-9]+y=([-]?[0-9]+)\b").unwrap();
    }

    // captures iter because it matches twice per line
    let mut captures = READ_SENSOR.captures_iter(&line);
    let s_location = captures.next().unwrap();
    let s_x = s_location.get(1).unwrap().as_str();
    let s_y = s_location.get(2).unwrap().as_str();

    let b_location = captures.next().unwrap();
    let b_x = b_location.get(1).unwrap().as_str();
    let b_y = b_location.get(2).unwrap().as_str();

    Sensor::new(s_x.parse().unwrap(), s_y.parse().unwrap(), b_x.parse().unwrap(), b_y.parse().unwrap())

}

fn part2(day : &str) {
    let mut sensors: Vec<Sensor> = Vec::new();
    
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        
        if line != "" {
            sensors.push(parse_line(line));
        }
    }

    let min_x = 0;
    let max_x = 4000000;

    'y: for y in 0..=4000000 {
        let mut ranges : Vec<(i32, i32)> = Vec::new();
        for sensor in &sensors {
            let range = sensor.count_invalid_for_y_bounded(y, min_x, max_x);
            match range {
                Some(r) => {
                    if r.0 == min_x && r.1 == max_x {
                        continue 'y;
                    }

                    let mut overlapped = false;
                    for existing_range in 0..ranges.len() {
                        // check if ranges overlap

                        if !(r.1 < ranges[existing_range].0 || r.0 > ranges[existing_range].1) {
                            overlapped = true;
                            ranges[existing_range] = (cmp::min(r.0, ranges[existing_range].0), cmp::max(r.1, ranges[existing_range].1));
                            break;
                        }
                    }

                    if !overlapped {
                        ranges.push(r);
                    }
                },
                None => continue,
            }
        }
    

        // check if now any existing ranges overlap with each other
        let mut no_overlaps = false;
        'w: while !no_overlaps {
            if ranges.len() == 0 {
                panic!("No ranges D:")
            }
            for r1 in 0..ranges.len()-1 {
                if ranges[r1].0 == min_x && ranges[r1].1 == max_x {
                    continue 'y;
                }
                for r2 in r1+1..ranges.len() {
                    if !(ranges[r1].1 < ranges[r2].0 || ranges[r1].0 > ranges[r2].1) {
                        ranges[r1] = (cmp::min(ranges[r1].0, ranges[r2].0), cmp::max(ranges[r1].1, ranges[r2].1));
                        ranges.remove(r2);
                        continue 'w;
                    }
                }
            }
            no_overlaps = true;
        }

        let mut sum = 0;
        for range in &ranges {
            sum += (range.1 - range.0)+1;
        }

        if sum == max_x - min_x {
            println!("{:?}", ranges);
            let mut x = -1;
            for r in &ranges {
                if r.0 == min_x {
                    println!("x is {}", r.1+1);
                    if x == -1 {
                        x = r.1+1;
                    } else if x != r.1+1{
                        panic!("mismatch D: {} vs {}", x, r.1+1);
                    }
                } else if r.1 == max_x {
                    println!("x is {}", r.0 - 1);
                    if x == -1 {
                        x = r.0-1;
                    } else if x != r.0-1 {
                        panic!("mismatch D: {} vs {}", x, r.0-1);
                    }
                }
            }
            println!("Day 15 part 2: {}", (x as u64 * max_x as u64)+y as u64);
            break;
        }
    }
}

fn test() {

}

// 