use super::utils;
use super::utils::Print::*;

use std::collections::{HashMap, HashSet};

pub fn run(print : utils::Print ) {
    let day : &str = "16t";

    assert!(day != "0", "CHANGE THE DAY");

    test(); // for any assertion tests

    match print {
        Part1 => part1(day),
        Part2 => part2(day),
        BothParts => { part1(day); part2(day); }
        NoParts => (),
    }
}

struct Valve {
    name: String,
    flow: u32,
    leads: Vec<String>,
}


struct Valve2 {
    name: String,
    flow: u32,
    leads: HashSet<String>,
    id: u32,
}

impl Valve2 {
    fn new(s: &str) -> Self {
        Self{name: s.to_string(), id: 0, flow: 0, leads: HashSet::new()}
    }
}

impl std::hash::Hash for Valve2 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Valve2 {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Clone for Valve2 {
    fn clone(&self) -> Self {
        Self { name: self.name.to_string(), flow: self.flow.clone(), leads: self.leads.clone(), id: self.id.clone() }
    }
}

impl Eq for Valve2 {}

fn part1(day: &str) {

    let mut valves : Vec<Valve> = Vec::new();
    
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        
        valves.push(parse_line(line));

    }

    let time = 30;

    let visited = 0;

    //let max = test_every_combination(&valves, &valves[0], &Vec::new(), 0, 0, time, visited);
    //println!("Day 16 part 1: {i} {max}");
    let valve_aa = valves.iter().find(|v| v.name == "AA".to_string()).unwrap();

    let mut memos : HashMap<ValveMemo, u32> = HashMap::new();
    let max = memoize_combinations(&mut memos, &valves, &valve_aa, &Vec::new(), 0, 0, time, visited);

    println!("Day 16 part 1: {max}");
}

#[allow(dead_code)]
fn test_every_combination(valves: &Vec<Valve>, current_valve: &Valve, valves_open: &Vec<String>,  flow_per_time: u32, total_flow: u32, time: u32, visited: u32) -> u32{
    // steps:
    // on each valve: 
    // check if time is up, return time
    // try the following:
    //    try open (if not opened already)
    //    move to one of the other connected valves
    //println!("{time}: {visited}, {total_flow}");

    if time == 0 {
        return total_flow;
    }

    let total_flow = flow_per_time + total_flow;
    let mut combinations: Vec<u32> = Vec::new();

    // try opening current valve
    if !valves_open.contains(&current_valve.name) && current_valve.flow != 0 {
        let valves_open = open_valve(&valves_open, current_valve.name.clone());
        let flow_per_time = flow_per_time + current_valve.flow;
        //current_path.push_str("open, ");
        combinations.push(test_every_combination(valves, current_valve, &valves_open, flow_per_time, total_flow, time-1, visited+1));
    }
    for s in &current_valve.leads {
        let new_valve = valves.iter().find(|v| &v.name == s).unwrap();
        //current_path.push_str(s);
        //current_path.push_str(", ");
        combinations.push(test_every_combination(valves, new_valve, valves_open, flow_per_time, total_flow, time-1, visited+1));
    }

    let mut max = 0;
    if combinations.len() == 0 {
        return 0;
    }
    for c in combinations {
        if max < c {
            max = c;
        }
    }
    return max;
}

#[derive(PartialEq,Eq,Hash)]
struct ValveMemo {
    valve: String,
    open: u64, // sum of open valves where one valve = 2^index
    time_remaining: u32,
}

#[derive(PartialEq,Eq,Hash)]
struct ValveMemo2 {
    valve: (u32, u32), // index of the two valves me and elephant is in, with the lower numbered valve first
    open: u64,
    time_remaining: u32,

}

fn memoize_combinations(memos: &mut HashMap<ValveMemo, u32>, valves: &Vec<Valve>, current_valve: &Valve, valves_open: &Vec<String>,  flow_per_time: u32, total_flow: u32, time: u32, visited: u32) -> u32{
    // steps:
    // on each valve: 
    // check if time is up, return time
    // check if memo has an entry matching "with this much time left, and these valves open, and on this valve, i can gain this much more flow"
    // try the following:
    //    try open (if not opened already). add to memo
    //    move to one of the other connected valves. add to memo
    // add current to memo
    // return 
    //println!("{time}: {visited}, {total_flow}");

    // memo
    // the memo tells me how much flow i can gain if i move to that spot with that much time left and those valves open
    // therefore the memo tells me how much flow i can still gain
    
    // calculation steps:
    // 1) add to total flow the flow per time
    // 2) perform action
    //   a) move to valve 
    //   b) open valve (flow per time increases)

    // this means: i will not gain any more from reaching time 0 as that has already been taken into account
    // 

    if time == 0 {
        return total_flow;
    }

    let new_total_flow = flow_per_time + total_flow;
    let mut combinations: Vec<u32> = Vec::new();

    // try opening current valve
    if !valves_open.contains(&current_valve.name) && current_valve.flow != 0 {
        let valves_open = open_valve(&valves_open, current_valve.name.clone());
        let memo = generate_memo(valves, current_valve, &valves_open ,time-1);
        if memos.contains_key(&memo) {
            //println!("FOUND MEMO");
            combinations.push(new_total_flow + *memos.get(&memo).unwrap());
        } else {
            let flow_per_time = flow_per_time + current_valve.flow;
            combinations.push(memoize_combinations(memos, valves, current_valve, &valves_open, flow_per_time, new_total_flow, time-1, visited+1));
        }
            
    }
    for s in &current_valve.leads {
        let new_valve = valves.iter().find(|v| &v.name == s).unwrap();
        let memo = generate_memo(valves, &new_valve, valves_open ,time-1);
        if memos.contains_key(&memo) {
            //println!("FOUND MEMO");
            combinations.push(new_total_flow + *memos.get(&memo).unwrap());
        } else {
            combinations.push(memoize_combinations(memos, valves, new_valve, valves_open, flow_per_time, new_total_flow, time-1, visited+1));
        }
    }

    let mut max = 0;
    if combinations.len() == 0 {
        return 0;
    }
    for c in combinations {
        if max < c {
            max = c;
        }
    }
    memos.insert(generate_memo(valves, current_valve, valves_open ,time), max - total_flow);
    //println!("{:?}", memos.values());
    return max;
}

