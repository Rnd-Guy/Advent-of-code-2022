use super::utils;
use super::utils::Print::*;

pub fn run(print : utils::Print ) {
    let day : &str = "17";

    assert!(day != "0", "CHANGE THE DAY");

    test(); // for any assertion tests

    match print {
        Part1 => part1(day),
        Part2 => part2(day),
        BothParts => { part1(day); part2(day); }
        NoParts => (),
    }
}

// |#######|
// |.......|
// |.......|
// |.......|
// +-------+

fn part1(day: &str) {

    let mut wind: Vec<char> = Vec::new();
    
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        
        wind = line.chars().collect();

    }

    let mut field: Vec<[bool; 7]> = Vec::new();
    field.push([true;7]);

    // loop will look like this:
    // (shape, next_shape) = get_shape (2, top_of_grid+3)
    // (shape, index) = move_shape(shape, wind, index)
    // collision = drop_shape(shape)
    // if (collision) continue;

    let mut next_shape = "row".to_string();
    let mut top_of_field = 0;
    let mut wind_index = 0;

    let mut shapes = 0;
    let target_shapes: u64 = 2022;

    while shapes < target_shapes {
        let next = get_next_shape_on_board(next_shape, &mut field, top_of_field);
        let mut shape = next.0;
        next_shape = next.1;

        let mut collision = false;
        while !collision {
            move_shape(&mut shape, &field, &wind, wind_index);
            wind_index += 1;
            if wind_index == wind.len() {
                wind_index = 0;
            }
            collision = drop_shape(&mut shape, &field);
        }

        let top = fill_shape(&shape, &mut field);
        if top > top_of_field {
            top_of_field = top;
        }
        shapes += 1;
        if shapes % 10000000000 == 0 {
            println!("%");
        }

        //print_field(&field);
    }

    println!("Day 17 Part 1: {top_of_field}");

}

fn fill_shape(shape: &Shape, field: &mut Vec<[bool;7]>) -> usize {
    let mut top = 0;
    for c in &shape.coordinates {
        let x = c.0 + shape.center.0;
        let y = c.1 + shape.center.1;
        if top < y {
            top = y;
        }
        field[y][x] = true;
    }

    top
}

fn print_field(field: &Vec<[bool; 7]>) {
    
    for r in (1..field.len()).rev() {
        let mut s = String::new();
        s.push('|');
        let mut all_true = true;
        for c in field[r] {
            if c {
                s.push('#');
            } else {
                s.push('.');
                all_true = false;
            }
        }
        s.push('|');
        // if all_true {
        //     println!("row {r} was all full!");
        // }
        println!("{}", s);
    }
    println!("+-------+");
    println!();
    
}

fn print_top_ten_field(field: &Vec<[bool; 7]>) {
    
    for r in (field.len()-11..field.len()).rev() {
        let mut s = String::new();
        s.push('|');
        let mut all_true = true;
        for c in field[r] {
            if c {
                s.push('#');
            } else {
                s.push('.');
                all_true = false;
            }
        }
        s.push('|');
        // if all_true {
        //     println!("row {r} was all full!");
        // }
        println!("{}", s);
    }
    println!("+-------+");
    println!();
    
}

fn move_shape(shape: &mut Shape, field: &Vec<[bool;7]>, wind: &Vec<char>, wind_index: usize) {
    let mut wind_index = wind_index;
    if wind_index >= wind.len() {
        wind_index = 0;
    }

    let direction = wind.get(wind_index).unwrap();
    let right_wind: bool = if direction == &'<' {false} else if direction == &'>' {true} else {panic!("bad direction: {direction}")};

    let mut collision = false;
    for coord in &shape.coordinates {
        let c = (&shape.center.0 + coord.0, &shape.center.1 + coord.1);

        if (c.0 == 0 && !right_wind) || (c.0 == 6 && right_wind){
            collision = true;
            break;
        }
        if (right_wind && field[c.1][c.0+1]) || (!right_wind && field[c.1][c.0-1]) {
            collision = true;
            break;
        }
    }

    if !collision {
        //shape.center.0 = ((shape.center.0 as i32) + direction) as usize;
        if right_wind {
            shape.center.0 += 1;
        } else {
            shape.center.0 -= 1;
        }
    }
    
}

fn drop_shape(shape: &mut Shape, field: &Vec<[bool;7]>) -> bool {
    for coord in &shape.coordinates {
        let c = (&shape.center.0 + coord.0, &shape.center.1 + coord.1 - 1);

        if field[c.1 as usize][c.0 as usize] {
            return true;
        }
    }

    shape.center.1 -= 1;
    return false;
}

fn _get_next_shape(s: &str, x: usize, y: usize) -> (Shape, String) {
    match s {
        "row" => (Shape::row(x,y), "plus".to_string()),
        "plus" => (Shape::plus(x,y), "corner".to_string()),
        "corner" => (Shape::corner(x,y), "col".to_string()),
        "col" => (Shape::col(x,y), "square".to_string()),
        "square" => (Shape::square(x,y), "row".to_string()),
        _ => panic!("couldn't match s"),
    }  
}

// is in charge of extending the field
fn get_next_shape_on_board(s: String, field: &mut Vec<[bool; 7]>, top_of_field: usize) -> (Shape, String) {
    let new_center_height: usize = top_of_field + 4;
    while field.len() < new_center_height+4 {
        field.push([false;7]);
    }

    _get_next_shape(s.as_str(), 2, new_center_height)
}

