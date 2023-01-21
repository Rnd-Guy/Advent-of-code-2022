use std::collections::HashMap;
use std::ops::{Add, Sub, Mul};

use super::utils;
use super::utils::Print::*;

// start 10:10
pub fn run(print : utils::Print ) {
    let day : &str = "19";

    assert!(day != "0", "CHANGE THE DAY");

    test(); // for any assertion tests

    match print {
        Part1 => part1(day),
        Part2 => part2(day),
        BothParts => { part1(day); part2(day); }
        NoParts => (),
    }
}

#[derive(Debug,Clone, Copy, PartialEq, Eq, Hash)]
struct Resource {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}

impl Resource {
    fn new(ore: i32, clay: i32, obsidian: i32, geode: i32) -> Self {
        Self{ore, clay, obsidian, geode}
    }

    fn is_not_negative(&self) -> bool {
        return self.ore >= 0 && self.clay >= 0 && self.obsidian >= 0 && self.geode >= 0;
    }
}

impl Add for Resource {
    type Output = Resource;

    fn add(self, rhs: Self) -> Self::Output {
        Resource{ore: self.ore + rhs.ore, clay: self.clay + rhs.clay, obsidian: self.obsidian + rhs.obsidian, geode: self.geode + rhs.geode}
    }
}

impl Sub for Resource {
    type Output = Resource;

    fn sub(self, rhs: Self) -> Self::Output {
        Resource{ore: self.ore - rhs.ore, clay: self.clay - rhs.clay, obsidian: self.obsidian - rhs.obsidian, geode: self.geode - rhs.geode}
    }
}

impl Mul<i32> for Resource {
    type Output = Resource;

    fn mul(self, rhs: i32) -> Self::Output {
        Resource{ore: self.ore * rhs, clay: self.clay * rhs, obsidian: self.obsidian * rhs, geode: self.geode * rhs}
    }
}



struct Blueprint {
    id: i32,
    ore_robot: Resource,
    clay_robot: Resource,
    obsidian_robot: Resource,
    geode_robot: Resource,
}

fn part1(day: &str) {

    let mut blueprints : Vec<Blueprint> = Vec::new();
    
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        
        let parsed_line = utils::regex_to_vec(line, &r"Blueprint ([0-9]+):.*([0-9]).*([0-9]).*([0-9]) ore and ([0-9]+) clay.*([0-9]) ore and ([0-9]+) obsidian".to_string());
        // groups: 1 = blueprint no
        //         2 = ore robot ore cost
        //         3 = clay robot ore cost
        //         4 = obs robot ore cost
        //         5 = obs robot clay cost
        //         6 = geode robot ore cost
        //         7 = geode robot obsidian cost
        let id = parsed_line[0][1].parse().unwrap();
        let ore_robot = Resource::new(parsed_line[0][2].parse().unwrap(), 0, 0, 0);
        let clay_robot = Resource::new(parsed_line[0][3].parse().unwrap(), 0, 0, 0);
        let obsidian_robot = Resource::new(parsed_line[0][4].parse().unwrap(), parsed_line[0][5].parse().unwrap(), 0, 0);
        let geode_robot = Resource::new(parsed_line[0][6].parse().unwrap(), 0, parsed_line[0][7].parse().unwrap(), 0);
        let blueprint = Blueprint{id, ore_robot, clay_robot, obsidian_robot, geode_robot};
        blueprints.push(blueprint);
    }

    let mut geodes : Vec<i32> = Vec::new();
    let mut sum = 0;
    for b in &blueprints {
        // uncomment this to show progress
        // println!("done 1");
        let geode = calculate_max_geodes_pat_strat(&mut HashMap::new(), b, 1, Resource::new(0,0,0,0), Resource::new(1,0,0,0), 24);
        geodes.push(geode);
        sum += geode * b.id;
    }

    // println!("{:?}", geodes);
    println!("Day 19 part 1: {sum}");


}