fn memoize_combinations2(memos: &mut HashMap<ValveMemo2, u32>, valves: &Vec<Valve>, my_valve: &Valve, e_valve: &Valve, valves_open: &Vec<String>,  flow_per_time: u32, total_flow: u32, time: u32, my_old_valve: &Valve, e_old_valve: &Valve) -> u32{

    if time == 0 {
        return total_flow;
    }

    let new_total_flow = flow_per_time + total_flow;
    let mut my_possible_actions: Vec<String> = Vec::new();
    let mut e_possible_actions: Vec<String> = Vec::new();
    let mut total_possible_actions: Vec<(String, String)> = Vec::new();

    // prevent duplicate actions ie i go to B and e goes to C along with i go to C and e goes to B
    // this approach also handles making sure only one person opens the valve if both are on the same valve
    if my_valve.name == e_valve.name {
        let mut total_leads = Vec::new();
        if !valves_open.contains(&my_valve.name) && my_valve.flow != 0 {
            total_leads.push("open".to_string());
        }
        for v in &my_valve.leads {
            // prevent either me or elephant going to where either came from, as me going to the elephants old position is the same as
            if !(v.to_string() == my_old_valve.name || v.to_string() == e_old_valve.name) {
                total_leads.push(v.to_string());
            }
        }

        if total_leads.len() > 0 {
            for i in 0..total_leads.len()-1 {
                for j in i+1..total_leads.len() {
                    total_possible_actions.push((total_leads[i].to_string(), total_leads[j].to_string()));
                }
            }
        }
    } else {
        if !valves_open.contains(&my_valve.name) && my_valve.flow != 0 {
            my_possible_actions.push("open".to_string());
        }
        // note the duplicate valve scenario doesn't occur here anymore
        if !valves_open.contains(&e_valve.name) && e_valve.flow != 0 {
            e_possible_actions.push("open".to_string());
        }
        for s in &my_valve.leads {
            if s != &my_old_valve.name {
                my_possible_actions.push(s.clone());
            }
        }
        for s in &e_valve.leads {
            if s != &e_old_valve.name {
                e_possible_actions.push(s.clone());
            }
        }

        for i in &my_possible_actions {
            for j in &e_possible_actions {
                total_possible_actions.push((i.to_string(), j.to_string()));
            }
        }
    }

    let mut combinations: Vec<u32> = Vec::new();

    for t in total_possible_actions {

        let i = t.0;
        let j = t.1;
        let mut flow_per_time = flow_per_time;
        let mut valves_open = copy_valve(valves_open);
        let mut my_new_valve = my_valve;
        let mut e_new_valve = e_valve;

        if i.eq("open") {
            flow_per_time += my_new_valve.flow;
            valves_open = open_valve(&valves_open, my_new_valve.name.clone());
        } else {
            my_new_valve = valves.iter().find(|v| v.name == i).unwrap();
        }

        if j.eq("open") {
            flow_per_time += e_new_valve.flow;
            valves_open = open_valve(&valves_open, e_new_valve.name.clone());
        } else {
            e_new_valve = valves.iter().find(|v| v.name == j).unwrap();
        }

        let memo = generate_memo2(valves, my_new_valve, e_new_valve, &valves_open ,time-1);
        if memos.contains_key(&memo) {
            //combinations.push(new_total_flow + *memos.get(&memo).unwrap());
            combinations.push(new_total_flow + *memos.get(&memo).unwrap());
        } else {
            combinations.push(memoize_combinations2(memos, valves, my_new_valve, e_new_valve,  &valves_open, flow_per_time, new_total_flow, time-1, my_valve, e_valve));
        }
    }
    let mut max = 0;
    if combinations.len() == 0 {
        // this happens due to the anti-back and forth filter
        // when this happens, it should be safe to assume that the result is optimised or severely unoptimal and there's no point continuing any further in this branch
        memos.insert(generate_memo2(valves, my_valve, e_valve, valves_open ,time), time * flow_per_time);
        return total_flow + time * flow_per_time;
    }
    for c in combinations {
        if max < c {
            max = c;
        }
    }
    memos.insert(generate_memo2(valves, my_valve, e_valve, valves_open ,time), max - total_flow);
    //println!("{:?}", memos.values());
    return max;
}