struct Shape {
    center: (usize,usize), // the bottom left block
    coordinates: Vec<(usize,usize)>,// relative to center
}

impl Shape {
    fn row(x: usize,y: usize) -> Self {
        Self{center: (x,y), coordinates: vec![(0,0), (1,0), (2,0), (3,0)]}
    }

    fn plus(x: usize, y: usize) -> Self {
        Self{center: (x,y), coordinates: vec![(0,1), (1,0), (1,1), (1,2), (2,1)]}
    }

    fn corner(x: usize, y: usize) -> Self {
        Self{center: (x,y), coordinates: vec![(0,0), (1,0), (2,0), (2,1), (2,2)]}
    }

    fn col(x: usize, y: usize) -> Self {
        Self{center: (x,y), coordinates: vec![(0,0), (0,1), (0,2), (0,3)]}
    }

    fn square(x: usize, y: usize) -> Self {
        Self{center: (x,y), coordinates: vec![(0,0), (1,0), (0,1), (1,1)]}
    }

}

fn get_height_after_x_rocks_from_start(wind: &Vec<char>, x: u64) -> u64 {
    let mut field: Vec<[bool; 7]> = Vec::new();
    field.push([true;7]);
    get_height_after_x_rocks(wind, x, &mut field, 0, 0).0
}

fn get_height_after_x_rocks(wind: &Vec<char>, x: u64, field: &mut Vec<[bool; 7]>, top_of_field: usize, wind_index: usize) -> (u64, usize) {
    let mut next_shape = "row".to_string();
    let mut top_of_field = top_of_field;
    let mut wind_index = wind_index;

    let mut shapes = 0;
    let target_shapes: u64 = x;

    while shapes < target_shapes {
        let next = get_next_shape_on_board(next_shape, field, top_of_field);
        let mut shape = next.0;
        next_shape = next.1;

        let mut collision = false;
        while !collision {
            move_shape(&mut shape, field, &wind, wind_index);
            wind_index += 1;
            if wind_index == wind.len() {
                wind_index = 0;
            }
            collision = drop_shape(&mut shape, field);
        }

        let top = fill_shape(&shape, field);
        if top > top_of_field {
            top_of_field = top;
        }
        shapes += 1;

    }
    
    //print_top_ten_field(field);

    return (top_of_field as u64, wind_index);
}

// 1000000000000 rocks wtf
fn part2(day : &str) {
    
    let mut wind: Vec<char> = Vec::new();
    
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        
        wind = line.chars().collect();

    }

    // the strat:
    // if we consider one cycle = wind length * 5
    // then we will be back at the starting position for both
    // there will likely be some sort of loop, find that loop and we can just multiply

    let cycle_len = wind.len() as u64 * 5;
    let total_rocks: u64 = 1000000000000;

    let target_shapes: u64 = cycle_len * 50;

    let mut prev_h = 0;
    let mut field: Vec<[bool; 7]> = Vec::new();
    field.push([true;7]);
    let mut top_of_field = 0;
    let mut wind_index = 0;
    for i in 1..1 {
        let (h, w) = get_height_after_x_rocks(&wind, cycle_len, &mut field, top_of_field as usize, wind_index);

        // this was used to find the start of the cycle, using the fact that 80325 appears twice in a row in one cycle
        if h - prev_h == 80325 {
            println!("{i}");
        }

        // this was used to confirm that the cycle does happen
        // let expected = match i % 7 {
        //     1 => 301,
        //     2 => 300,
        //     3 => 306,
        //     4 => 303,
        //     5 => 303,
        //     6 => 301,
        //     0 => 306,
        //     _ => panic!(""),
        // };
        // if h - prev_h != expected {
        //     println!("expected {expected}, got {}", h-prev_h);
        // }

        prev_h = h;
        top_of_field = h;
        wind_index = w;

    }

    // fill these two fields in after finding the loop
    // for the test case: first cycle = 308 but then it repeats [300 306 303 303 301 306 301]
    // so test case = (total_rocks / (cycle_len * 7)) * sum of repeat
    let cycles_before_loop = 42;  // find this manually, for test it was 1, real data was 42 (prob earlier though)
    let cycle_loop_length = 349;        // find this manually, for test it was 7, real data was 349

    let total_rocks_before_loop = cycle_len * cycles_before_loop;
    let total_rocks_in_loop = cycle_len * cycle_loop_length;

    let before_loop_height = get_height_after_x_rocks_from_start(&wind, total_rocks_before_loop);
    let height_after_one_loop = get_height_after_x_rocks_from_start(&wind, cycle_len*cycles_before_loop + total_rocks_in_loop);
    let loop_height = height_after_one_loop - before_loop_height;

    let mut total_loops = total_rocks / total_rocks_in_loop;
    let mut remainder_rocks = total_rocks % total_rocks_in_loop;
    while remainder_rocks < total_rocks_before_loop {
        total_loops -= 1;
        remainder_rocks += total_rocks_in_loop;
    }

    // for safety, use the heights after at least one loop
    let remainder_height = get_height_after_x_rocks_from_start(&wind, remainder_rocks + total_rocks_in_loop);
    let before_height = get_height_after_x_rocks_from_start(&wind, total_rocks_before_loop + total_rocks_in_loop);
    let pre_loop_height = get_height_after_x_rocks_from_start(&wind, total_rocks_before_loop);
    let remainder = remainder_height - before_height;

    let answer = (total_loops * loop_height) + remainder + pre_loop_height;
    println!("Day 17 Part 2: {answer}");

}

fn test() {

}