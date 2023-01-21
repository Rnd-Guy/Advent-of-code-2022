use super::utils;
use super::utils::Print::*;

// for debugging
use std::{thread, time};

// start 09:59
// part1 fin: 11:11
pub fn run(print : utils::Print ) {
    let day : &str = "22";

    assert!(day != "0", "CHANGE THE DAY");

    if !matches!(print, NoParts) {test();} // for any assertion tests

    match print {
        Part1 => part1(day),
        Part2 => part2(day),
        BothParts => { part1(day); part2(day); }
        NoParts => (),
    }
}

#[derive(Debug)]
enum Tile {
    Open,
    Wall,
    Off,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir {
    Left,
    Right,
    Up,
    Down
}

#[derive(Debug)]
struct Me {
    x: usize,
    y: usize,
    facing: Dir
}

fn part1(day: &str) {

    //let map = [[Tile;150]]
    let map = get_map(day);
    let instruction = get_instruction(day);

    let mut me = get_start_position(&map);

    let instructions = utils::regex_to_vec(instruction, &r"(?:([0-9]+)|([LR]))".to_string());
    //println!("{:?}", instructions);
    for i in 0..instructions.len() {
        // if direction
        if instructions[i][1].len() == 0 {
            rotate_me(&mut me, instructions[i][2].chars().next().unwrap());
        } else if instructions[i][2].len() == 0 {
            //println!("{}", instructions[i][1]);
            move_me(&map, &mut me, instructions[i][1].parse().unwrap());
        }
    }

    // final value: right = 0, down = 1, left = 2, up = 3
    //  4(x+1) * 1000(y+1) + direction
    //println!("{:?}", me);

    println!("Day 22 part 1: {}", (me.x+1) *4 +  (me.y+1)*1000 + match me.facing {Dir::Right => 0, Dir::Down => 1, Dir::Left => 2, Dir::Up => 3});


}

fn move_me(map: &Vec<Vec<Tile>>, me: &mut Me, amount: u32) {
    let mut increment_x = 0;
    let mut increment_y = 0;

    let width = map[0].len();
    let height = map.len();

    match &me.facing {
        Dir::Up => increment_y = -1,
        Dir::Down => increment_y = 1,
        Dir::Left => increment_x = -1,
        Dir::Right => increment_x = 1,
    }

    for _ in 0..amount {

        // use i32 to prevent overflowing from going below 0, maybe there's a better way though
        let mut new_x = me.x as i32 + increment_x;
        let mut new_y = me.y as i32 + increment_y;

        // first guarantee that the next position we are looking is in bounds (both as a whole and on the grid)
        if new_x < 0 || (matches!(me.facing, Dir::Left) && matches!(map[me.y][new_x as usize], Tile::Off)) {
            for i in (0..width).rev() {
                match map[me.y][i] {
                    Tile::Off => continue,
                    Tile::Open => {new_x = i as i32; break;},
                    Tile::Wall => {new_x = me.x as i32; break;},
                }
            }
        }
        if new_x as usize >= width || (matches!(me.facing, Dir::Right) && matches!(map[me.y][new_x as usize], Tile::Off)){
            for i in 0..width {
                match map[me.y][i] {
                    Tile::Off => continue,
                    Tile::Open => {new_x = i as i32; break;},
                    Tile::Wall => {new_x = me.x as i32; break;},
                }
            }
        }
        if new_y < 0 || (matches!(me.facing, Dir::Up) && matches!(map[new_y as usize][me.x], Tile::Off)){
            for j in (0..height).rev() {
                match map[j][me.x] {
                    Tile::Off => continue,
                    Tile::Open => {new_y = j as i32; break;},
                    Tile::Wall => {new_y = me.y as i32; break;},
                }
            }
        }
        if new_y as usize >= height || (matches!(me.facing, Dir::Down) && matches!(map[new_y as usize][me.x], Tile::Off)) {
            for j in 0..height {
                match map[j][me.x] {
                    Tile::Off => continue,
                    Tile::Open => {new_y = j as i32; break;},
                    Tile::Wall => {new_y = me.y as i32; break;},
                }
            }
        }

        // then check that the next position isn't a wall
        if matches!(map[new_y as usize][new_x as usize], Tile::Wall) {
            new_x = me.x as i32;
            new_y = me.y as i32;
        }

        // if the next spot is the same spot, ie if we got blocked then we're finished
        if new_x == me.x as i32 && new_y == me.y as i32{
            break;
        }

        me.x = new_x as usize;
        me.y = new_y as usize;
    }
    
}

fn rotate_me(me: &mut Me, dir: char) {
    println!("rotate {dir}");
    match dir {
        'R' => match me.facing {
            Dir::Left => me.facing = Dir::Up,
            Dir::Up => me.facing = Dir::Right,
            Dir::Right => me.facing = Dir::Down,
            Dir::Down => me.facing = Dir::Left,
        },
        'L' => match me.facing {
            Dir::Left => me.facing = Dir::Down,
            Dir::Down => me.facing = Dir::Right,
            Dir::Right => me.facing = Dir::Up,
            Dir::Up => me.facing = Dir::Left,
        },
        _ => panic!("couldn't rotate, couldn't parse dir")
    }
}

fn get_start_position(map: &Vec<Vec<Tile>>) -> Me{
    for i in 0..map[0].len() {
        if matches!(map[0][i], Tile::Open) {
            return Me{x: i, y: 0, facing: Dir::Right};
        }
    }

    panic!("Couldn't find start pos");
}

fn get_map(day : &str) -> Vec<Vec<Tile>> {
    let mut map: Vec<Vec<Tile>> = Vec::new();
    let mut len = 0;

    // first find the longest length line
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        if len < line.len() {
            len = line.len();
        }
        if line.len() == 0 {
            break;
        }
    }