// somehow slower
fn memoize_combinations2_no_loops(memos: &mut HashMap<ValveMemo2, u32>, valves: &Vec<Valve>, my_valve: &Valve, e_valve: &Valve, valves_open: &Vec<String>,  flow_per_time: u32, total_flow: u32, time: u32, my_old_valves: &Vec<String>, e_old_valves: &Vec<String>) -> u32{

    if time == 0 {
        return total_flow;
    }

    let new_total_flow = flow_per_time + total_flow;
    let mut my_possible_actions: Vec<String> = Vec::new();
    let mut e_possible_actions: Vec<String> = Vec::new();
    let mut total_possible_actions: Vec<(String, String)> = Vec::new();


    // prevent duplicate actions ie i go to B and e goes to C along with i go to C and e goes to B
    // this approach also handles making sure only one person opens the valve if both are on the same valve
    if my_valve.name == e_valve.name {
        let mut total_leads = Vec::new();
        if !valves_open.contains(&my_valve.name) && my_valve.flow != 0 {
            total_leads.push("open".to_string());
        }
        for v in &my_valve.leads {
            // prevent either me or elephant going to where either came from, as me going to the elephants old position is the same as
            if !(my_old_valves.contains(v) || e_old_valves.contains(v)) {
                total_leads.push(v.to_string());
            }
        }

        if total_leads.len() > 0 {
            for i in 0..total_leads.len()-1 {
                for j in i+1..total_leads.len() {
                    total_possible_actions.push((total_leads[i].to_string(), total_leads[j].to_string()));
                }
            }
        }
    } else {
        if !valves_open.contains(&my_valve.name) && my_valve.flow != 0 {
            my_possible_actions.push("open".to_string());
        }
        // note the duplicate valve scenario doesn't occur here anymore
        if !valves_open.contains(&e_valve.name) && e_valve.flow != 0 {
            e_possible_actions.push("open".to_string());
        }
        for s in &my_valve.leads {
            if !my_old_valves.contains(s) {
                my_possible_actions.push(s.clone());
            }
        }
        for s in &e_valve.leads {
            if !e_old_valves.contains(s) {
                e_possible_actions.push(s.clone());
            }
        }

        for i in &my_possible_actions {
            for j in &e_possible_actions {
                total_possible_actions.push((i.to_string(), j.to_string()));
            }
        }
    }

    let mut combinations: Vec<u32> = Vec::new();
    for t in total_possible_actions {

        let i = t.0;
        let j = t.1;
        let mut flow_per_time = flow_per_time;
        let mut valves_open = copy_valve(valves_open);
        let mut my_new_valve = my_valve;
        let mut e_new_valve = e_valve;
        let mut my_new_valves = copy_valve(my_old_valves);
        let mut e_new_valves = copy_valve(e_old_valves);

        if i.eq("open") {
            flow_per_time += my_new_valve.flow;
            valves_open = open_valve(&valves_open, my_new_valve.name.clone());
            my_new_valves = Vec::new();
        } else {
            my_new_valve = valves.iter().find(|v| v.name == i).unwrap();
            my_new_valves.push(i);
        }

        if j.eq("open") {
            flow_per_time += e_new_valve.flow;
            valves_open = open_valve(&valves_open, e_new_valve.name.clone());
            e_new_valves = Vec::new();
        } else {
            e_new_valve = valves.iter().find(|v| v.name == j).unwrap();
            e_new_valves.push(j);
        }

        let memo = generate_memo2(valves, my_new_valve, e_new_valve, &valves_open ,time-1);
        if memos.contains_key(&memo) {
            //combinations.push(new_total_flow + *memos.get(&memo).unwrap());
            combinations.push(new_total_flow + *memos.get(&memo).unwrap());
        } else {
            combinations.push(memoize_combinations2_no_loops(memos, valves, my_new_valve, e_new_valve,  &valves_open, flow_per_time, new_total_flow, time-1, &my_new_valves, &e_new_valves));
        }
    }
    let mut max = 0;
    if combinations.len() == 0 {
        // this happens due to the anti-back and forth filter
        // when this happens, it should be safe to assume that the result is optimised or severely unoptimal and there's no point continuing any further in this branch
        memos.insert(generate_memo2(valves, my_valve, e_valve, valves_open ,time), time * flow_per_time);
        return total_flow + time * flow_per_time;
    }
    for c in combinations {
        if max < c {
            max = c;
        }
    }
    memos.insert(generate_memo2(valves, my_valve, e_valve, valves_open ,time), max - total_flow);
    //println!("{:?}", memos.values());
    return max;
}

