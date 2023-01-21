use std::cmp::Ordering;

use super::utils;
use super::utils::Print::*;

pub fn run(print : utils::Print ) {
    let day : &str = "13";

    assert!(day != "0", "CHANGE THE DAY");

    test();

    match print {
        Part1 => part1(day),
        Part2 => part2(day),
        BothParts => { part1(day); part2(day); }
        NoParts => (),
    }
}

fn part1(day: &str) {

    let mut current_pair = 1;
    let mut is_line_one = true;
    
    let mut left_item: Item = Item::new_list();
    let mut right_item: Item = Item::new_list();

    let mut ordered_pairs: Vec<i32> = Vec::new();
    
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        
        if line == "" {
            match compare_item(&mut left_item, &mut right_item) {
                Ordering::Greater => {
                    println!("pair {current_pair} is greater");
                },
                Ordering::Less => {
                    ordered_pairs.push(current_pair);
                    println!("pair {current_pair} is less")
                },
                Ordering::Equal => panic!("Top level items compared as equal which should never happen!")
            }

            current_pair += 1;
            is_line_one = true;
            continue;

        }

        if is_line_one {
            //println!("Line 1 is {}", &line);
            left_item = parse_line(line);
            is_line_one = false;
        } else {
            //println!("Line 2 is {}", &line);
            right_item = parse_line(line);
        }

    }
    let mut sum = 0;
    for p in ordered_pairs {
        sum += p;
    }
    println!("Day 13 part 1: {sum}");
}

fn parse_line(line: String) -> Item {
    let mut chars = line.as_bytes();
    let mut index = 1;
    //println!("{line}");
    let item = parse_list(&mut chars, &mut index);
    



    item
}

fn parse_list(chars: &[u8], index: &mut usize) -> Item {
    let mut list = Item::new_list();


    let mut current_int = -1;
    loop {
        if index >= &mut chars.len() {
            //println!("end of list");
            return list;
        }
        let c = chars[*index];
        //println!("index: {index}, c: [{}], current int: {current_int}", c as char);
        match c as char {
            '[' => {
                *index += 1;
                list.get_list_mut().push(parse_list(chars, &mut (*index)));
            },
            '0'..='9' => {
                //println!("int");
                let new_number = utils::convert_ascii_to_number(c as char);
                if current_int >= 0 {
                    //println!("old int: {current_int}");
                    current_int *= 10;
                    //current_int += c.to_string().parse::<i32>().unwrap();
                    //current_int += c.to_string().parse::<i32>().unwrap();
                    current_int += new_number;
                    //println!("new int: {current_int}");
                } else {
                    //current_int = c.to_string().parse().unwrap();
                    current_int = new_number;
                    //println!("new int found, first number: {current_int}");
                }
            },
            ',' | ' ' => {
                if current_int >= 0 {
                    (list.get_list_mut()).push(Item::Int(current_int));
                    current_int = -1;
                    //println!("Storing int: {current_int}");
                }
            },
            ']' => {
                if current_int >= 0 {
                    //println!("Storing int: {current_int}");
                    list.get_list_mut().push(Item::Int(current_int));
                }
                *index += 1;
                break;
            },
            _ => panic!("c isn't one of the matches D:")

        }
        *index += 1;
    }

    list
}

#[derive(Debug)]
enum Item {
    Int(i32),
    List(Box<Vec<Item>>)
}

impl Item {
    fn is_int(&self) -> bool {
        match *self {
            Item::Int(_) => true,
            Item::List(_) => false
        }
    }

    fn get_int(&self) -> i32 {
        if let Item::Int(i) = self {
            *i
        } else {
            panic!("Not an int!");
        }
    }

    fn get_list_mut(&mut self) -> &mut Box<Vec<Item>> {
        if let Self::List(l) = self {
            &mut(*l)
        } else {
            panic!("Not a list!");
        }
       
    }

    fn get_list(&self) -> &Box<Vec<Item>> {
        if let Self::List(l) = self {
            l
        } else {
            panic!("Not a list!");
        }
       
    }

    fn new_list() -> Item {
        Item::List(Box::new(Vec::new()))
    }

    fn to_string(&self) -> String {
        match self {
            Item::Int(i) => i.to_string(),
            Item::List(l) => {
                let mut s = "[".to_string();
                for item in 0..l.len() {
                    s.push_str(l[item].to_string().as_str());
                    if item < l.len() - 1 {
                        s.push(',');
                    }
                }
                s.push(']');
                s
            }
        }
    }
}