    // then read the map
    for line in utils::read_lines(day) {
        let line = line.unwrap();

        if line.len() == 0 {
            break;
        }

        let mut chars = line.chars().into_iter();
        let mut char_count = 0;
        let mut row = Vec::new();
        let mut current_char = chars.next();
        while !matches!(current_char, None) {
            let c = current_char.unwrap();
            match c {
                '.' => row.push(Tile::Open),
                ' ' => row.push(Tile::Off),
                '#' => row.push(Tile::Wall),
                _ => panic!("test")
            }
            char_count += 1;
            current_char = chars.next();
        }
        for _ in char_count..len {
            row.push(Tile::Off);
        }
        map.push(row);
        
    }

    map
}

fn get_instruction(day : &str) -> String {
    let mut blank = false;
    for line in utils::read_lines(day) {
        let line = line.unwrap();

        if line.len() == 0 {
            blank = true;
            continue;
        }

        if blank {
            return line;
        }
        
    }

    panic!("couldn't find instruction D:");
}

// this is going to be hardcoded for my data
fn find_next_point(pos: (i32, i32, Dir)) -> (i32, i32, Dir){
    
    //  my grid looks like this

    //            *   #
    //       ~+---+---+
    //        | B | R |
    //        +---+---+@
    //        | U |
    //    +---+---+@
    //    | L | T |
    //   ~+---+---+
    //    | D |
    //   *+---+
    //        #
    
    // where the symbols outside the map correspond to the matching side and will touch when folded
    // the placement of said symbols are also directionally correct 
    //   (the left side of B will fold onto the left side of L, with the top part of B folding onto the bottom part of L)
    // this is much harder to see in code, so there's a paint diagram: day22p.png
    // what we will therefore do is just code in each side so that they match to the associated place
    let x = pos.0;
    let y = pos.1;
    let dir = pos.2;
    let mut new_pos = (x,y,dir);

    // basically gonna do each case from top of B clockwise
    // this is going to be heavily based off of the day22p.png pic, where offset is always from the circle end of the side (ie the marked ends above)
    // ie offset = 1 is the circle end, offset = 50 is the tail end
    // as a result, the offset calc for one side should be similar to the x or y calc for the other and vice versa
    // eg if offset = 100-y then for the other side y = 100-offset
    let mut dont_print = false;
    // top of B, guaranteed to be facing up to reach here
    if y == -1 && (50..100).contains(&x) {
        // move to left of D
        let offset = 100-x;
        new_pos = (0,200-offset,Dir::Right);
    } 
    // top of R, guaranteed to be facing up to reach here
    else if y == -1 && (100..150).contains(&x) {
        // move to bottom of D
        let offset = 150-x;
        new_pos = (50-offset, 199, Dir::Up);
    }
    // right of R, guaranteed to be facing right to reach here
    else if x == 150 {
        // move to the right of T, note the direction change, top of R goes to bottom of T
        let offset = 50-y;
        new_pos = (99, 99+offset, Dir::Left);
    }
    // bottom of R, note this shares a corner with U
    else if (100..150).contains(&x) && y == 50 && matches!(dir, Dir::Down) {
        // move to the right of U
        let offset = 150-x;
        new_pos = (99, 100-offset, Dir::Left);
    }
    // right of U, yeah you get the idea
    else if x == 100 && (50..100).contains(&y) && matches!(dir, Dir::Right) {
        let offset = 100-y;
        new_pos = (150-offset, 49, Dir::Up);
    }
    // right of T
    else if x == 100 && (100..150).contains(&y) {
        let offset = y - 99;
        new_pos = (149,50-offset, Dir::Left);
    }
    // bottom of T
    else if (50..100).contains(&x) && y == 150 && matches!(dir, Dir::Down) {
        let offset = 100-x;
        new_pos = (49, 200-offset, Dir::Left);
    }
    // right of D
    else if x == 50 && (150..200).contains(&y) && matches!(dir, Dir::Right) {
        let offset = 200-y;
        new_pos = (100-offset, 149, Dir::Up);
    }
    // bottom of D
    else if y == 200 {
        let offset = 50-x;
        new_pos = (150-offset, 0, Dir::Down);
    }
    // left of D
    else if x == -1 && (150..200).contains(&y) {
        let offset = 200-y;
        new_pos = (100-offset, 0, Dir::Down);
    }
    // left of L
    else if x == -1 && (100..150).contains(&y) {
        let offset = 150-y;
        new_pos = (50, offset-1, Dir::Right);
    }
    // up of L
    else if (0..50).contains(&x) && y == 99 && matches!(dir, Dir::Up) {
        let offset = x+1;
        new_pos = (50, 49+offset, Dir::Right);
    }
    // left of U
    else if x == 49 && (50..100).contains(&y) && matches!(dir, Dir::Left) {
        let offset = y - 49;
        new_pos = (offset-1, 100, Dir::Down);
    }
    // left of B
    else if x == 49 && (0..50).contains(&y) {
        let offset = y+1;
        new_pos = (0, 150-offset, Dir::Right);
    // otherwise we're still on the map
    } else {
        dont_print = true;
        // match dir {
        //     Dir::Left => new_pos = (x-1, y, Dir::Left),
        //     Dir::Up => new_pos = (1, y-1, Dir::Up),
        //     Dir::Right => new_pos = (x+1, y, Dir::Right),
        //     Dir::Down => new_pos = (x, y+1, Dir::Down),
        // }

    }

    if !dont_print {
        println!("was at ({},{}) facing {:?}, now at ({},{}) facing {:?}", x, y, dir, new_pos.0, new_pos.1, new_pos.2);
    }

    return new_pos;


}