// somehow still slower
fn memoize_combinations2_no_loops_hash(memos: &mut HashMap<ValveMemo2, u32>, valves: &Vec<Valve>, my_valve: &Valve, e_valve: &Valve, valves_open: &Vec<String>,  flow_per_time: u32, total_flow: u32, time: u32, my_old_valves: &HashSet<String>, e_old_valves: &HashSet<String>) -> u32{

    if time == 0 {
        return total_flow;
    }

    let new_total_flow = flow_per_time + total_flow;
    let mut my_possible_actions: Vec<String> = Vec::new();
    let mut e_possible_actions: Vec<String> = Vec::new();
    let mut total_possible_actions: Vec<(String, String)> = Vec::new();


    // prevent duplicate actions ie i go to B and e goes to C along with i go to C and e goes to B
    // this approach also handles making sure only one person opens the valve if both are on the same valve
    if my_valve.name == e_valve.name {
        let mut total_leads = Vec::new();
        if !valves_open.contains(&my_valve.name) && my_valve.flow != 0 {
            total_leads.push("open".to_string());
        }
        for v in &my_valve.leads {
            // prevent either me or elephant going to where either came from, as me going to the elephants old position is the same as
            if !(my_old_valves.contains(v) || e_old_valves.contains(v)) {
                total_leads.push(v.to_string());
            }
        }

        if total_leads.len() > 0 {
            for i in 0..total_leads.len()-1 {
                for j in i+1..total_leads.len() {
                    total_possible_actions.push((total_leads[i].to_string(), total_leads[j].to_string()));
                }
            }
        }
    } else {
        if !valves_open.contains(&my_valve.name) && my_valve.flow != 0 {
            my_possible_actions.push("open".to_string());
        }
        // note the duplicate valve scenario doesn't occur here anymore
        if !valves_open.contains(&e_valve.name) && e_valve.flow != 0 {
            e_possible_actions.push("open".to_string());
        }
        for s in &my_valve.leads {
            if !my_old_valves.contains(s) {
                my_possible_actions.push(s.clone());
            }
        }
        for s in &e_valve.leads {
            if !e_old_valves.contains(s) {
                e_possible_actions.push(s.clone());
            }
        }

        for i in &my_possible_actions {
            for j in &e_possible_actions {
                total_possible_actions.push((i.to_string(), j.to_string()));
            }
        }
    }

    let mut combinations: Vec<u32> = Vec::new();
    for t in total_possible_actions {

        let i = t.0;
        let j = t.1;
        let mut flow_per_time = flow_per_time;
        let mut valves_open = copy_valve(valves_open);
        let mut my_new_valve = my_valve;
        let mut e_new_valve = e_valve;
        let mut my_new_valves = my_old_valves.clone();
        let mut e_new_valves = e_old_valves.clone();

        if i.eq("open") {
            flow_per_time += my_new_valve.flow;
            valves_open = open_valve(&valves_open, my_new_valve.name.clone());
            my_new_valves = HashSet::new();
        } else {
            my_new_valve = valves.iter().find(|v| v.name == i).unwrap();
            my_new_valves.insert(i);
        }

        if j.eq("open") {
            flow_per_time += e_new_valve.flow;
            valves_open = open_valve(&valves_open, e_new_valve.name.clone());
            e_new_valves = HashSet::new();
        } else {
            e_new_valve = valves.iter().find(|v| v.name == j).unwrap();
            e_new_valves.insert(j);
        }

        let memo = generate_memo2(valves, my_new_valve, e_new_valve, &valves_open ,time-1);
        if memos.contains_key(&memo) {
            //combinations.push(new_total_flow + *memos.get(&memo).unwrap());
            combinations.push(new_total_flow + *memos.get(&memo).unwrap());
        } else {
            combinations.push(memoize_combinations2_no_loops_hash(memos, valves, my_new_valve, e_new_valve,  &valves_open, flow_per_time, new_total_flow, time-1, &my_new_valves, &e_new_valves));
        }
    }
    let mut max = 0;
    if combinations.len() == 0 {
        // this happens due to the anti-back and forth filter
        // when this happens, it should be safe to assume that the result is optimised or severely unoptimal and there's no point continuing any further in this branch
        memos.insert(generate_memo2(valves, my_valve, e_valve, valves_open ,time), time * flow_per_time);
        return total_flow + time * flow_per_time;
    }
    for c in combinations {
        if max < c {
            max = c;
        }
    }
    memos.insert(generate_memo2(valves, my_valve, e_valve, valves_open ,time), max - total_flow);
    //println!("{:?}", memos.values());
    return max;
}

fn memoize_combinations2_hash(memos: &mut HashMap<ValveMemo2, u32>, valves: &Vec<Valve>, my_valve: &Valve, e_valve: &Valve, valves_open: &HashSet<String>,  flow_per_time: u32, total_flow: u32, time: u32, my_old_valve: &Valve, e_old_valve: &Valve) -> u32{

    if time == 0 {
        return total_flow;
    }

    let new_total_flow = flow_per_time + total_flow;
    let mut my_possible_actions: Vec<String> = Vec::new();
    let mut e_possible_actions: Vec<String> = Vec::new();
    let mut total_possible_actions: Vec<(String, String)> = Vec::new();

    // prevent duplicate actions ie i go to B and e goes to C along with i go to C and e goes to B
    // this approach also handles making sure only one person opens the valve if both are on the same valve
    if my_valve.name == e_valve.name {
        let mut total_leads = Vec::new();
        if !valves_open.contains(&my_valve.name) && my_valve.flow != 0 {
            total_leads.push("open".to_string());
        }
        for v in &my_valve.leads {
            // prevent either me or elephant going to where either came from, as me going to the elephants old position is the same as
            if !(v.to_string() == my_old_valve.name || v.to_string() == e_old_valve.name) {
                total_leads.push(v.to_string());
            }
        }

        if total_leads.len() > 0 {
            for i in 0..total_leads.len()-1 {
                for j in i+1..total_leads.len() {
                    total_possible_actions.push((total_leads[i].to_string(), total_leads[j].to_string()));
                }
            }
        }
    } else {
        if !valves_open.contains(&my_valve.name) && my_valve.flow != 0 {
            my_possible_actions.push("open".to_string());
        }
        // note the duplicate valve scenario doesn't occur here anymore
        if !valves_open.contains(&e_valve.name) && e_valve.flow != 0 {
            e_possible_actions.push("open".to_string());
        }
        for s in &my_valve.leads {
            if s != &my_old_valve.name {
                my_possible_actions.push(s.clone());
            }
        }
        for s in &e_valve.leads {
            if s != &e_old_valve.name {
                e_possible_actions.push(s.clone());
            }
        }

        for i in &my_possible_actions {
            for j in &e_possible_actions {
                total_possible_actions.push((i.to_string(), j.to_string()));
            }
        }
    }

    let mut combinations: Vec<u32> = Vec::new();

    for t in total_possible_actions {

        let i = t.0;
        let j = t.1;
        let mut flow_per_time = flow_per_time;
        let mut valves_open = valves_open.clone();
        let mut my_new_valve = my_valve;
        let mut e_new_valve = e_valve;

        if i.eq("open") {
            flow_per_time += my_new_valve.flow;
            valves_open.insert(my_valve.name.to_string());
        } else {
            my_new_valve = valves.iter().find(|v| v.name == i).unwrap();
        }

        if j.eq("open") {
            flow_per_time += e_new_valve.flow;
            valves_open.insert(e_new_valve.name.to_string());
        } else {
            e_new_valve = valves.iter().find(|v| v.name == j).unwrap();
        }

        let memo = generate_memo2_hash(valves, my_new_valve, e_new_valve, &valves_open ,time-1);
        if memos.contains_key(&memo) {
            //combinations.push(new_total_flow + *memos.get(&memo).unwrap());
            combinations.push(new_total_flow + *memos.get(&memo).unwrap());
        } else {
            combinations.push(memoize_combinations2_hash(memos, valves, my_new_valve, e_new_valve,  &valves_open, flow_per_time, new_total_flow, time-1, my_valve, e_valve));
        }
    }
    let mut max = 0;
    if combinations.len() == 0 {
        // this happens due to the anti-back and forth filter
        // when this happens, it should be safe to assume that the result is optimised or severely unoptimal and there's no point continuing any further in this branch
        memos.insert(generate_memo2_hash(valves, my_valve, e_valve, valves_open ,time), time * flow_per_time);
        return total_flow + time * flow_per_time;
    }
    for c in combinations {
        if max < c {
            max = c;
        }
    }
    memos.insert(generate_memo2_hash(valves, my_valve, e_valve, valves_open ,time), max - total_flow);
    //println!("{:?}", memos.values());
    return max;
}

