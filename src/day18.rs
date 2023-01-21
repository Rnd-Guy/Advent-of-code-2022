use super::utils;
use super::utils::Print::*;
// 14:48 start
// 14:58 part 2

pub fn run(print : utils::Print ) {
    let day : &str = "18";

    assert!(day != "0", "CHANGE THE DAY");

    test(); // for any assertion tests

    match print {
        Part1 => part1(day),
        Part2 => part2_attempt_2(day),
        BothParts => { part1(day); part2_attempt_2(day); }
        NoParts => (),
    }
}

fn part1(day: &str) {

    let mut grid = [[[false;50];50];50];
    let mut total_surface_area = 0;
    
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        
        let parsed_line = utils::regex_to_vec(line, &r"^([0-9]+),([0-9]+),([0-9]+)$".to_string());

        let x: usize = parsed_line[0][1].parse().unwrap();
        let y: usize = parsed_line[0][2].parse().unwrap();
        let z: usize = parsed_line[0][3].parse().unwrap();

        grid[z][y][x] = true;

        total_surface_area += 6;
        if x > 0 && grid[z][y][x-1] {
            total_surface_area-=2;
        }
        if grid[z][y][x+1] {
            total_surface_area-=2;
        }
        if y > 0 && grid[z][y-1][x] {
            total_surface_area-=2;
        }
        if grid[z][y+1][x] {
            total_surface_area-=2;
        }
        if z > 0 && grid[z-1][y][x] {
            total_surface_area-=2;
        }
        if grid[z+1][y][x] {
            total_surface_area-=2;
        }

    }

    println!("Day 18 Part 1: {total_surface_area}");
}

fn part2_attempt_1(day : &str) {
    
    let mut grid = [[[false;50];50];50];
    let mut total_surface_area = 0;

    let mut min_x = 50;
    let mut max_x = 0;
    let mut min_y = 50;
    let mut max_y = 0;
    let mut min_z = 50;
    let mut max_z = 0;
    
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        
        let parsed_line = utils::regex_to_vec(line, &r"^([0-9]+),([0-9]+),([0-9]+)$".to_string());

        let x: usize = parsed_line[0][1].parse().unwrap();
        let y: usize = parsed_line[0][2].parse().unwrap();
        let z: usize = parsed_line[0][3].parse().unwrap();

        grid[z][y][x] = true;

        if x < min_x {
            min_x = x;
        }
        if x > max_x {
            max_x = x;
        }
        if y < min_y {
            min_y = y;
        }
        if y > max_y {
            max_y = y;
        }
        if z < min_z {
            min_z = z;
        }
        if z > max_z {
            max_z = z;
        }

        total_surface_area += 6;
        if x > 0 && grid[z][y][x-1] {
            total_surface_area-=2;
        }
        if grid[z][y][x+1] {
            total_surface_area-=2;
        }
        if y > 0 && grid[z][y-1][x] {
            total_surface_area-=2;
        }
        if grid[z][y+1][x] {
            total_surface_area-=2;
        }
        if z > 0 && grid[z-1][y][x] {
            total_surface_area-=2;
        }
        if grid[z+1][y][x] {
            total_surface_area-=2;
        }

    }

    
    for x in min_x..=max_x {
        println!("{x}");
        for y in min_y..=max_y {
            for z in min_z..=max_z {
                if !check_if_escapable(&grid, x, y, z, (min_x,max_x,min_y,max_y,min_z,max_z), &Vec::new()) {
                    grid[z][y][x] = true;

                    total_surface_area += 6;
                    if x > 0 && grid[z][y][x-1] {
                        total_surface_area-=2;
                    }
                    if grid[z][y][x+1] {
                        total_surface_area-=2;
                    }
                    if y > 0 && grid[z][y-1][x] {
                        total_surface_area-=2;
                    }
                    if grid[z][y+1][x] {
                        total_surface_area-=2;
                    }
                    if z > 0 && grid[z-1][y][x] {
                        total_surface_area-=2;
                    }
                    if grid[z+1][y][x] {
                        total_surface_area-=2;
                    }
                }
            }
        }
    }

    println!("Day 18 Part 2: {total_surface_area}");
}