fn move_me2(map: &Vec<Vec<Tile>>, me: &mut Me, amount: u32) {
    println!("move {amount}");
    let mut increment_x = 0;
    let mut increment_y = 0;

    for _ in 0..amount {

        match me.facing {
            Dir::Up => {increment_y = -1; increment_x = 0},
            Dir::Down => {increment_y = 1; increment_x = 0},
            Dir::Left => {increment_x = -1; increment_y = 0},
            Dir::Right => {increment_x = 1; increment_y = 0},
        }

        // use i32 to prevent overflowing from going below 0, maybe there's a better way though
        let new_x = me.x as i32 + increment_x;
        let new_y = me.y as i32 + increment_y;

        
        let new_pos = find_next_point((new_x, new_y, me.facing));

        // then check that the next position isn't a wall
        if matches!(map[new_pos.1 as usize][new_pos.0 as usize], Tile::Wall) {
            //new_x = me.x as i32;
            //new_y = me.y as i32;
            break;
        } else if matches!(map[new_pos.1 as usize][new_pos.0 as usize], Tile::Off) {
            panic!("something went wrong D:")
        }

        me.x = new_pos.0 as usize;
        me.y = new_pos.1 as usize;
        me.facing = new_pos.2;

        //print_map(map, me);
    }
    
}

fn part2(day : &str) {
    
    //let map = [[Tile;150]]
    let map = get_map(day);
    let instruction = get_instruction(day);

    let mut me = get_start_position(&map);

    // turn into a vector of Vecs length 3, where vec[1] is "" or "123" and vec[2] is "L"/"R" or "" respectively
    let instructions = utils::regex_to_vec(instruction, &r"(?:([0-9]+)|([LR]))".to_string());

    for i in 0..instructions.len() {
        if instructions[i][1].len() == 0 {
            rotate_me(&mut me, instructions[i][2].chars().next().unwrap());
            //print_map(&map, &me);
        } else if instructions[i][2].len() == 0 {
            move_me2(&map, &mut me, instructions[i][1].parse().unwrap());
        }
    }

    // final value: right = 0, down = 1, left = 2, up = 3
    //  4(x+1) * 1000(y+1) + direction
    //println!("{:?}", me);

    println!("Day 22 part 2: {}", (me.x+1) *4 +  (me.y+1)*1000 + match me.facing {Dir::Right => 0, Dir::Down => 1, Dir::Left => 2, Dir::Up => 3});


}