fn memoize_combinations2_super_hash(memos: &mut HashMap<ValveMemo2, u32>, valves: &HashSet<Valve2>, my_valve: &Valve2, e_valve: &Valve2, valves_open: &HashSet<Valve2>,  flow_per_time: u32, total_flow: u32, time: u32, my_old_valve: &Valve2, e_old_valve: &Valve2) -> u32{

    if time == 0 {
        return total_flow;
    }

    let new_total_flow = flow_per_time + total_flow;
    let mut my_possible_actions: Vec<String> = Vec::new();
    let mut e_possible_actions: Vec<String> = Vec::new();
    let mut total_possible_actions: Vec<(String, String)> = Vec::new();

    // prevent duplicate actions ie i go to B and e goes to C along with i go to C and e goes to B
    // this approach also handles making sure only one person opens the valve if both are on the same valve
    if my_valve.name == e_valve.name {
        let mut total_leads = Vec::new();
        if !valves_open.contains(&my_valve) && my_valve.flow != 0 {
            total_leads.push("open".to_string());
        }
        for v in &my_valve.leads {
            // prevent either me or elephant going to where either came from, as me going to the elephants old position is the same as
            if !(v.to_string() == my_old_valve.name || v.to_string() == e_old_valve.name) {
                total_leads.push(v.to_string());
            }
        }

        if total_leads.len() > 0 {
            for i in 0..total_leads.len()-1 {
                for j in i+1..total_leads.len() {
                    total_possible_actions.push((total_leads[i].to_string(), total_leads[j].to_string()));
                }
            }
        }
    } else {
        if !valves_open.contains(&my_valve) && my_valve.flow != 0 {
            my_possible_actions.push("open".to_string());
        }
        // note the duplicate valve scenario doesn't occur here anymore
        if !valves_open.contains(&e_valve) && e_valve.flow != 0 {
            e_possible_actions.push("open".to_string());
        }
        for s in &my_valve.leads {
            if s != &my_old_valve.name {
                my_possible_actions.push(s.clone());
            }
        }
        for s in &e_valve.leads {
            if s != &e_old_valve.name {
                e_possible_actions.push(s.clone());
            }
        }

        for i in &my_possible_actions {
            for j in &e_possible_actions {
                total_possible_actions.push((i.to_string(), j.to_string()));
            }
        }
    }

    let mut combinations: Vec<u32> = Vec::new();

    for t in total_possible_actions {

        let i = t.0;
        let j = t.1;
        let mut flow_per_time = flow_per_time;
        let mut valves_open = valves_open.clone();
        let mut my_new_valve = my_valve;
        let mut e_new_valve = e_valve;

        if i.eq("open") {
            flow_per_time += my_new_valve.flow;
            //new_valves_open.insert(Valve2::new("AA"));
            //new_valves_open.insert(Valve2{name: my_valve.name.to_string(), id: my_valve.id, leads: my_valve.leads.clone(), flow: my_valve.flow});
            valves_open.insert(Valve2{name: my_valve.name.to_string(), id: my_valve.id, leads: my_valve.leads.clone(), flow: my_valve.flow});
        } else {
            my_new_valve = valves.iter().find(|v| v.name == i).unwrap();
        }

        if j.eq("open") {
            flow_per_time += e_new_valve.flow;
            valves_open.insert(Valve2{name: e_valve.name.to_string(), id: e_valve.id, leads: e_valve.leads.clone(), flow: e_valve.flow});
        } else {
            e_new_valve = valves.iter().find(|v| v.name == j).unwrap();
        }

        let memo = generate_memo2_super_hash(valves, my_new_valve, e_new_valve, &valves_open ,time-1);
        if memos.contains_key(&memo) {
            //combinations.push(new_total_flow + *memos.get(&memo).unwrap());
            combinations.push(new_total_flow + *memos.get(&memo).unwrap());
        } else {
            combinations.push(memoize_combinations2_super_hash(memos, valves, my_new_valve, e_new_valve,  &valves_open, flow_per_time, new_total_flow, time-1, my_valve, e_valve));
        }
    }
    let mut max = 0;
    if combinations.len() == 0 {
        // this happens due to the anti-back and forth filter
        // when this happens, it should be safe to assume that the result is optimised or severely unoptimal and there's no point continuing any further in this branch
        memos.insert(generate_memo2_super_hash(valves, my_valve, e_valve, valves_open ,time), time * flow_per_time);
        return total_flow + time * flow_per_time;
    }
    for c in combinations {
        if max < c {
            max = c;
        }
    }
    memos.insert(generate_memo2_super_hash(valves, my_valve, e_valve, valves_open ,time), max - total_flow);
    //println!("{:?}", memos.values());
    return max;
}