fn calculate_max_geodes(memos: &mut HashMap<(i32, Resource, Resource), i32>, blueprint: &Blueprint, time_elapsed: i32, current_resources: Resource, current_robots: Resource, max_time: i32) -> i32 {

    // actions: wait, buy robot
    let new_current_resources = current_resources + current_robots;
    if time_elapsed == max_time {
        return new_current_resources.geode;
    }
    
    let mut combinations : Vec<i32> = Vec::new();

    // optimisation checks:
    // - never need more resources than the max needed to build all robots
    // - if you can build a geode, build a geode


    // geode robot
    if (current_resources - blueprint.geode_robot).is_not_negative() {
        let resources_after_purchase = new_current_resources - blueprint.geode_robot;
        let robots_after_purchase = current_robots + Resource::new(0,0,0,1);
        if memos.contains_key(&(time_elapsed+1, resources_after_purchase, robots_after_purchase)) {
            combinations.push(*memos.get(&(time_elapsed+1, resources_after_purchase, robots_after_purchase)).unwrap());
        } else {
            combinations.push(calculate_max_geodes(memos, blueprint, time_elapsed+1, 
                resources_after_purchase, robots_after_purchase, max_time));
        }
    } else {

    // ore robot
    if (current_resources - blueprint.ore_robot).is_not_negative() && 
        current_robots.ore < [blueprint.ore_robot.ore, blueprint.clay_robot.ore, blueprint.obsidian_robot.ore, blueprint.geode_robot.ore].iter().fold(0, |a,&b| std::cmp::max(a,b)) {
        let resources_after_purchase = new_current_resources - blueprint.ore_robot;
        let robots_after_purchase = current_robots + Resource::new(1,0,0,0);
        if memos.contains_key(&(time_elapsed+1, resources_after_purchase, robots_after_purchase)) {
            combinations.push(*memos.get(&(time_elapsed+1, resources_after_purchase, robots_after_purchase)).unwrap());
        } else {
            combinations.push(calculate_max_geodes(memos, blueprint, time_elapsed+1, 
                resources_after_purchase, robots_after_purchase, max_time));
        }
    }

    // clay robot
    if (current_resources - blueprint.clay_robot).is_not_negative() && current_robots.clay < blueprint.obsidian_robot.clay {
        let resources_after_purchase = new_current_resources - blueprint.clay_robot;
        let robots_after_purchase = current_robots + Resource::new(0,1,0,0);
        if memos.contains_key(&(time_elapsed+1, resources_after_purchase, robots_after_purchase)) {
            combinations.push(*memos.get(&(time_elapsed+1, resources_after_purchase, robots_after_purchase)).unwrap());
        } else {
            combinations.push(calculate_max_geodes(memos, blueprint, time_elapsed+1, 
                resources_after_purchase, robots_after_purchase, max_time));
        }
    }

    // obsidian robot
    if (current_resources - blueprint.obsidian_robot).is_not_negative() && current_robots.obsidian < blueprint.geode_robot.obsidian {
        let resources_after_purchase = new_current_resources - blueprint.obsidian_robot;
        let robots_after_purchase = current_robots + Resource::new(0,0,1,0);
        if memos.contains_key(&(time_elapsed+1, resources_after_purchase, robots_after_purchase)) {
            combinations.push(*memos.get(&(time_elapsed+1, resources_after_purchase, robots_after_purchase)).unwrap());
        } else {
            combinations.push(calculate_max_geodes(memos, blueprint, time_elapsed+1, 
                resources_after_purchase, robots_after_purchase, max_time));
        }
    }

    // wait, but remember that it is unobtimal to wait if we can already afford all the robots we can possibly afford
    // if combinations.len() != 4 ||
    //    combinations.len() == 3 && current_robots.obsidian == 0 ||
    //    combinations.len() == 2 && current_robots.clay == 0
    if combinations.len() <= 2
    {
        if memos.contains_key(&(time_elapsed+1, new_current_resources, current_robots)) {
            combinations.push(*memos.get(&(time_elapsed+1, new_current_resources, current_robots)).unwrap());
        } else {
            combinations.push(calculate_max_geodes(memos, blueprint, time_elapsed+1, new_current_resources, current_robots, max_time));
        }
    }

    }

    let mut max = 0;
    for c in combinations {
        if c > max {
            max = c;
        }
    }

    memos.insert((time_elapsed, current_resources, current_robots), max);
    
    max
}

