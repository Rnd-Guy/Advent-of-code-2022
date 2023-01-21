use super::utils;
use super::utils::Print::*;

// start 09:53
// part1 fin: 10:54


pub fn run(print : utils::Print ) {
    let day : &str = "23";

    assert!(day != "0", "CHANGE THE DAY");

    test(); // for any assertion tests
    //getDataSize(day);
    match print {
        Part1 => part1(day),
        Part2 => part2(day),
        BothParts => { part1(day); part2(day); }
        NoParts => (),
    }
}

fn getDataSize(day: &str) {
    let mut getRowSize = false;
    let mut lineCount = 0;
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        if !getRowSize {println!("row is {} long", line.len()); getRowSize = true;}
        lineCount+=1;
    }
    println!("there are {lineCount} rows");

}

#[derive(Clone, Copy, Debug)]
enum Dir {
    North,
    South,
    West,
    East,
}

// priority: 1 for "get first priority dir", 4 for "get bottom of list", step: "which step we are currently at"
fn getDirection(priority: u32, step: u32) -> Dir {
    let directions = [Dir::North, Dir::South, Dir::West, Dir::East];
    let nextStep = (step+2+ priority) % 4; // prevent going below 0, and step begins at 1
    return directions[nextStep as usize];
}

fn part1(day: &str) {
    // only 10 days needed to process so can just expand the grid by 10
    let mut elves = getInitialMap(day);
    //printMap(elves);
    printMap(elves);
    for step in 1..=10 {
        let proposedMap = getProposedMap(&elves, step);
        (elves,_) = updateMap(&elves, &proposedMap, step);
    }
    //printMap(elves);
    let bbox = getBoundingBox(&elves);
    let mut spaces = 0;
    for i in bbox.0..=bbox.1 {
        for j in bbox.2..=bbox.3 {
            if !elves[j][i] {spaces += 1}
        }
    }
    println!("Day 22 part 1: {}", spaces);
}

fn getBoundingBox(elves: &[[bool;200];200]) -> (usize,usize,usize,usize){
    let mut minx = elves[0].len();
    let mut maxx = 0;
    let mut miny = elves.len();
    let mut maxy = 0;

    for j in 0..elves.len() {
        for i in 0..elves[0].len() {
            if elves[j][i] {
                if i < minx {minx = i}
                if i > maxx {maxx = i}
                if j < miny {miny = j}
                if j > maxy {maxy = j}
            }
        }
    }

    (minx,maxx,miny,maxy)
}

// 2nd return value is for part 2, whether any elves moved
fn updateMap(elves: &[[bool;200];200], proposedMap: &[[u32;200];200], step: u32) -> ([[bool;200];200], bool) {
    let mut newMap = [[false;200];200];
    let mut moved = false;
    for j in 0..elves.len() {
        for i in 0..elves[0].len() {
            if elves[j][i] {
                let proposed = getProposedDirection(elves, i, j, step);
                if proposedMap[proposed.1][proposed.0] == 1 {
                    newMap[proposed.1][proposed.0] = true;
                    if proposed.0 != i || proposed.1 != j {
                        moved = true;
                    }
                } else {
                    newMap[j][i] = true;
                }
            }
        }
    }
    (newMap, moved)
}

fn printMap(elves: [[bool;200];200]) {
    println!("Printing map:");
    for j in 0..elves.len() {
        let mut row = String::new();
        for i in 0..elves[0].len() {
            row.push(match elves[j][i] {false => '.', true => '#'});
        }
        println!("{}", row);
    }

}

// gives all the proposed positions are
fn getProposedMap(elves: &[[bool;200];200], step: u32) -> [[u32;200];200]{
    let mut proposedMap = [[0;200];200];
    for j in 0..elves.len() {
        for i in 0..elves[0].len() {
            if elves[j][i] == true {
                let proposed = getProposedDirection(elves, i, j, step);
                proposedMap[proposed.1][proposed.0] += 1;
            }
        }
    }
    return proposedMap;
}

fn getProposedDirection(elves: &[[bool;200];200], x: usize, y: usize, step: u32) -> (usize,usize) {
    let mut surrounding_elves = 0; // including self
    for i in x-1..=x+1 {
        for j in y-1..=y+1 {
            if elves[j][i] {
                surrounding_elves+=1;
            }
        }
    }

    if surrounding_elves == 1 {
        return (x,y);
    }
    
    for priority in 1..=4 {
        let dir = getDirection(priority,step);
        let mut open = true;
        let proposed ;
        match dir {
            Dir::North => {
                proposed = (x,y-1);
                for i in x-1..=x+1 {
                    if elves[y-1][i] {
                        open = false;
                        break;
                    }
                }
            },
            Dir::South => {
                proposed = (x,y+1);
                for i in x-1..=x+1 {
                    if elves[y+1][i] {
                        open = false;
                        break;
                    }
                }
            },
            Dir::West => {
                proposed = (x-1,y);
                for j in y-1..=y+1 {
                    if elves[j][x-1] {
                        open = false;
                        break;
                    }
                }
            },
            Dir::East => {
                proposed = (x+1,y);
                for j in y-1..=y+1 {
                    if elves[j][x+1] {
                        open = false;
                        break;
                    }
                }
            }
        }
        if open {
            return proposed
        }
    }

    // all directions full, stand still
    return (x,y);
}

fn getInitialMap(day: &str) -> [[bool;200];200] {
    let mut elves = [[false;200];200];
    let lineOffset = 70;
    let mut lineNumber = 0;
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        
        let array: Vec<char> = line.chars().collect();
        for i in 0..array.len() {
            match array[i] {
                '#' => elves[lineOffset + lineNumber][lineOffset+i] = true,
                _ => (),
            }
        }
        lineNumber += 1;

    }

    elves
}

fn part2(day : &str) {
    // for part 2, the map size was changed from 90 to 200
    let mut elves = getInitialMap(day);
    let mut moved ;
    let mut no_move_step = 0;
    for step in 1.. {
        let proposedMap = getProposedMap(&elves, step);
        (elves,moved) = updateMap(&elves, &proposedMap, step);
        if !moved {
            no_move_step = step;
            break;
        }
    }
    println!("Day 22 part 2: {}", no_move_step);
}

fn test() {

}