fn memoize_combinations2_super_hash2(memos: &mut HashMap<ValveMemo2, u32>, valves: &HashSet<Valve2>, my_valve: &Valve2, e_valve: &Valve2, valves_open: &HashSet<Valve2>,  flow_per_time: u32, total_flow: u32, time: u32, my_old_valves: &HashSet<Valve2>, e_old_valves: &HashSet<Valve2>) -> u32{

    if time == 0 {
        return total_flow;
    }

    let new_total_flow = flow_per_time + total_flow;
    let mut my_possible_actions: Vec<String> = Vec::new();
    let mut e_possible_actions: Vec<String> = Vec::new();
    let mut total_possible_actions: Vec<(String, String)> = Vec::new();

    // prevent duplicate actions ie i go to B and e goes to C along with i go to C and e goes to B
    // this approach also handles making sure only one person opens the valve if both are on the same valve
    if my_valve.name == e_valve.name {
        let mut total_leads = Vec::new();
        if !valves_open.contains(&my_valve) && my_valve.flow != 0 {
            total_leads.push("open".to_string());
        }
        for v in &my_valve.leads {
            // prevent either me or elephant going to where either came from, as me going to the elephants old position is the same as
            if !(my_old_valves.contains(&Valve2::new(v)) || e_old_valves.contains(&Valve2::new(v))) {
                total_leads.push(v.to_string());
            }
        }

        if total_leads.len() > 0 {
            for i in 0..total_leads.len()-1 {
                for j in i+1..total_leads.len() {
                    total_possible_actions.push((total_leads[i].to_string(), total_leads[j].to_string()));
                }
            }
        }
    } else {
        if !valves_open.contains(&my_valve) && my_valve.flow != 0 {
            my_possible_actions.push("open".to_string());
        }
        // note the duplicate valve scenario doesn't occur here anymore
        if !valves_open.contains(&e_valve) && e_valve.flow != 0 {
            e_possible_actions.push("open".to_string());
        }
        for s in &my_valve.leads {
            if !my_old_valves.contains(&Valve2::new(s)) {
                my_possible_actions.push(s.clone());
            }
        }
        for s in &e_valve.leads {
            if !e_old_valves.contains(&Valve2::new(s)) {
                e_possible_actions.push(s.clone());
            }
        }

        for i in &my_possible_actions {
            for j in &e_possible_actions {
                total_possible_actions.push((i.to_string(), j.to_string()));
            }
        }
    }

    let mut combinations: Vec<u32> = Vec::new();

    for t in total_possible_actions {

        let i = t.0;
        let j = t.1;
        let mut flow_per_time = flow_per_time;
        let mut valves_open = valves_open.clone();
        let mut my_new_valve = my_valve;
        let mut e_new_valve = e_valve;
        let mut my_new_old_valves = my_old_valves.clone();
        let mut e_new_old_valves = e_old_valves.clone();

        if i.eq("open") {
            flow_per_time += my_new_valve.flow;
            //new_valves_open.insert(Valve2::new("AA"));
            //new_valves_open.insert(Valve2{name: my_valve.name.to_string(), id: my_valve.id, leads: my_valve.leads.clone(), flow: my_valve.flow});
            valves_open.insert(Valve2{name: my_valve.name.to_string(), id: my_valve.id, leads: my_valve.leads.clone(), flow: my_valve.flow});
            my_new_old_valves = HashSet::new();
        } else {
            my_new_valve = valves.get(&Valve2::new(i.as_str())).unwrap();
            my_new_old_valves.insert(Valve2::new(i.as_str()));
        }

        if j.eq("open") {
            flow_per_time += e_new_valve.flow;
            valves_open.insert(Valve2{name: e_valve.name.to_string(), id: e_valve.id, leads: e_valve.leads.clone(), flow: e_valve.flow});
            e_new_old_valves = HashSet::new();
        } else {
            e_new_valve = valves.get(&Valve2::new(j.as_str())).unwrap();
            e_new_old_valves.insert(Valve2::new(j.as_str()));
        }

        let memo = generate_memo2_super_hash(valves, my_new_valve, e_new_valve, &valves_open ,time-1);
        if memos.contains_key(&memo) {
            //combinations.push(new_total_flow + *memos.get(&memo).unwrap());
            combinations.push(new_total_flow + *memos.get(&memo).unwrap());
        } else {
            combinations.push(memoize_combinations2_super_hash2(memos, valves, my_new_valve, e_new_valve,  &valves_open, flow_per_time, new_total_flow, time-1, &my_new_old_valves, &e_new_old_valves));
        }
    }
    let mut max = 0;
    if combinations.len() == 0 {
        // this happens due to the anti-back and forth filter
        // when this happens, it should be safe to assume that the result is optimised or severely unoptimal and there's no point continuing any further in this branch
        memos.insert(generate_memo2_super_hash(valves, my_valve, e_valve, valves_open ,time), time * flow_per_time);
        return total_flow + time * flow_per_time;
    }
    for c in combinations {
        if max < c {
            max = c;
        }
    }
    memos.insert(generate_memo2_super_hash(valves, my_valve, e_valve, valves_open ,time), max - total_flow);
    //println!("{:?}", memos.values());
    return max;
}