fn calculate_max_geodes_pat_strat(memos: &mut HashMap<(Resource, Resource), (i32, i32)>, blueprint: &Blueprint, time_elapsed: i32, current_resources: Resource, current_robots: Resource, max_time: i32) -> i32 {

    // actions: wait, buy robot
    let new_current_resources = current_resources + current_robots;
    if time_elapsed == max_time {
        return new_current_resources.geode;
    }
    
    let mut combinations : Vec<i32> = Vec::new();

    // optimisation checks:
    // - never need more resources than the max needed to build all robots
    // - if you can build a geode, build a geode


    // geode robot
    if (current_resources - blueprint.geode_robot).is_not_negative() {
        let resources_after_purchase = new_current_resources - blueprint.geode_robot;
        let robots_after_purchase = current_robots + Resource::new(0,0,0,1);
        if memos.contains_key(&(resources_after_purchase, robots_after_purchase)) {
            let memo = memos.get(&(resources_after_purchase, robots_after_purchase)).unwrap();
            if memo.0 < time_elapsed + 1 {
                // there was a faster way to get to this point
                combinations.push(0);
            } else if memo.0 == time_elapsed + 1{
                combinations.push(memos.get(&(resources_after_purchase, robots_after_purchase)).unwrap().1);
            } else {
                combinations.push(calculate_max_geodes_pat_strat(memos, blueprint, time_elapsed+1, 
                    resources_after_purchase, robots_after_purchase, max_time));
            }
        } else {
            combinations.push(calculate_max_geodes_pat_strat(memos, blueprint, time_elapsed+1, 
                resources_after_purchase, robots_after_purchase, max_time));
        }
    } else {

    // ore robot
    if (current_resources - blueprint.ore_robot).is_not_negative() && 
        current_robots.ore < [blueprint.ore_robot.ore, blueprint.clay_robot.ore, blueprint.obsidian_robot.ore, blueprint.geode_robot.ore].iter().fold(0, |a,&b| std::cmp::max(a,b)) {
        let resources_after_purchase = new_current_resources - blueprint.ore_robot;
        let robots_after_purchase = current_robots + Resource::new(1,0,0,0);
        if memos.contains_key(&(resources_after_purchase, robots_after_purchase)) {
            let memo = memos.get(&(resources_after_purchase, robots_after_purchase)).unwrap();
            if memo.0 < time_elapsed + 1 {
                // there was a faster way to get to this point
                combinations.push(0);
            } else if memo.0 == time_elapsed + 1{
                combinations.push(memos.get(&(resources_after_purchase, robots_after_purchase)).unwrap().1);
            } else {
                combinations.push(calculate_max_geodes_pat_strat(memos, blueprint, time_elapsed+1, 
                    resources_after_purchase, robots_after_purchase, max_time));
            }
        } else {
            combinations.push(calculate_max_geodes_pat_strat(memos, blueprint, time_elapsed+1, 
                resources_after_purchase, robots_after_purchase, max_time));
        }
    }

    // clay robot
    if (current_resources - blueprint.clay_robot).is_not_negative() && current_robots.clay < blueprint.obsidian_robot.clay {
        let resources_after_purchase = new_current_resources - blueprint.clay_robot;
        let robots_after_purchase = current_robots + Resource::new(0,1,0,0);
        if memos.contains_key(&(resources_after_purchase, robots_after_purchase)) {
            let memo = memos.get(&(resources_after_purchase, robots_after_purchase)).unwrap();
            if memo.0 < time_elapsed + 1 {
                // there was a faster way to get to this point
                combinations.push(0);
            } else if memo.0 == time_elapsed + 1{
                combinations.push(memos.get(&(resources_after_purchase, robots_after_purchase)).unwrap().1);
            } else {
                combinations.push(calculate_max_geodes_pat_strat(memos, blueprint, time_elapsed+1, 
                    resources_after_purchase, robots_after_purchase, max_time));
            }
        } else {
            combinations.push(calculate_max_geodes_pat_strat(memos, blueprint, time_elapsed+1, 
                resources_after_purchase, robots_after_purchase, max_time));
        }
    }

    // obsidian robot
    if (current_resources - blueprint.obsidian_robot).is_not_negative() && current_robots.obsidian < blueprint.geode_robot.obsidian {
        let resources_after_purchase = new_current_resources - blueprint.obsidian_robot;
        let robots_after_purchase = current_robots + Resource::new(0,0,1,0);
        if memos.contains_key(&(resources_after_purchase, robots_after_purchase)) {
            let memo = memos.get(&(resources_after_purchase, robots_after_purchase)).unwrap();
            if memo.0 < time_elapsed + 1 {
                // there was a faster way to get to this point
                combinations.push(0);
            } else if memo.0 == time_elapsed + 1{
                combinations.push(memos.get(&(resources_after_purchase, robots_after_purchase)).unwrap().1);
            } else {
                combinations.push(calculate_max_geodes_pat_strat(memos, blueprint, time_elapsed+1, 
                    resources_after_purchase, robots_after_purchase, max_time));
            }
        } else {
            combinations.push(calculate_max_geodes_pat_strat(memos, blueprint, time_elapsed+1, 
                resources_after_purchase, robots_after_purchase, max_time));
        }
    }

    // wait, but remember that it is unobtimal to wait if we can already afford all the robots we can possibly afford
    // if combinations.len() != 4 ||
    //    combinations.len() == 3 && current_robots.obsidian == 0 ||
    //    combinations.len() == 2 && current_robots.clay == 0
    if combinations.len() <= 2
    {
        if memos.contains_key(&(new_current_resources, current_robots)) {
            let memo = memos.get(&(new_current_resources, current_robots)).unwrap();
            if memo.0 < time_elapsed + 1 {
                // there was a faster way to get to this point
                combinations.push(0);
            } else if memo.0 == time_elapsed + 1{
                combinations.push(memos.get(&(new_current_resources, current_robots)).unwrap().1);
            } else {
                combinations.push(calculate_max_geodes_pat_strat(memos, blueprint, time_elapsed+1, 
                    new_current_resources, current_robots, max_time));
            }
        } else {
            combinations.push(calculate_max_geodes_pat_strat(memos, blueprint, time_elapsed+1, 
                new_current_resources, current_robots, max_time));
        }
    }

    }

    let mut max = 0;
    for c in combinations {
        if c > max {
            max = c;
        }
    }

    memos.insert((current_resources, current_robots), (time_elapsed, max));
    
    max
}

