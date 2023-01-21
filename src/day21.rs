use super::utils;
use super::utils::Print::*;

// start 09:31
pub fn run(print : utils::Print ) {
    let day : &str = "21";

    assert!(day != "0", "CHANGE THE DAY");

    test(); // for any assertion tests

    match print {
        Part1 => part1(day),
        Part2 => part2(day),
        BothParts => { part1(day); part2(day); }
        NoParts => (),
    }
}

struct Monkey {
    id: String,
    value: i128,
    has_value: bool,
    monkey1: String,
    monkey2: String,
    op: char,
}

struct Monkey2 {
    id: String,
    value: i128,
    has_value: bool,
    monkey1: String,
    monkey2: String,
    op: char,
    monkey1_val: i128,
    monkey1_partial: bool,
    monkey2_val: i128,
    monkey2_partial: bool,
    has_partial_value: bool,
}

impl Monkey {
    fn new_num(id: String, value: i128) -> Monkey {
        Monkey{id, value, has_value: true, monkey1: String::from(""), monkey2: String::from(""), op: ' '}
    }

    fn new_op(id: String, monkey1: String, op: char, monkey2: String) -> Monkey {
        Monkey{id, value: 0, has_value: false, monkey1, op, monkey2}
    }
}

impl Monkey2 {
    fn new_num(id: String, value: i128) -> Monkey2 {
        Monkey2{id, value, has_value: true, monkey1: String::from(""), monkey2: String::from(""), op: ' ', monkey1_val: 0, monkey1_partial: false, monkey2_val: 0, monkey2_partial: false, has_partial_value: false}
    }

    fn new_op(id: String, monkey1: String, op: char, monkey2: String) -> Monkey2 {
        Monkey2{id, value: 0, has_value: false, monkey1, op, monkey2, monkey1_val: 0, monkey1_partial: false, monkey2_val: 0, monkey2_partial: false, has_partial_value: false}
    }
}

fn part1(day: &str) {
    
    let mut monkeys: Vec<Monkey> = Vec::new();

    for line in utils::read_lines(day) {
        let line = line.unwrap();
        
        let parsed_line = utils::regex_to_vec(line, &r"^(.*): (?:([0-9]+)|([a-z]*) (.) (.*))$".to_string());
        let len = parsed_line[0].len();
        //println!("line: {:?}", parsed_line);
        // match always gives 6 groups, so we check if the [2] group is empty or not
        if parsed_line[0][2].len() > 0 {
            let val = parsed_line[0][2].parse().unwrap();
            monkeys.push(Monkey::new_num(parsed_line[0][1].to_string(), val))
        } else {
            let id = parsed_line[0][1].to_string();
            // [2] is the number group that doesn't get matched here
            let m1 = parsed_line[0][3].to_string();
            let op = parsed_line[0][4].chars().next().unwrap();
            let m2 = parsed_line[0][5].to_string();
            monkeys.push(Monkey::new_op(id, m1, op, m2));
        }
    }

    while !monkeys.iter().find(|m| m.id == "root".to_string()).unwrap().has_value {
        for i in 0..monkeys.len() {
            if monkeys[i].has_value {
                continue;
            }
            let m1 = monkeys.iter().find(|m| m.id == monkeys[i].monkey1).unwrap();
            if !m1.has_value {
                continue;
            }
            let m2 = monkeys.iter().find(|m| m.id == monkeys[i].monkey2).unwrap();
            if !m2.has_value {
                continue;
            }

            let m1_val = m1.value.clone();
            let m2_val = m2.value.clone();
            monkeys[i].value = match monkeys[i].op {
                '+' => m1_val + m2_val,
                '-' => m1_val - m2_val,
                '*' => m1_val * m2_val,
                '/' => m1_val / m2_val,
                _ => panic!("D:")
            };
            monkeys[i].has_value = true;
        }
    }

    let val = monkeys.iter().find(|m| m.id == "root".to_string()).unwrap().value;

    println!("Day 21 part 1: {val}");
}