fn memoize_combinations_zap(memos: &mut HashMap<ValveMemo, u32>, valves: &Vec<Valve>, current_valve: &Valve, valves_open: &Vec<String>,  flow_per_time: u32, total_flow: u32, time: u32, time_initial: u32, reset: bool) -> u32{

    if time == 0 {
        if reset {
            return total_flow;
        } else {
            let valve_aa = valves.iter().find(|v| v.name == "AA").unwrap();
            return memoize_combinations_zap(memos, valves, &valve_aa, valves_open, 0, total_flow, time_initial, time_initial, true);
        }
    }

    let new_total_flow = flow_per_time + total_flow;
    let mut combinations: Vec<u32> = Vec::new();

    // try opening current valve
    if !valves_open.contains(&current_valve.name) && current_valve.flow != 0 {
        let valves_open = open_valve(&valves_open, current_valve.name.clone());
        let memo = generate_memo(valves, current_valve, &valves_open ,time-1);
        if memos.contains_key(&memo) {
            //println!("FOUND MEMO");
            combinations.push(new_total_flow + *memos.get(&memo).unwrap());
        } else {
            let flow_per_time = flow_per_time + current_valve.flow;
            combinations.push(memoize_combinations_zap(memos, valves, current_valve, &valves_open, flow_per_time, new_total_flow, time-1, time_initial, reset));
        }
            
    }
    for s in &current_valve.leads {
        let new_valve = valves.iter().find(|v| &v.name == s).unwrap();
        let memo = generate_memo(valves, &new_valve, valves_open ,time-1);
        if memos.contains_key(&memo) {
            //println!("FOUND MEMO");
            combinations.push(new_total_flow + *memos.get(&memo).unwrap());
        } else {
            combinations.push(memoize_combinations_zap(memos, valves, new_valve, valves_open, flow_per_time, new_total_flow, time-1, time_initial, reset));
        }
    }
    // if !reset {
    //     let valve_aa = valves.iter().find(|v| v.name == "AA").unwrap();
    //     combinations.push(time*flow_per_time + memoize_combinations_zap(memos, valves, &valve_aa, valves_open, 0, new_total_flow, time_initial, time_initial, true));
    // }

    let mut max = 0;
    if combinations.len() == 0 {
        return 0;
    }
    for c in combinations {
        if max < c {
            max = c;
        }
    }
    memos.insert(generate_memo(valves, current_valve, valves_open ,time), max - total_flow);
    //println!("{:?}", memos.values());
    return max;
}

fn generate_memo(valves: &Vec<Valve>, current_valve: &Valve, valves_open: &Vec<String>, time: u32) -> ValveMemo {
    //let mut open = Vec::new();
    let mut open: u64 = 0;
    for (i,v) in valves.iter().enumerate() {
        if valves_open.contains(&v.name) {
            open += u64::pow(2, i.try_into().unwrap());
        }
    }
    ValveMemo{
        valve: current_valve.name.clone(),
        open,
        time_remaining: time,
    }
}

fn generate_memo2(valves: &Vec<Valve>, my_valve: &Valve, e_valve: &Valve, valves_open: &Vec<String>, time: u32) -> ValveMemo2 {
    //let mut open = Vec::new();
    let mut open: u64 = 0;
    let mut v1: u32 = 0;
    let mut v2: u32 = 0;
    for (i,v) in valves.iter().enumerate() {
        if v.name.eq(&my_valve.name) {
            v1 = i.try_into().unwrap();
        }
        if v.name.eq(&e_valve.name) {
            v2 = i.try_into().unwrap();
        }
        if valves_open.contains(&v.name) {
            open += u64::pow(2, i.try_into().unwrap());
        }
    }

    let valve = if v1 <= v2 {
        (v1,v2)
    } else {
        (v2,v1)
    };
    ValveMemo2{
        valve,
        open,
        time_remaining: time,
    }
}

fn generate_memo2_hash(valves: &Vec<Valve>, my_valve: &Valve, e_valve: &Valve, valves_open: &HashSet<String>, time: u32) -> ValveMemo2 {
    //let mut open = Vec::new();
    let mut open: u64 = 0;
    let mut v1: u32 = 0;
    let mut v2: u32 = 0;
    for (i,v) in valves.iter().enumerate() {
        if v.name.eq(&my_valve.name) {
            v1 = i.try_into().unwrap();
        }
        if v.name.eq(&e_valve.name) {
            v2 = i.try_into().unwrap();
        }
        if valves_open.contains(&v.name) {
            open += u64::pow(2, i.try_into().unwrap());
        }
    }

    let valve = if v1 <= v2 {
        (v1,v2)
    } else {
        (v2,v1)
    };
    ValveMemo2{
        valve,
        open,
        time_remaining: time,
    }
}

fn generate_memo2_super_hash(valves: &HashSet<Valve2>, my_valve: &Valve2, e_valve: &Valve2, valves_open: &HashSet<Valve2>, time: u32) -> ValveMemo2 {
    //let mut open = Vec::new();
    let mut open: u64 = 0;
    let v1: u32 = valves.get(my_valve).unwrap().id;
    let v2: u32 = valves.get(e_valve).unwrap().id;
    for v in valves_open {
        open += u64::pow(2, v.id.try_into().unwrap());
    }
    // for (i,v) in valves.iter().enumerate() {
    //     if v.name.eq(&my_valve.name) {
    //         v1 = i.try_into().unwrap();
    //     }
    //     if v.name.eq(&e_valve.name) {
    //         v2 = i.try_into().unwrap();
    //     }
    //     if valves_open.contains(&v.name) {
    //         open += u64::pow(2, i.try_into().unwrap());
    //     }
    // }

    let valve = if v1 <= v2 {
        (v1,v2)
    } else {
        (v2,v1)
    };
    ValveMemo2{
        valve,
        open,
        time_remaining: time,
    }
}

fn open_valve(current: &Vec<String>, new: String) -> Vec<String>{
    // let mut new_vec: Vec<String> = Vec::new();
    // for i in current {
    //     new_vec.push(i.clone());
    // }
    let mut new_vec = current.to_vec();
    new_vec.push(new);
    new_vec
}

fn copy_valve(current: &Vec<String>) -> Vec<String>{
    // let mut new_vec: Vec<String> = Vec::new();
    // for i in current {
    //     new_vec.push(i.clone());
    // }
    // new_vec
    current.to_vec()
}