// only take first 3, but use 32 time instead of 24
fn part2(day : &str) {
    
    let mut blueprints : Vec<Blueprint> = Vec::new();
    
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        
        let parsed_line = utils::regex_to_vec(line, &r"Blueprint ([0-9]+):.*([0-9]).*([0-9]).*([0-9]) ore and ([0-9]+) clay.*([0-9]) ore and ([0-9]+) obsidian".to_string());
        // groups: 1 = blueprint no
        //         2 = ore robot ore cost
        //         3 = clay robot ore cost
        //         4 = obs robot ore cost
        //         5 = obs robot clay cost
        //         6 = geode robot ore cost
        //         7 = geode robot obsidian cost
        let id = parsed_line[0][1].parse().unwrap();
        let ore_robot = Resource::new(parsed_line[0][2].parse().unwrap(), 0, 0, 0);
        let clay_robot = Resource::new(parsed_line[0][3].parse().unwrap(), 0, 0, 0);
        let obsidian_robot = Resource::new(parsed_line[0][4].parse().unwrap(), parsed_line[0][5].parse().unwrap(), 0, 0);
        let geode_robot = Resource::new(parsed_line[0][6].parse().unwrap(), 0, parsed_line[0][7].parse().unwrap(), 0);
        let blueprint = Blueprint{id, ore_robot, clay_robot, obsidian_robot, geode_robot};
        blueprints.push(blueprint);

        if blueprints.len() == 3 {
            break;
        }
    }

    let mut geodes : Vec<i32> = Vec::new();
    let mut mult = 1;
    for b in &blueprints {
        // uncomment this to show progress
        println!("done 1");
        let geode = calculate_max_geodes_pat_strat(&mut HashMap::new(), b, 1, Resource::new(0,0,0,0), Resource::new(1,0,0,0), 32);
        geodes.push(geode);
        mult *= geode;
    }

    println!("{:?}", geodes);
    println!("Day 19 part 2: {mult}");
}

fn test() {

}