fn check_if_escapable(grid: &[[[bool;50];50];50], x: usize, y: usize, z: usize, bounds: (usize,usize,usize,usize,usize,usize), checked: &Vec<(usize, usize, usize)>) -> bool{
    if x == bounds.0 || x == bounds.1 || y == bounds.2 || y == bounds.3 || z == bounds.4 || z == bounds.5 {
        return true;
    }

    let mut clone = checked.clone();
    clone.push((x,y,z));

    if !grid[z][y][x-1] && !checked.contains(&(x-1,y,z)) && check_if_escapable(grid, x-1, y, z, bounds, &clone) {
        return true;
    }
    if !grid[z][y][x+1] && !checked.contains(&(x+1,y,z)) && check_if_escapable(grid, x+1, y, z, bounds, &clone) {
        return true;
    }
    if !grid[z][y-1][x] && !checked.contains(&(x,y-1,z)) &&  check_if_escapable(grid, x, y-1, z, bounds, &clone) {
        return true;
    }
    if !grid[z][y+1][x] && !checked.contains(&(x,y+1,z)) &&  check_if_escapable(grid, x, y+1, z, bounds, &clone) {
        return true;
    }
    if !grid[z-1][y][x] && !checked.contains(&(x,y,z-1)) &&  check_if_escapable(grid, x, y, z-1, bounds, &clone) {
        return true;
    }
    if !grid[z+1][y][x] && !checked.contains(&(x,y,z+1)) &&  check_if_escapable(grid, x, y, z+1, bounds, &clone) {
        return true;
    }

    false
}

#[derive(Clone, Copy)]
enum Cube {
    Filled, // filled by data
    Empty, // empty and not checked
    CheckedFilled, // was checked and was contained
    CheckedEmpty, // was checked and is free
    Checking, // in the middle of checking
}

fn fill_all_checking(grid: &mut [[[Cube;50];50];50], current_area: &mut i32, bounds: (usize, usize, usize, usize, usize, usize), fill_type: Cube) {
    
    for x in bounds.0..=bounds.1 {
        for y in bounds.2..=bounds.3 {
            for z in bounds.4..=bounds.5 {
                match grid[z][y][x] {
                    Cube::Checking => fill_cube(grid, x, y, z, current_area, fill_type),
                    _ => continue,
                }
            }
        }
    }
}

fn fill_cube(grid: &mut [[[Cube;50];50];50], x: usize, y: usize, z: usize, current_area: &mut i32, fill_type: Cube) {
    
    match grid[z][y][x] {
        Cube::Filled | Cube::CheckedFilled => return,
        Cube::CheckedEmpty => match fill_type { Cube::CheckedEmpty => return, _ => panic!("tried to fill area that should be empty")},
        Cube::Empty | Cube::Checking => grid[z][y][x] = fill_type,
    }

    match fill_type {
        Cube::CheckedEmpty | Cube::Empty => return,
        Cube::Checking => panic!("Only modify_if_escapable should set grid to Checking"),
        _ => (),
    }

    *current_area += 6;
    if x > 0 && match grid[z][y][x-1] {Cube::Filled | Cube::CheckedFilled => true, _ => false} {
        *current_area-=2;
    }
    if match grid[z][y][x+1] {Cube::Filled | Cube::CheckedFilled => true, _ => false} {
        *current_area-=2;
    }
    if y > 0 && match grid[z][y-1][x] {Cube::Filled | Cube::CheckedFilled => true, _ => false} {
        *current_area-=2;
    }
    if match grid[z][y+1][x] {Cube::Filled | Cube::CheckedFilled => true, _ => false} {
        *current_area-=2;
    }
    if z > 0 && match grid[z-1][y][x] {Cube::Filled | Cube::CheckedFilled => true, _ => false} {
        *current_area-=2;
    }
    if match grid[z+1][y][x] {Cube::Filled | Cube::CheckedFilled => true, _ => false} {
        *current_area-=2;
    }
}