// 10:06 start
fn part2(day : &str) {
    
let mut monkeys: Vec<Monkey2> = Vec::new();

    for line in utils::read_lines(day) {
        let line = line.unwrap();
        
        let parsed_line = utils::regex_to_vec(line, &r"^(.*): (?:([0-9]+)|([a-z]*) (.) (.*))$".to_string());
        let len = parsed_line[0].len();
        //println!("line: {:?}", parsed_line);
        // match always gives 6 groups, so we check if the [2] group is empty or not
        if parsed_line[0][2].len() > 0 {
            let val = parsed_line[0][2].parse().unwrap();
            monkeys.push(Monkey2::new_num(parsed_line[0][1].to_string(), val))
        } else {
            let id = parsed_line[0][1].to_string();
            // [2] is the number group that doesn't get matched here
            let m1 = parsed_line[0][3].to_string();
            let op = parsed_line[0][4].chars().next().unwrap();
            let m2 = parsed_line[0][5].to_string();
            monkeys.push(Monkey2::new_op(id, m1, op, m2));
        }
    }

    // now we ignore the humn and root and see how far we get
    let mut changed = true;
    while changed {
        changed = false;
        for i in 0..monkeys.len() {

            // note partial monkeys can still be full eventually
            if is_human(&monkeys[i]) || monkeys[i].has_value {
                continue;
            }

            // plan:
            // 1) if i am special or if i have been calculated already, skip me
            // 2) check if i can be calculated already by seeing if my two monkeys have values and isn't special
            // 3) check if either of my monkeys can be evaluated

            let mut special = false;
            let m1 = get_monkey(&monkeys, monkeys[i].monkey1.to_string());
            let m2 = get_monkey(&monkeys, monkeys[i].monkey2.to_string());

            if !is_special(m1) && !is_special(m2) && m1.has_value && m2.has_value {
                monkeys[i].value = op(m1.value, monkeys[i].op, m2.value);
                monkeys[i].has_value = true;
                changed = true;
            }

            let m1 = get_monkey(&monkeys, monkeys[i].monkey1.to_string());
            let m2 = get_monkey(&monkeys, monkeys[i].monkey2.to_string());

            if !is_special(m1) && m1.has_value && !monkeys[i].monkey1_partial {
                monkeys[i].monkey1_val = m1.value.clone();
                monkeys[i].has_partial_value = true;
                monkeys[i].monkey1_partial = true;
                changed = true;
            }

            let m1 = get_monkey(&monkeys, monkeys[i].monkey1.to_string());
            let m2 = get_monkey(&monkeys, monkeys[i].monkey2.to_string());

            if !is_special(m2) && m2.has_value && !monkeys[i].monkey2_partial {
                monkeys[i].monkey2_val = m2.value.clone();
                monkeys[i].has_partial_value = true;
                monkeys[i].monkey2_partial = true;
                changed = true;
            }

        }
    }

    // mostly debugging
    for i in 0..monkeys.len() {
        if !monkeys[i].has_value {
            let m1;
            if monkeys[i].monkey1_partial {
                m1 = monkeys[i].monkey1_val.to_string();
            } else {
                m1 = monkeys[i].monkey1.to_string();
            }
            let m2;
            if monkeys[i].monkey2_partial {
                m2 = monkeys[i].monkey2_val.to_string();
            } else {
                m2 = monkeys[i].monkey2.to_string();
            }
            //println!("{}: {} {} {}", monkeys[i].id, monkeys[i].monkey1, monkeys[i].op, monkeys[i].monkey2);
            println!("{}: {} {} {}", monkeys[i].id, m1, monkeys[i].op, m2);
        }
    }

    let root = get_monkey(&monkeys, "root".to_string());
    let initial_val = if root.monkey1_partial {root.monkey1_val} else {root.monkey2_val};;
    let first = if root.monkey1_partial {root.monkey2.to_string()} else {root.monkey1.to_string()};
    let humn = get_root_value(&monkeys, initial_val, first.to_string());

        // debugging
    for i in 0..monkeys.len() {
        if is_human(&monkeys[i]) {
            monkeys[i].value = humn;
            break;
        }
    }

    changed = true;
    while changed {
        changed = false;
        for i in 0..monkeys.len() {

            if is_root(&monkeys[i]) || monkeys[i].has_value {
                continue;
            }

            // plan:
            // 1) if i am special or if i have been calculated already, skip me
            // 2) check if i can be calculated already by seeing if my two monkeys have values and isn't special
            // 3) check if either of my monkeys can be evaluated

            let mut special = false;
            let m1 = get_monkey(&monkeys, monkeys[i].monkey1.to_string());
            let m2 = get_monkey(&monkeys, monkeys[i].monkey2.to_string());

            if m1.has_value && m2.has_value {
                monkeys[i].value = op(m1.value, monkeys[i].op, m2.value);
                monkeys[i].has_value = true;
                changed = true;
                println!("got value for {i}");
            }

            let m1 = get_monkey(&monkeys, monkeys[i].monkey1.to_string());
            let m2 = get_monkey(&monkeys, monkeys[i].monkey2.to_string());

            if is_root(m1) && m1.has_value && !monkeys[i].monkey1_partial {
                monkeys[i].monkey1_val = m1.value.clone();
                monkeys[i].has_partial_value = true;
                monkeys[i].monkey1_partial = true;
                changed = true;
            }

            let m1 = get_monkey(&monkeys, monkeys[i].monkey1.to_string());
            let m2 = get_monkey(&monkeys, monkeys[i].monkey2.to_string());

            if is_root(m2) && m2.has_value && !monkeys[i].monkey2_partial {
                monkeys[i].monkey2_val = m2.value.clone();
                monkeys[i].has_partial_value = true;
                monkeys[i].monkey2_partial = true;
                changed = true;
            }

        }
    }

    let root = get_monkey(&monkeys, "root".to_string());
    println!("root: {} {} {}", root.monkey1_val, root.op, root.monkey2_val);
    //println!("{}: {} {} {}", monkeys[i].id, monkeys[i].monkey1, monkeys[i].op, monkeys[i].monkey2);

    //let val = monkeys.iter().find(|m| m.id == "root".to_string()).unwrap().value;

    println!("Day 21 part 2: {humn}");
}