fn compare_item(left: &Item, right: &Item) -> Ordering {
    
    if left.is_int() && right.is_int() {
        return left.get_int().cmp(&right.get_int());
    } else if !left.is_int() && !right.is_int() {
        let left_list = left.get_list();
        let right_list = right.get_list();

        for index in 0..left_list.len() {
            if right_list.len() <= index {
                return Ordering::Greater;
            }

            let left_item = &left_list[index];
            let right_item = &right_list[index];

            match compare_item(&left_item, &right_item) {
                Ordering::Greater => return Ordering::Greater,
                Ordering::Equal => continue,
                Ordering::Less => return Ordering::Less,
            }
            
        }

        // left list is over
        if left_list.len() < right_list.len() {
            return Ordering::Less;
        } else {
            return Ordering::Equal;
        }
    } else {
        if left.is_int() {
            //let new_left = //vec!(Item::List(Box::new(Item::List(Int(left.get_int())))));
            let new_left = Item::List(Box::new(vec!(Item::Int(left.get_int()))));
            return compare_item(&new_left, right);

        } else if right.is_int() {
            let new_right = Item::List(Box::new(vec!(Item::Int(right.get_int()))));
            return compare_item(left, &new_right);
        } else {
            panic!("no clause matched at all D:")
        }
    }
}

fn part2(day : &str) {
    
    let mut items: Vec<Item> = Vec::new();
    items.push(parse_line("[[2]]".to_string()));
    items.push(parse_line("[[6]]".to_string()));
    
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        
        if line != "" {
            items.push(parse_line(line));
        }

    }
    items.sort_by(|a, b| compare_item(a,  b));

    let mut answer = 1; // find index of [[2]] and [[6]]
    for i in 0..items.len() {
        println!("{}", items[i].to_string());
        match compare_item(&mut items[i], &mut parse_line("[[2]]".to_string())) {
            Ordering::Equal => {
                answer *= i+1;
                println!("found [[2]]: {i}");
            },
            _ => ()
        }
        match compare_item(&mut items[i], &mut parse_line("[[6]]".to_string())) {
            Ordering::Equal => {
                answer *= i+1;
                println!("found [[6]]: {i}");
                //break;
            },
            _ => ()
        }
    }

    println!("Day 13 part 2: {answer}");
}

fn test() {
    // parse line
    // parse list
    let test = "[]";
    let mut index: usize = 1;
    let item = parse_list(&mut test.as_bytes(), &mut index);
    match item {
        Item::Int(_) => panic!("not "),
        Item::List(l) => assert!(l.len() == 0)
    }


    // compare item
    let test = "[]";
    index = 1;
    let item = parse_list(&mut test.as_bytes(), &mut index);
    match item {
        Item::Int(_) => panic!("not "),
        Item::List(l) => assert!(l.len() == 0)
    }
    let item = parse_line(test.to_string());
    match item {
        Item::Int(_) => panic!("not "),
        Item::List(l) => assert!(l.len() == 0)
    }

    //println!("offending test starts here");
    let test = "[1]";
    index = 1;
    let item = parse_list(&mut test.as_bytes(), &mut index);
    match item {
        Item::Int(_) => panic!("not "),
        Item::List(l) => {
            //println!("{:?}", l);
            assert!(l.len() == 1);
            match l[0] {
                Item::Int(i) => assert!(i == 1),
                Item::List(_) => panic!("not list")
            }
        }
    }
    let item = parse_line(test.to_string());
    match item {
        Item::Int(_) => panic!("not "),
        Item::List(l) => {
            assert!(l.len() == 1);
            match l[0] {
                Item::Int(i) => assert!(i == 1),
                Item::List(_) => panic!("not list")
            }
        }
    }

    let test = "[11, 2]";
    let item = parse_line(test.to_string());
    match item {
        Item::Int(_) => panic!("not "),
        Item::List(l) => {
            assert!(l.len() == 2);
            match l[0] {
                Item::Int(i) => assert!(i == 11),
                Item::List(_) => panic!("not list")
            }
            match l[1] {
                Item::Int(i) => assert!(i == 2),
                Item::List(_) => panic!("not list")
            }
        }
    }

    let test = "[[1]]";
    let item = parse_line(test.to_string());
    match item {
        Item::Int(_) => panic!("not "),
        Item::List(l) => {
            assert!(l.len() == 1);
            match &l[0] {
                Item::Int(_) => panic!("not int"),
                Item::List(m) => {
                    assert!(m.len() == 1);
                    match m[0] {
                        Item::Int(i) => assert!(i == 1),
                        Item::List(_) => panic!("not a list")
                    }
                }
            }
        }
    }

    match compare_item(&mut parse_line("[1]".to_string()), &mut parse_line("[2]".to_string())) {
        Ordering::Less => (),
        _ => panic!("Not correct!")
    }
    match compare_item(&mut parse_line("[2]".to_string()), &mut parse_line("[1]".to_string())) {
        Ordering::Greater => (),
        _ => panic!("Not correct!")
    }
    match compare_item(&mut parse_line("[1,1]".to_string()), &mut parse_line("[1,2]".to_string())) {
        Ordering::Less => (),
        _ => panic!("Not correct!")
    }
    match compare_item(&mut parse_line("[1]".to_string()), &mut parse_line("[1,2]".to_string())) {
        Ordering::Less => (),
        _ => panic!("Not correct!")
    }
    match compare_item(&mut parse_line("[1,1]".to_string()), &mut parse_line("[1]".to_string())) {
        Ordering::Greater => (),
        _ => panic!("Not correct!")
    }

}