fn print_map(map: &Vec<Vec<Tile>>, me: &Me) {
    for j in 0..map.len() {
        let mut row = String::new();
        for i in 0..map[0].len() {
            if me.x == i && me.y == j {
                row.push(match me.facing {Dir::Left => '<', Dir::Up => '^', Dir::Right => '>', Dir::Down => 'v'});
            } else {
                row.push(match map[j][i] {Tile::Off => ' ', Tile::Open => '.', Tile::Wall => '#'});
            }
        }
        println!("{}", row);
    }

    println!();
    //thread::sleep(time::Duration::from_secs(3));
}

fn test() {
    // test all places, i could also be wrong though
    // 1 is tail end, 2 is circle end, refer to paint diagram
    let top_of_b1 = (50,-1,Dir::Up);
    assert!(find_next_point(top_of_b1) == (0,150,Dir::Right));
    let top_of_b2 = (99,-1,Dir::Up);
    assert!(find_next_point(top_of_b2) == (0,199,Dir::Right));

    let top_of_r1 = (100,-1,Dir::Up);
    assert!(find_next_point(top_of_r1) == (0,199,Dir::Up));
    let top_of_r2 = (149,-1,Dir::Up);
    assert!(find_next_point(top_of_r2) == (49,199,Dir::Up));

    let right_of_r1 = (150,0,Dir::Right);
    assert!(find_next_point(right_of_r1) == (99,149,Dir::Left));
    let right_of_r2 = (150,49,Dir::Right);
    assert!(find_next_point(right_of_r2) == (99,100,Dir::Left));

    let bot_of_r1 = (100,50,Dir::Down);
    assert!(find_next_point(bot_of_r1) == (99,50,Dir::Left));
    let bot_of_r2 = (149,50,Dir::Down);
    assert!(find_next_point(bot_of_r2) == (99,99,Dir::Left));

    let right_of_u1 = (100,50,Dir::Right);
    assert!(find_next_point(right_of_u1) == (100,49,Dir::Up));
    let right_of_u2 = (100,99,Dir::Right);
    assert!(find_next_point(right_of_u2) == (149,49,Dir::Up));

    let right_of_t1 = (100,149,Dir::Right);
    assert!(find_next_point(right_of_t1) == (149,0,Dir::Left));
    let right_of_t2 = (100,100,Dir::Right);
    assert!(find_next_point(right_of_t2) == (149,49,Dir::Left));

    let bot_of_t1 = (50,150,Dir::Down);
    assert!(find_next_point(bot_of_t1) == (49,150,Dir::Left));
    let bot_of_t2 = (99,150,Dir::Down);
    assert!(find_next_point(bot_of_t2) == (49,199,Dir::Left));

    let right_of_d1 = (50,150,Dir::Right);
    assert!(find_next_point(right_of_d1) == (50,149,Dir::Up));
    let right_of_d2 = (50,199,Dir::Right);
    assert!(find_next_point(right_of_d2) == (99,149,Dir::Up));

    let bot_of_d1 = (0,200,Dir::Down);
    assert!(find_next_point(bot_of_d1) == (100,0,Dir::Down));
    let bot_of_d2 = (49,200,Dir::Down);
    assert!(find_next_point(bot_of_d2) == (149,0,Dir::Down));

    let left_of_d1 = (-1,150,Dir::Left);
    assert!(find_next_point(left_of_d1) == (50,0,Dir::Down));
    let left_of_d2 = (-1,199,Dir::Left);
    assert!(find_next_point(left_of_d2) == (99,0,Dir::Down));

    let left_of_l1 = (-1,100,Dir::Left);
    assert!(find_next_point(left_of_l1) == (50,49,Dir::Right));
    let left_of_l2 = (-1,149,Dir::Left);
    assert!(find_next_point(left_of_l2) == (50,0,Dir::Right));

    let top_of_l1 = (49,99,Dir::Up);
    assert!(find_next_point(top_of_l1) == (50,99,Dir::Right));
    let top_of_l2 = (0,99,Dir::Up);
    assert!(find_next_point(top_of_l2) == (50,50,Dir::Right));

    let left_of_u1 = (49,99,Dir::Left);
    assert!(find_next_point(left_of_u1) == (49,100,Dir::Down));
    let left_of_u2 = (49,50,Dir::Left);
    assert!(find_next_point(left_of_u2) == (0,100,Dir::Down));

    let left_of_b1 = (49,49,Dir::Left);
    assert!(find_next_point(left_of_b1) == (0,100,Dir::Right));
    let left_of_b2 = (49,0,Dir::Left);
    assert!(find_next_point(left_of_b2) == (0,149,Dir::Right));

}