fn parse_line(line: String) -> Valve {
    let strings = utils::regex_to_vec(line, &r"^Valve ([A-Z]+) has flow rate=([0-9]+); tunnels? leads? to valves? (.*)$".to_string());
    // one match per line, 3 groups per match
    let name = strings[0][1].clone();
    let flow: u32 =  strings[0][2].parse().unwrap();
    let leads: Vec<String> = strings[0][3].split(", ").map(|s| s.to_string()).collect();
    Valve{name, flow, leads}
}

fn parse_line2(line: String, index: u32) -> Valve2 {
    let strings = utils::regex_to_vec(line, &r"^Valve ([A-Z]+) has flow rate=([0-9]+); tunnels? leads? to valves? (.*)$".to_string());
    // one match per line, 3 groups per match
    let name = strings[0][1].clone();
    let flow: u32 =  strings[0][2].parse().unwrap();
    let leads: HashSet<String> = strings[0][3].split(", ").map(|s| s.to_string()).collect();
    Valve2{name, flow, leads, id: index}
}

fn part2(day : &str) {
    
    let mut valves : Vec<Valve> = Vec::new();
    let mut max_flow = 0;
    
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        
        valves.push(parse_line(line));

    }

    for v in &valves {
        max_flow += v.flow;
    }
    println!("{max_flow}");

    let time = 26;

    let loop1 = true;

    //let max = test_every_combination(&valves, &valves[0], &Vec::new(), 0, 0, time, visited);
    //println!("Day 16 part 1: {i} {max}");
    let valve_aa = valves.iter().find(|v| v.name == "AA".to_string()).unwrap();

     if !loop1 {
    //     // let mut memos : HashMap<ValveMemo2, u32> = HashMap::new();
    //     // let max = memoize_combinations2(&mut memos, &valves, &valve_aa, &valve_aa, &Vec::new(), 0, 0, time, &valve_aa, &valve_aa);
         let mut memos : HashMap<ValveMemo2, u32> = HashMap::new();
         let max = memoize_combinations2_no_loops(&mut memos, &valves, &valve_aa, &valve_aa, &Vec::new(), 0, 0, time, &Vec::new(), &Vec::new());
        
    //     // let mut memos2 : HashMap<ValveMemo2, (u32, String, String)> = HashMap::new();
    //     // let max = memoize_combinations2_with_path(&mut memos2, &valves, &valve_aa, &valve_aa, &Vec::new(), 0, 0, time);
        
         println!("Day 16 part 2: {:?}", max);
     } else {
        for i in 1..30 {
            // let mut memos : HashMap<ValveMemo2, u32> = HashMap::new();
            // let max = memoize_combinations2(&mut memos, &valves, &valve_aa, &valve_aa, &Vec::new(), 0, 0, i, &valve_aa, &valve_aa);
            // let mut memos : HashMap<ValveMemo2, u32> = HashMap::new();
            // let max = memoize_combinations2_no_loops(&mut memos, &valves, &valve_aa, &valve_aa, &Vec::new(), 0, 0, i, &Vec::new(), &Vec::new());
            // let mut memos : HashMap<ValveMemo2, u32> = HashMap::new();
            // let max = memoize_combinations2_no_loops_hash(&mut memos, &valves, &valve_aa, &valve_aa, &Vec::new(), 0, 0, i, &HashSet::new(), &HashSet::new());
            // let mut memos : HashMap<ValveMemo2, u32> = HashMap::new();
            // let max = memoize_combinations2_hash(&mut memos, &valves, &valve_aa, &valve_aa, &HashSet::new(), 0, 0, i, &valve_aa, &valve_aa);

            // let mut memos2 : HashMap<ValveMemo2, (u32, String, String)> = HashMap::new();
            // let max = memoize_combinations2_with_path(&mut memos2, &valves, &valve_aa, &valve_aa, &Vec::new(), 0, 0, i);

            let mut memos : HashMap<ValveMemo, u32> = HashMap::new();
            let max = memoize_combinations_zap(&mut memos, &valves, &valve_aa, &Vec::new(), 0, 0, i, i, false);
            println!("{i}: {:?}", max);
        }
    }
}

fn part2_hash(day : &str) {

    println!("using hash");
    
    let mut valves : HashSet<Valve2> = HashSet::new();
    let mut index = 0;
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        
        valves.insert(parse_line2(line, index));
        index += 1;
    }

    let time = 26;

    let loop1 = true;

    //let max = test_every_combination(&valves, &valves[0], &Vec::new(), 0, 0, time, visited);
    //println!("Day 16 part 1: {i} {max}");
    //let valve_aa = valves.iter().find(|v| v.name == "AA".to_string()).unwrap();
    let valve_aa = valves.get(&Valve2::new("AA")).unwrap();

     if !loop1 {
         let mut memos : HashMap<ValveMemo2, u32> = HashMap::new();
         let max = memoize_combinations2_super_hash(&mut memos, &valves, &valve_aa, &valve_aa, &HashSet::new(), 0, 0, time, &valve_aa, &valve_aa);
        
         println!("Day 16 part 2: {:?}", max);
     } else {
        for i in 1..30 {
            let mut memos : HashMap<ValveMemo2, u32> = HashMap::new();
            //let max = memoize_combinations2_super_hash(&mut memos, &valves, &valve_aa, &valve_aa, &HashSet::new(), 0, 0, i, &valve_aa, &valve_aa);
            let max = memoize_combinations2_super_hash2(&mut memos, &valves, &valve_aa, &valve_aa, &HashSet::new(), 0, 0, i, &HashSet::new(), &HashSet::new());

            println!("{i}: {:?}", max);
        }
    }
}

fn test() {

}