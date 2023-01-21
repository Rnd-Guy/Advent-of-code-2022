use super::utils;
use super::utils::Print::*;

// start 13:46
// part1 clear: 14:24
pub fn run(print : utils::Print ) {
    let day : &str = "24";

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
    let (mut blizzards, width, height) = get_blizzards(day);
    let mut steps = 0;
    let mut positions = vec![(1,0)];
    while !positions.contains(&(width-2,height-1)) {
        move_blizzards(&mut blizzards, width, height);
        positions = get_new_potential_positions(&blizzards, &positions, width, height);
        steps+=1;
    }

    println!("Day 24 part 1: {steps}");

}

struct Blizzard {
    x: usize,
    y: usize,
    dir: Dir
}

enum Dir {
    Up,
    Down,
    Left,
    Right
}

fn get_new_potential_positions(blizzards: &Vec<Blizzard>, potential_positions: &Vec<(usize,usize)>, width: usize, height: usize) -> Vec<(usize,usize)> {
    let mut new_potential_positions: Vec<(usize,usize)> = Vec::new();
    for p in potential_positions {
        // stationary
        if !new_potential_positions.contains(p) && is_safe_location(blizzards, p, width, height){
            new_potential_positions.push(p.clone());
        }
        // up
        if p.1 != 0 {
            let q = (p.0, p.1-1);
            if !new_potential_positions.contains(&q) && is_safe_location(blizzards, &q, width, height){
                new_potential_positions.push(q.clone());
            }
        }
        // right
        let q = (p.0+1, p.1);
        if !new_potential_positions.contains(&q) && is_safe_location(blizzards, &q, width, height){
            new_potential_positions.push(q.clone());
        }
        // down
        if p.1 != height-1 {
            let q = (p.0, p.1+1);
            if !new_potential_positions.contains(&q) && is_safe_location(blizzards, &q, width, height){
                new_potential_positions.push(q.clone());
            }
        }
        // left
        let q = (p.0-1, p.1);
        if !new_potential_positions.contains(&q) && is_safe_location(blizzards, &q, width, height){
            new_potential_positions.push(q.clone());
        }
    }
    return new_potential_positions;
}

fn is_safe_location(blizzards: &Vec<Blizzard>, p: &(usize,usize), width: usize, height: usize) -> bool {
    if p == &(1,0) || p == &(width-2,height-1) {
        return true;
    } else if p.0 == 0 || p.0 == width-1 || p.1 == 0 || p.1 == height-1 {
        return false;
    } else {
        return !blizzards.iter().any(|b| b.x == p.0 && b.y == p.1);
    }
}

fn move_blizzards(blizzards: &mut Vec<Blizzard>, width: usize, height: usize) {
    for b in blizzards {
        match b.dir {
            Dir::Down => b.y += 1,
            Dir::Left => b.x -= 1,
            Dir::Right => b.x += 1,
            Dir::Up => b.y -= 1,
        }
        if b.x == 0 {
            b.x = width-2;
        }
        if b.x == width-1 {
            b.x = 1;
        }
        if b.y == 0 {
            b.y = height-2;
        }
        if b.y == height-1 {
            b.y = 1;
        }
    }
    
}

fn add_blizzards(blizzards: &mut Vec<Blizzard>, line: String, y: usize ) {
    let chars: Vec<char> = line.chars().collect();
    for i in 0..chars.len() {
        match chars[i] {
            '#'|'.' => continue,
            '<' => blizzards.push(Blizzard { x: i, y, dir: Dir::Left }),
            '>' => blizzards.push(Blizzard { x: i, y, dir: Dir::Right }),
            '^' => blizzards.push(Blizzard { x: i, y, dir: Dir::Up }),
            'v' => blizzards.push(Blizzard { x: i, y, dir: Dir::Down }),
            _ => panic!("Couldn't read char")
        }
    }
}

fn get_blizzards(day : &str) -> (Vec<Blizzard>, usize, usize) {
    let mut blizzards : Vec<Blizzard> = Vec::new();
    let mut y = 0;
    let mut len = 0;
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        len = line.len();
        
        add_blizzards(&mut blizzards, line, y);
        y+=1;
    }

    (blizzards, len, y)
}

fn part2(day : &str) {
    
    let (mut blizzards, width, height) = get_blizzards(day);
    let mut steps = 0;
    let mut positions = vec![(1,0)];
    while !positions.contains(&(width-2,height-1)) {
        move_blizzards(&mut blizzards, width, height);
        positions = get_new_potential_positions(&blizzards, &positions, width, height);
        steps+=1;
    }

    let mut positions = vec![(width-2, height-1)];
    while !positions.contains(&(1,0)) {
        move_blizzards(&mut blizzards, width, height);
        positions = get_new_potential_positions(&blizzards, &positions, width, height);
        steps+=1;
    }

    let mut positions = vec![(1,0)];
    while !positions.contains(&(width-2,height-1)) {
        move_blizzards(&mut blizzards, width, height);
        positions = get_new_potential_positions(&blizzards, &positions, width, height);
        steps+=1;
    }

    println!("Day 24 part 1: {steps}");
}

fn test() {

}