fn get_monkey(monkeys: &Vec<Monkey2>, id: String) -> &Monkey2 {
    monkeys.iter().find(|m| m.id == id).unwrap()
}

fn is_special(monkey: &Monkey2) -> bool {
    if monkey.id == "humn".to_string() || monkey.id == "root".to_string() {
        return true
    }
    return false
}

fn is_root(monkey: &Monkey2) -> bool {
    if monkey.id == "root".to_string() {
        return true
    }
    return false
}

fn is_human(monkey: &Monkey2) -> bool {
    if monkey.id == "humn".to_string() {
        return true
    }
    return false
}

fn op(val1: i128, op: char, val2: i128) -> i128 {
    match op {
        '+' => val1 + val2,
        '-' => val1 - val2,
        '*' => val1 * val2,
        '/' => { println!("{}", val1 % val2); val1 / val2},
        _ => panic!("D:")
    }
}

fn get_root_value(monkeys: &Vec<Monkey2>, current_val: i128, next: String) -> i128 {
    println!("current target: {}, next: {}", current_val, next);
    if next == "humn".to_string() {
        return current_val;
    }
    
    let m = get_monkey(monkeys, next.to_string());
    let op = m.op;
    let m1_p = m.monkey1_partial;
    let m2_p = m.monkey2_partial;
    let m1_v = m.monkey1_val;
    let m2_v = m.monkey2_val;
    let v = if {m1_p} {m1_v} else if {m2_p} {m2_v} else {panic!("ahh")};

    // m1 has partial
    // value = (10 + a) -> a = value - 10 
    // value = (10 - a) -> a = 10 - value
    // value = (10 * a) -> a = value / 10
    // value = (10 / a) -> a = 10 / value

    // m2 has partial
    // value = (a + 10) -> a = value - 10
    // value = (a - 10) -> a = value + 10
    // value = (a * 10) -> a = value / 10
    // value = (a / 10) -> a = value * 10

    let val = match op {
        '+' => current_val - v,
        '-' => {
            if m1_p {
                v - current_val
            } else {
                current_val + v
            }
        },
        '*' => { println!("{}", current_val % v); current_val / v},
        '/' => {
            if m1_p {
                { println!("{}", v % current_val); // testing to see if division ever results
                    v / current_val
                }
            } else {
                v * current_val
            }
        },
        _ => panic!("D:")
    };


    //println!("{val}");
    let n = if m1_p {m.monkey2.to_string()} else {m.monkey1.to_string()};
    return get_root_value(monkeys, val, n);

}

fn test() {

}