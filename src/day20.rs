use super::utils;
use super::utils::Print::*;

// start 09:42
pub fn run(print : utils::Print ) {
    let day : &str = "20";

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

    let mut numbers : Vec<(i32, usize)> = Vec::new();
    
    let mut line_number = 0;
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        
        numbers.push((line.parse().unwrap(), line_number));
        line_number += 1;

    }

    let len = numbers.len();
    println!("{len}");

    for i in 0..len {
        let item = numbers.iter().enumerate().find(|en| en.1.1 == i).unwrap();
        let current_index = item.0;
        let current_number = item.1;

        if current_number.0 == 0 {
            continue;
        }

        let mut new_index = (current_index as i32) + current_number.0;
        // circular means first index and last index are synonymous, so gotta do a little extra
        if new_index >= len as i32 {
            new_index += 1;
            new_index %= len as i32;
        }
        if new_index < 0 {
            new_index += len as i32;
            new_index -= 1; 
        }
        if new_index < 0 {
            new_index += len as i32;
            new_index -= 1; 
        }
        if new_index > current_index as i32 {
            //new_index += 1;
        }

        // debugging
        let mut check = current_index as i32 + current_number.0;
        if check < 0 {check += len as i32}
        check = check % len as i32;

        //println!("{current_index} -> {new_index}");
        let remove = numbers.remove(current_index);
        //numbers.insert(new_index as usize, current_number.clone());
        numbers.insert(new_index as usize, remove);

        let item2 = numbers.iter().enumerate().find(|en| en.1.1 == i).unwrap();
        assert!(&numbers.len() == &len);
        //assert!(check == item2.0 as i32, "fail, item {} was at index {current_index} and was supposed to now be at {check} but is instead at {}", item2.1.0, item2.0);
        //println!("{:?}", numbers);
    }

    let zero = numbers.iter().enumerate().find(|en| en.1.0 == 0).unwrap();
    let zero_index = zero.0;
    let mut answer = Vec::new();
    let mut sum = 0;
    for i in 1..=3 {
        answer.push(numbers.get((zero_index + 1000*i) % len).unwrap().0);
        sum += numbers.get((zero_index + 1000*i) % len).unwrap().0;
    }

    println!("{:?}", answer);
    println!("Day 20 part 1: {}", sum);

}

fn part2(day : &str) {
    
    let mut numbers : Vec<(i64, usize)> = Vec::new();
    
    let mut line_number = 0;
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        let val: i64 = line.parse().unwrap();
        let new_val = val * 811589153;
        numbers.push((new_val, line_number));
        line_number += 1;

    }

    let len = numbers.len();
    println!("{len}");

    for j in 0..10 {
        for i in 0..len {
            let item = numbers.iter().enumerate().find(|en| en.1.1 == i).unwrap();
            let current_index = item.0;
            let current_number = item.1;

            if current_number.0 == 0 {
                continue;
            }

            let mut new_index = (current_index as i64) + current_number.0;
            // circular means first index and last index are synonymous, so gotta do a little extra
            while !(0..len as i64).contains(&new_index) {
                let mut loops = new_index / len as i64;
                if loops < 0 {loops *= -1}
                if new_index >= len as i64 {
                    //new_index += 1;
                    new_index %= len as i64;
                    new_index += loops;
                }
                if new_index < 0 {
                    new_index %= len as i64;
                    new_index -= loops; 
                    new_index += len as i64;
                    new_index -= 1;
                }
            }

            // debugging
            let mut check = current_index as i64 + current_number.0;
            if check < 0 {check += len as i64}
            check = check % len as i64;

            //println!("{current_index} -> {new_index}");
            let remove = numbers.remove(current_index);
            //numbers.insert(new_index as usize, current_number.clone());
            numbers.insert(new_index as usize, remove);

            let item2 = numbers.iter().enumerate().find(|en| en.1.1 == i).unwrap();
            assert!(&numbers.len() == &len);
            //assert!(check == item2.0 as i64, "fail, item {} was at index {current_index} and was supposed to now be at {check} but is instead at {}", item2.1.0, item2.0);
        }
    }
    //println!("{:?}", numbers);

    let zero = numbers.iter().enumerate().find(|en| en.1.0 == 0).unwrap();
    let zero_index = zero.0;
    let mut answer = Vec::new();
    let mut sum = 0;
    for i in 1..=3 {
        answer.push(numbers.get((zero_index + 1000*i) % len).unwrap().0);
        sum += numbers.get((zero_index + 1000*i) % len).unwrap().0;
    }

    //println!("{:?}", answer);
    println!("Day 20 part 1: {}", sum);
}

fn test() {

}