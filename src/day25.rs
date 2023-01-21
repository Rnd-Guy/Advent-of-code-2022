use super::utils;
use super::utils::Print::*;

// start 16:45
pub fn run(print : utils::Print ) {
    let day : &str = "25";

    assert!(day != "0", "CHANGE THE DAY");

    test(); // for any assertion tests

    match print {
        Part1 => part1(day),
        Part2 => part2(day),
        BothParts => { part1(day); part2(day); }
        NoParts => (),
    }
}
// 2 1 0 - =

fn part1(day: &str) {
    let mut machines : Vec<i128> = Vec::new();
    let mut total_sum = 0;
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        let chars : Vec<char> = line.chars().collect();

        let mut sum = 0;
        let mut pow = 0;
        for i in (0..chars.len()).rev() {
            let num;
            match chars[i] {
                '2' => num = 2,
                '1' => num = 1,
                '0' => num = 0,
                '-' => num = -1,
                '=' => num = -2,
                _ => panic!("char not found"),
            }
            sum += num * i128::pow(5,pow);
            pow += 1;
        }
        machines.push(sum);
        total_sum += sum;
    }

    // need to now convert it back into their number scheme
    let snafu = num_to_snafu(total_sum);

    println!("Day 25 part 1: {snafu}");
}

fn num_to_snafu(num: i128) -> String{
    //let mut digits = 0;
    let mut current_num = num;
    // find the number in base 5
    let mut base5 = 0;
    let mut pow = 0;
    while current_num > 0 {
        let divider = current_num / 5;
        let remainder = current_num % 5;
        base5 += remainder * i128::pow(10,pow);
        pow += 1;
        current_num = divider;
    }

    println!("{} in base 5: {}", num, base5);

    // then if 3 or 4 appear, then increase the next digit by 1 and replace 3 with = and 4 with -
    let mut base5: Vec<char> = base5.to_string().chars().collect();
    let mut carry = false;
    for i in (0..base5.len()).rev() {
        if base5[i] == '0' && carry {
            base5[i] = '1';
            carry = false;
        } else if base5[i] == '1' && carry {
            base5[i] = '2';
            carry = false;
        } else if (base5[i] == '3' && !carry) || (base5[i] == '2' && carry) {
            carry = true;
            base5[i] = '=';
        } else if (base5[i] == '4' && !carry) || (base5[i] == '3' && carry) {
            carry = true;
            base5[i] = '-';
        } else if base5[i] == '4' && carry {
            carry = true;
            base5[i] = '0';
        } else {
            carry = false;
        }
    }
    if carry {
        base5.insert(0,'1');
    }

    let mut s = String::new();
    for i in 0..base5.len() {
        s.push(base5[i]);
    }

    println!("converted to snafu: {}", s);

    return s;
}

fn part2(day : &str) {
    
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        


    }
}

fn test() {
    assert!(num_to_snafu(1) == "1".to_string());
    assert!(num_to_snafu(2) == "2".to_string());
    assert!(num_to_snafu(3) == "1=".to_string());
    assert!(num_to_snafu(4) == "1-".to_string());
    assert!(num_to_snafu(5) == "10".to_string());
    assert!(num_to_snafu(2022) == "1=11-2".to_string());
    assert!(num_to_snafu(12345) == "1-0---0".to_string());
    assert!(num_to_snafu(314159265) == "1121-1110-1=0".to_string());
}