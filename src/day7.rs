use super::utils;
use super::utils::Print::*;
use std::collections::HashMap;
use lazy_static::lazy_static;

use regex::Regex;

pub fn run(print : utils::Print ) {
    let day = "7";

    match print {
        Part1 => part1(day),
        Part2 => part2(day),
        BothParts => { part1(day); part2(day); }
        NoParts => (),
    }
}

enum Command {
    Dir(String),
    File(String, u32),
    Cd(String),
    Ls
}

fn part1(day: &str) {

    let mut current_path: Vec<String> = vec!(s("/"));

    let mut directories: HashMap<String, u32> = HashMap::new();
    directories.insert(s("/"), 0);


    for line in utils::read_lines(day) {
        let line = line.unwrap();
        
        match parse_line(&line) {
            Command::Dir(name) => {
                directories.insert(current_path.join("/").to_string() +"/" + &name, 0);
            },
            Command::File(_name, size) => {
                
                // for folder in &current_path {
                //     //let current_size = directories.entry(folder.to_string()).or_insert(0);
                //     directories.entry(folder.to_string()).and_modify(|i| *i += size);
                //     //*current_size += size;
                // }
                for i in 1..current_path.len()+1 {
                    let folder_name = current_path[0..i].join("/");
                    directories.entry(folder_name.to_string()).and_modify(|i| *i += size);
                }
                
            },
            Command::Ls => (),
            Command::Cd(name) => {
                if name == ".." {
                    current_path.pop();
                } else if name == "/" {
                    current_path = vec!(s("/"));
                } else {
                    current_path.push(name);
                }
                //println!("{}", current_path.join(","));
            },
        }
    }

    let mut sum = 0;
    for dir in directories {
        //println!("Dir: {}, Size: {}", dir.0, dir.1);
        if dir.1 < 100000 {
            sum += dir.1;
        }
    }

    println!("Day 7 part 1: {sum}");
}

fn parse_line(line: &str) -> Command {
    lazy_static! {
        static ref IS_DIR : Regex = Regex::new(r"^dir (.*)$").unwrap();
        static ref IS_FILE : Regex = Regex::new(r"^([0-9]+) (.*)$").unwrap();
        static ref IS_CD : Regex = Regex::new(r"^\$ cd (.*)$").unwrap();
        static ref IS_LS : Regex = Regex::new(r"^\$ ls$").unwrap();
    }

    if IS_DIR.is_match(line) { 
        return Command::Dir(IS_DIR.captures(line).unwrap().get(1).unwrap().as_str().to_string());
    }
    else if IS_FILE.is_match(line) {
        let caps = IS_FILE.captures(line).unwrap();
        let size: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let name = caps.get(2).unwrap().as_str().to_string();
        return Command::File(name, size);
    }
    else if IS_LS.is_match(line) {return Command::Ls}
    else if IS_CD.is_match(line) {
        return Command::Cd(IS_CD.captures(line).unwrap().get(1).unwrap().as_str().to_string());
    } else {

        panic!("Line type could not be parsed D:, line was {line}")
    }
}

fn s(string: &str) -> String {
    String::from(string)
}

fn part2(day : &str) {

    let mut current_path: Vec<String> = vec!(s("/"));

    let mut directories: HashMap<String, u32> = HashMap::new();
    directories.insert(s("/"), 0);


    for line in utils::read_lines(day) {
        let line = line.unwrap();
        
        match parse_line(&line) {
            Command::Dir(name) => {
                // FOLDER NAMES CAN BE DUPLICATED
                directories.insert(current_path.join("/").to_string() +"/" + &name, 0);
            },
            Command::File(_name, size) => {
                
                // for folder in &current_path {
                //     //let current_size = directories.entry(folder.to_string()).or_insert(0);
                //     directories.entry(folder.to_string()).and_modify(|i| *i += size);
                //     //*current_size += size;
                // }
                for i in 1..current_path.len()+1 {
                    let folder_name = current_path[0..i].join("/");
                    directories.entry(folder_name.to_string()).and_modify(|i| *i += size);
                }
                
            },
            Command::Ls => (),
            Command::Cd(name) => {
                if name == ".." {
                    current_path.pop();
                } else if name == "/" {
                    current_path = vec!(s("/"));
                } else {
                    current_path.push(name);
                }
                //println!("{}", current_path.join(","));
            },
        }
    }

    let total = directories.get(&"/".to_string()).unwrap();
    let size_to_remove = total - 40000000;
    let mut min_to_remove: u32 = *total;

    for dir in directories {
        if dir.1 > size_to_remove && dir.1 < min_to_remove {
            min_to_remove = dir.1;
        }
    }

    println!("Day 7 part 2: {min_to_remove}");
}


// original attempt that didnt work
// struct Data {
//     name: String,
//     data_type: DataType
// }

// enum DataType {
//     Dir(Vec<Data>),
//     File(u32),
// }

// impl Data {
//     fn newDir(name: String) -> Data {
//         Data{name, data_type: DataType::Dir(Vec::new())}
//     }

//     fn newFile(name: String, size: u32) -> Data {
//         Data{name, data_type: DataType::File(size)}
//     }

//     fn childDir(&mut self, name: String) {
//         match &self.data_type {
//             DataType::Dir(dir) => (*dir).push(Data::newDir(name)),
//             DataType::File(_) => panic!("Ahh a file cant have children D:")
//         }
//     }

//     fn childFile(&mut self, name: String, size: u32) {
//         match &self.data_type {
//             DataType::Dir(dir) => dir.push(Data::newFile(name, size)),
//             DataType::File(_) => panic!("Ahhx2, files cant be parents")
//         }
//     }

//     fn getSize(&self) -> u32 {
//         match &self.data_type {
//             DataType::Dir(dir) => {
//                 let mut sum = 0;
//                 for d in dir {
//                     sum += d.getSize();
//                 }
//                 sum
//             },
//             DataType::File(file) => *file
//         }
//     }

//     fn get(&self, path: Vec<String>) -> Option<&Data> {
//         if path.len() == 0 {
//             return Some(self);
//         }
//         match &self.data_type {
//             DataType::Dir(dir) => {
//                 let mut newPath = path.clone();
//                 let nextIndex = newPath.remove(0);
//                 match dir.iter().find(|i| i.name == nextIndex) {
//                     Some(child) => child.get(newPath),
//                     None => None
//                 }
//             },
//             DataType::File(_) => None
//         }
//     }