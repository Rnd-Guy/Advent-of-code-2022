use std::collections::HashMap;
use std::fs::File;
use std::io::Lines;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;
use lazy_static::lazy_static;
use std::sync::Mutex;

/**
 * Usage: 
 * for line in readLines(2) {
 *     let line = line.unwrap();
 *     // use line here
 * }
 */
pub fn read_lines(day: &str) -> Lines<BufReader<File>> {
    let path = format!("{}{}{}", "inputs/day", day, ".txt");
    let file = File::open(path).expect("could not read file D:");
    let buf = BufReader::new(file);
    buf.lines()
}

pub static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',
];

pub static ASCII_NUMBER: [char; 10] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
];

pub fn convert_ascii_to_index(c: char) -> u32 {

    if !c.is_ascii_alphabetic() {
        panic!("cannot convert non ascii to index. Attempted to convert [{c}]")
    }

    // this will treat lower and upper differently
    let mut value: u32 = 0;
    let mut letter = c;
    if c.is_ascii_uppercase() {
        value += 26;
        letter = c.to_lowercase().to_string().chars().next().unwrap();
    }

    // find value
    value += u32::try_from(ASCII_LOWER.iter().position(|x| x == &letter).unwrap()).unwrap();

    value
}

pub fn convert_ascii_to_number(c: char) -> i32 {

    if !c.is_ascii_digit() {
        panic!("cannot convert non ascii to index. Attempted to convert [{c}]")
    }

    i32::try_from(ASCII_NUMBER.iter().position(|x| x == &c).unwrap()).unwrap()
}

pub fn utils_test() {
    assert!(convert_ascii_to_index('a') == 0, "a gave back {} when it should be 0", convert_ascii_to_index('a'));
    assert!(convert_ascii_to_index('z') == 25, "a gave back {} when it should be 25", convert_ascii_to_index('z'));
    assert!(convert_ascii_to_index('A') == 26, "a gave back {} when it should be 26", convert_ascii_to_index('A'));
    assert!(convert_ascii_to_index('Z') == 51, "a gave back {} when it should be 51", convert_ascii_to_index('Z'));

    assert!(regex_to_vec("test line, x=123, x=456".to_string(), &r"(x=)([0-9]+\b)".to_string())[0][2] == "123");
}

// remember that the first match is the full match, so you will usually do result[0][1] instead of result[0][0], maybe i'll change later
pub fn regex_to_vec(line: String, reg: &String) -> Vec<Vec<String>> {
    lazy_static! {
        //static ref READ_SENSOR : Regex = Regex::new(r"x=([-]?[0-9]+)[^0-9]+y=([-]?[0-9]+)\b").unwrap();
        static ref REGI : Mutex<HashMap<String, Regex>> = Mutex::new(HashMap::new());
    }

    let mut data = REGI.lock().unwrap();
    if !data.contains_key(reg) {
        data.insert(reg.clone(), Regex::new(reg).unwrap());
    }
    return use_regex(line, data.get(reg).unwrap());
}

fn use_regex(line: String, reg: &Regex) -> Vec<Vec<String>>{
    let mut results: Vec<Vec<String>> = Vec::new();
    let matches = reg.captures_iter(&line);

    for m in matches.into_iter() {
        results.push(Vec::new());
        for c in m.iter() {
            if let None = c {
                results.last_mut().unwrap().push("".to_string());
            } else {
                results.last_mut().expect("e1").push(c.expect("e2").as_str().to_string());
            }
        }
    }
    results
}

#[allow(dead_code)]
pub enum Print {
    Part1,
    Part2,
    BothParts,
    NoParts
}