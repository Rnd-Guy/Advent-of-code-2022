use super::utils;
use super::utils::Print::*;

pub fn run(print : utils::Print ) {
    let day : &str = "14";

    assert!(day != "0", "CHANGE THE DAY");

    test(); // for any assertion tests

    match print {
        Part1 => part1(day),
        Part2 => part2(day),
        BothParts => { part1(day); part2(day); }
        NoParts => (),
    }
}

#[derive(Clone)]
enum Item {
    Rock,
    Sand,
    SandSpawner,
    Air,
}

impl Item {
    fn to_string(&self) -> String {
        let s = match self {
            Self::Rock => "#",
            Self::Sand =>  "o",
            Self::SandSpawner => "O",
            Self::Air => ".",

        };

        s.to_string()
    }
}

fn part1(day: &str) {

    let mut map : Vec<Vec<Item>> = vec![vec![Item::Air; 1000]; 200];
    let mut max_x = 1000;
    let mut max_y = 200;
    let mut actual_min_x = 500;
    let mut actual_min_y = 0;
    let mut actual_max_x = 500;
    let mut actual_max_y = 0;

    map[0][500] = Item::SandSpawner;
    
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        
        let split = line.split(" -> ");
        let mut previous_coordinate = (0,0);
        for coordinate in split {
            let mut c_split = coordinate.split(",");
            let x: usize = c_split.next().unwrap().parse().unwrap();
            let y: usize = c_split.next().unwrap().parse().unwrap();

            // extend if needed
            if x > max_x {
                for i in max_x..x {
                    for v in &mut map {
                        v.push(Item::Air);
                    }
                }
                max_x = x;
            }

            if x > actual_max_x {
                actual_max_x = x;
            }
            if x < actual_min_x {
                actual_min_x = x;
            }

            if y > max_y {
                for i in max_y..y {
                    map.push(vec![Item::Air; max_x]);
                }
            }
            
            if y > actual_max_y {
                actual_max_y = y;
            }
            if y < actual_min_y {
                actual_min_y = y;
            }

            if previous_coordinate != (0,0) {
                for i in previous_coordinate.0..=x {
                    map[y][i] = Item::Rock;
                }
                for i in x..=previous_coordinate.0 {
                    map[y][i] = Item::Rock;
                }
                for j in previous_coordinate.1..=y {
                    map[j][x] = Item::Rock;
                }
                for j in y..=previous_coordinate.1 {
                    map[j][x] = Item::Rock;
                }
            }

            previous_coordinate = (x, y);
        }

    }

    //print_map(&map, actual_min_x, actual_min_y, actual_max_x, actual_max_y);
    
    let mut sand = 0;
     while !drop_sand(&mut map, 500, 0, actual_max_y) {
         sand += 1;
     }
    
    print_map(&map, actual_min_x, actual_min_y, actual_max_x, actual_max_y);

    println!("Day 14 part 1: {sand}");


}

fn drop_sand(map: &mut Vec<Vec<Item>>, spawn_x: usize, spawn_y: usize, max_y: usize) -> bool {
    for y in spawn_y..=max_y {
        match map[y][spawn_x] {
            Item::Rock | Item::Sand => {
                if let Item::Air = map[y][spawn_x-1] {
                    return drop_sand(map, spawn_x-1, y, max_y);
                } else if let Item::Air = map[y][spawn_x+1] {
                    return drop_sand(map, spawn_x+1, y, max_y);
                } else {
                    map[y-1][spawn_x] = Item::Sand;
                    return false;
                }
            },
            _ => continue,
        };
    }

    // dropped off the map
    return true;
}

fn print_map(map: &Vec<Vec<Item>>, min_x: usize, min_y: usize, max_x: usize, max_y: usize) {
    for y in min_y..=max_y {
        let mut row = "".to_string();
        for x in min_x..=max_x {
            row.push_str(map[y][x].to_string().as_str());
        }
        println!("{row}");
    }
}

fn part2(day : &str) {
    
    let mut map : Vec<Vec<Item>> = vec![vec![Item::Air; 1000]; 200];
    let mut max_x = 1000;
    let mut max_y = 200;
    let mut actual_min_x = 500;
    let mut actual_min_y = 0;
    let mut actual_max_x = 500;
    let mut actual_max_y = 0;

    map[0][500] = Item::SandSpawner;
    
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        
        let split = line.split(" -> ");
        let mut previous_coordinate = (0,0);
        for coordinate in split {
            let mut c_split = coordinate.split(",");
            let x: usize = c_split.next().unwrap().parse().unwrap();
            let y: usize = c_split.next().unwrap().parse().unwrap();

            // extend if needed
            if x > max_x {
                for i in max_x..x {
                    for v in &mut map {
                        v.push(Item::Air);
                    }
                }
                max_x = x;
            }

            if x > actual_max_x {
                actual_max_x = x;
            }
            if x < actual_min_x {
                actual_min_x = x;
            }

            if y > max_y {
                for i in max_y..y {
                    map.push(vec![Item::Air; max_x]);
                }
            }
            
            if y > actual_max_y {
                actual_max_y = y;
            }
            if y < actual_min_y {
                actual_min_y = y;
            }

            if previous_coordinate != (0,0) {
                for i in previous_coordinate.0..=x {
                    map[y][i] = Item::Rock;
                }
                for i in x..=previous_coordinate.0 {
                    map[y][i] = Item::Rock;
                }
                for j in previous_coordinate.1..=y {
                    map[j][x] = Item::Rock;
                }
                for j in y..=previous_coordinate.1 {
                    map[j][x] = Item::Rock;
                }
            }

            previous_coordinate = (x, y);
        }

    }

    for i in 0..max_x {
        map[actual_max_y+2][i] = Item::Rock;
    }

    actual_max_y += 2;

    //print_map(&map, actual_min_x, actual_min_y, actual_max_x, actual_max_y);
    
    let mut sand = 0;
     while let Item::SandSpawner = map[0][500] {
        drop_sand(&mut map, 500, 0, actual_max_y);
         sand += 1;
     }
    
    print_map(&map, actual_min_x, actual_min_y, actual_max_x, actual_max_y);

    println!("Day 14 part 2: {sand}");

}

fn test() {

}