fn part2_attempt_2(day : &str) {
    
    let mut grid = [[[Cube::Empty;50];50];50];
    let mut total_surface_area = 0;

    let mut min_x = 50;
    let mut max_x = 0;
    let mut min_y = 50;
    let mut max_y = 0;
    let mut min_z = 50;
    let mut max_z = 0;
    
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        
        let parsed_line = utils::regex_to_vec(line, &r"^([0-9]+),([0-9]+),([0-9]+)$".to_string());

        let x: usize = parsed_line[0][1].parse().unwrap();
        let y: usize = parsed_line[0][2].parse().unwrap();
        let z: usize = parsed_line[0][3].parse().unwrap();

        if x < min_x {
            min_x = x;
        }
        if x > max_x {
            max_x = x;
        }
        if y < min_y {
            min_y = y;
        }
        if y > max_y {
            max_y = y;
        }
        if z < min_z {
            min_z = z;
        }
        if z > max_z {
            max_z = z;
        }

        fill_cube(&mut grid, x, y, z, &mut total_surface_area, Cube::Filled);

    }

    let bounds = (min_x, max_x, min_y, max_y, min_z, max_z);
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            for z in min_z..=max_z {
                match grid[z][y][x] {
                    Cube::Filled | Cube::CheckedFilled | Cube::CheckedEmpty => (),
                    Cube::Checking => panic!("Shouldn't find any checking at this point!"),
                    Cube::Empty => {
                        // let result = modify_if_escapable(&mut grid, x, y, z, bounds, &Vec::new());
                        // for i in result.0 {
                        //     if result.2 == false {
                        //         fill_cube(&mut grid, i.0, i.1, i.2, &mut total_surface_area, result.1);
                        //     } else {
                        //         fill_cube(&mut grid, i.0, i.1, i.2, &mut total_surface_area, Cube::CheckedFilled);
                        //     }
                        // }
                        let result = modify_if_escapable(&mut grid, x, y, z, bounds);
                        if result.1 {
                            // couldn't find an escape
                            fill_all_checking(&mut grid, &mut total_surface_area, bounds, Cube::CheckedFilled);
                        } else {
                            fill_all_checking(&mut grid, &mut total_surface_area, bounds, result.0);
                        }
                    }
                }
               

            }
        }
    }

    println!("Day 18 Part 2: {total_surface_area}");
}

fn modify_if_escapable(grid: &mut [[[Cube;50];50];50], x: usize, y: usize, z: usize, bounds: (usize,usize,usize,usize,usize,usize)) -> (Cube, bool){
    grid[z][y][x] = Cube::Checking;

    if x == bounds.0 || x == bounds.1 || y == bounds.2 || y == bounds.3 || z == bounds.4 || z == bounds.5 {
        return (Cube::CheckedEmpty, false);
    }

    let mut check : Vec<(usize,usize,usize)> = Vec::new();
    match grid[z][y][x-1] {
        Cube::Empty => check.push((x-1,y,z)),
        Cube::CheckedFilled => return (Cube::CheckedFilled, false),
        Cube::CheckedEmpty => return (Cube::CheckedEmpty, false),
        Cube::Filled | Cube::Checking => (),
    }
    match grid[z][y][x+1] {
        Cube::Empty => check.push((x+1,y,z)),
        Cube::CheckedFilled => return (Cube::CheckedFilled,false),
        Cube::CheckedEmpty => return (Cube::CheckedEmpty,false),
        Cube::Filled | Cube::Checking => (),
    }
    match grid[z][y-1][x] {
        Cube::Empty => check.push((x,y-1,z)),
        Cube::CheckedFilled => return (Cube::CheckedFilled,false),
        Cube::CheckedEmpty => return (Cube::CheckedEmpty,false),
        Cube::Filled | Cube::Checking => (),
    }
    match grid[z][y+1][x] {
        Cube::Empty => check.push((x,y+1,z)),
        Cube::CheckedFilled => return (Cube::CheckedFilled,false),
        Cube::CheckedEmpty => return (Cube::CheckedEmpty,false),
        Cube::Filled | Cube::Checking => (),
    }
    match grid[z-1][y][x] {
        Cube::Empty => check.push((x,y,z-1)),
        Cube::CheckedFilled => return (Cube::CheckedFilled,false),
        Cube::CheckedEmpty => return (Cube::CheckedEmpty,false),
        Cube::Filled | Cube::Checking => (),
    }
    match grid[z+1][y][x] {
        Cube::Empty => check.push((x,y,z+1)),
        Cube::CheckedFilled => return (Cube::CheckedFilled,false),
        Cube::CheckedEmpty => return (Cube::CheckedEmpty,false),
        Cube::Filled | Cube::Checking => (),
    }
    for c in &check {
        let answer = modify_if_escapable(grid, c.0, c.1, c.2, bounds);
        if answer.1 == false {
            return (answer.0, false);
        }
    }

    return (Cube::Checking, true);
}

fn test() {

}