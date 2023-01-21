use std::thread::current;

use super::utils;
use super::utils::Print::*;

static DIRECTIONS: [Dir; 4] = [Dir::Left, Dir::Right, Dir::Up, Dir::Down];
/**
 * Data: heightmap, a low z high
 * Other symbols: S = current position (a), E best signal (z)
 */
pub fn run(print : utils::Print ) {
    let day : &str = "12";

    assert!(day != "0", "CHANGE THE DAY");

    quick_tests();

    match print {
        Part1 => part1(day),
        Part2 => part2(day),
        BothParts => { part1(day); part2(day); }
        NoParts => (),
    }
}

// part 1:
// can only go 1 elevation at a time, find length of shortest path from S to E
fn part1(day: &str) {

    let (height_map, s, e) = read_data(day);
    println!("start: ({},{})  end: ({},{})", s.0, s.1, e.0, e.1);




    //let path_length = find_path(&height_map, &traversed_path, s, e, 0);
    let path_length = find_path_by_current_distance(&height_map, s, e);

    println!("Day 12 part 1: {}", path_length);
    // find the E and the S

}

// fn find_path(height_map: &Vec<Vec<char>>, traversed_path: &Vec<Vec<i32>>, current_pos: (usize, usize), end_pos: (usize, usize), current_distance: i32) -> i32{

//     // check if we found the exit
//     traversed_path[current_pos.1][current_pos.0] = current_distance;
//     if current_pos == end_pos {
//         return current_distance;
//     }

//     // check if we have already found the exit
//     if traversed_path[end_pos.1][end_pos.0] >= 0 {
//         return traversed_path[end_pos.1][end_pos.0];
//     }

//     let height = height_map.len();
//     let width = height_map[0].len();

//     assert!(current_pos.0 < width, "current position is off the map to the right");
//     assert!(current_pos.1 < height, "current position is off the map to the bottom");

//     let mut new_positions: Vec<(usize, usize)> = Vec::new();
//     let mut found_end = false;
//     for dir in &DIRECTIONS {
//         if is_traversable(height_map, traversed_path, current_pos, dir) {
//             let new_pos = get_new_pos(height_map, current_pos, dir).unwrap();
//             new_positions.push(new_pos);

//             if new_pos == end_pos {
//                 found_end = true;
//             }
//         }
//     }

//     if found_end {
//         //let distance = get_length_of_path(traversed_path) + 1;
//         return current_distance + 1;
//     }

//     let mut new_path_lengths: Vec<u32> = Vec::new();
//     new_path_lengths.push(u32::MAX); // guarantee we have at least one path length
//     for pos in new_positions {
//         let mut new_path = make_path_copy(&traversed_path);
//         new_path[pos.1][pos.0] = true;
//         new_path_lengths.push(find_path(height_map, &new_path, pos, end_pos, shortest_distance));
//     }

//     return *new_path_lengths.iter().min().unwrap();


// }

fn find_path_by_current_distance(height_map: &Vec<Vec<char>>, start_pos: (usize, usize), end_pos: (usize, usize)) -> i32{

    // // check if we have already found the exit
    // if traversed_path[end_pos.1][end_pos.0] >= 0 {
    //     return traversed_path[end_pos.1][end_pos.0];
    // }
    let height = height_map.len();
    let width = height_map[0].len();

    let mut traversed_path: Vec<Vec<i32>> = Vec::new();
    for i in 0..height {
        traversed_path.push(Vec::new());
        for _ in 0..width {
            traversed_path[i].push(-1);
        }
    }
    traversed_path[start_pos.1][start_pos.0] = 0;

    assert!(height_map.len() == traversed_path.len());
    for i in 0..height_map.len() {
        assert!(height_map[i].len() == traversed_path[i].len())
    }

    let height = height_map.len();
    let width = height_map[0].len();

    let mut current_distance = 0;
    while traversed_path[end_pos.1][end_pos.0] == -1 {
        let mut current_positions: Vec<(usize, usize)> = Vec::new();
        for i in 0..height {
            for j in 0..width {
                if traversed_path[i][j] == current_distance {
                    current_positions.push((j,i));
                }
            }
        }

        if current_positions.len() == 0 {
            return -1;
        }

        for current_pos in current_positions {
            for dir in &DIRECTIONS {
                if is_traversable(height_map, &traversed_path, current_pos, dir) {
                    let new_pos = get_new_pos(height_map, current_pos, dir).unwrap();
                    traversed_path[new_pos.1][new_pos.0] = current_distance + 1;
                }
            }
        }

        current_distance += 1;
    }

    return current_distance;


}

fn find_path_by_current_distance2(height_map: &Vec<Vec<char>>, end_pos: (usize, usize)) -> i32{

    // // check if we have already found the exit
    // if traversed_path[end_pos.1][end_pos.0] >= 0 {
    //     return traversed_path[end_pos.1][end_pos.0];
    // }
    let height = height_map.len();
    let width = height_map[0].len();

    let mut traversed_path: Vec<Vec<i32>> = Vec::new();
    for i in 0..height {
        traversed_path.push(Vec::new());
        for _ in 0..width {
            traversed_path[i].push(-1);
        }
    }

    for i in 0..height {
        for j in 0..width {
            if height_map[i][j] == 'a' {

                traversed_path[i][j] = 0;
            }
        }
    }

    assert!(height_map.len() == traversed_path.len());
    for i in 0..height_map.len() {
        assert!(height_map[i].len() == traversed_path[i].len())
    }

    let height = height_map.len();
    let width = height_map[0].len();

    let mut current_distance = 0;
    while traversed_path[end_pos.1][end_pos.0] == -1 {
        let mut current_positions: Vec<(usize, usize)> = Vec::new();
        for i in 0..height {
            for j in 0..width {
                if traversed_path[i][j] == current_distance {
                    current_positions.push((j,i));
                }
            }
        }

        if current_positions.len() == 0 {
            return -1;
        }

        for current_pos in current_positions {
            for dir in &DIRECTIONS {
                if is_traversable(height_map, &traversed_path, current_pos, dir) {
                    let new_pos = get_new_pos(height_map, current_pos, dir).unwrap();
                    traversed_path[new_pos.1][new_pos.0] = current_distance + 1;
                }
            }
        }

        current_distance += 1;
    }

    return current_distance;


}



// fn make_path_copy(traversed_path: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
//     let mut new_path = Vec::new();

//     for i in 0..traversed_path.len() {
//         new_path.push(Vec::new());
//         for j in 0..traversed_path[0].len() {
//             new_path[i].push(traversed_path[i][j]);
//         }
//     }

//     assert!(new_path.len() == traversed_path.len());
//     for i in 0..traversed_path.len() {
//         assert!(new_path[i].len() == traversed_path[i].len())
//     }

//     new_path
// }

// fn get_length_of_path(traversed_path: &Vec<Vec<bool>>) -> u32 {
//     let mut sum = 0;
//     for i in 0..traversed_path.len() {
//         for j in 0..traversed_path[0].len() {
//             if traversed_path[i][j] {
//                 sum += 1;
//             }
//         }
//     }
//     sum
// }

fn get_height(c: char) -> u32 {
    if c == 'S' {
        0
    } else if c == 'E' {
        25
    } else {
        utils::convert_ascii_to_index(c)
    }
}

enum Dir {
    Left,
    Right,
    Up,
    Down
}

fn is_traversable(height_map: &Vec<Vec<char>>, traversed_path: &Vec<Vec<i32>>, current_pos: (usize, usize), direction: &Dir) -> bool {
    
    let new_pos = get_new_pos(height_map, current_pos, direction);
    
    let new_pos = match new_pos {
        None => return false,
        Some(pos) => pos
    };

    assert!(new_pos.0 < height_map[0].len());
    assert!(new_pos.1 < height_map.len());

    // dont go to previously traversed path
    if traversed_path[new_pos.1][new_pos.0] >= 0 {
        return false;
    }

    // needs to be within 1
    let current_height = get_height(height_map[current_pos.1][current_pos.0]);
    let dest_height = get_height(height_map[new_pos.1][new_pos.0]);

    if dest_height <= current_height + 1 {
        return true
    } else {
        return false
    }

}

fn get_new_pos(height_map: &Vec<Vec<char>>, current_pos: (usize, usize), direction: &Dir) -> Option<(usize, usize)> {
    let height = height_map.len();
    let width = height_map[0].len();

    match direction {
        Dir::Left => {
            if current_pos.0 == 0 {
                None
            } else {
                Some((current_pos.0 - 1, current_pos.1))
            }
        },

        Dir::Right => {
            if current_pos.0 + 1 >= width {
                None
            } else {
                Some((current_pos.0 + 1, current_pos.1))
            }
        },

        Dir::Up => {
            if current_pos.1 == 0 {
                None
            } else {
                Some((current_pos.0, current_pos.1 - 1))
            }
        },

        Dir::Down => {
            if current_pos.1 + 1 >= height {
                None
            } else {
                Some((current_pos.0, current_pos.1 + 1))
            }
        }
    }
}

fn read_data(day: &str) -> (Vec<Vec<char>>, (usize, usize), (usize, usize)) {
    // store data
    let mut height_map: Vec<Vec<char>> = Vec::new();

    // find the E and the S
    let mut s: (usize, usize) = (0,0);
    let mut e: (usize, usize) = (0,0);

    let mut row = 0;
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        
        height_map.push(Vec::new());

        let mut col = 0;
        for c in line.chars() {
            height_map.last_mut().unwrap().push(c);
            if c == 'E' {
                e = (col, row);
            } else if c == 'S' {
                s = (col, row);
            }
            col += 1;
        }

        row += 1;
    }
    (height_map, s, e)
}

fn part2(day : &str) {
    
    let (height_map, s, e) = read_data(day);
    println!("start: ({},{})  end: ({},{})", s.0, s.1, e.0, e.1);




    //let path_length = find_path(&height_map, &traversed_path, s, e, 0);
    let path_length = find_path_by_current_distance2(&height_map, e);

    println!("Day 12 part 2: {}", path_length);
}

// some asserts to make sure my code works
fn quick_tests() {

    assert!(get_height('a') == 0);
    assert!(get_height('b') == 1);
    assert!(get_height('c') == 2);
    assert!(get_height('d') == 3);
    assert!(get_height('e') == 4);
    assert!(get_height('f') == 5);
    assert!(get_height('z') == 25);
    assert!(get_height('S') == 0);
    assert!(get_height('E') == 25);

    let fake_map = vec![vec!['S','b','c'],vec!['f','e','d'],vec!['g','h','i']];
    assert!(get_new_pos(&fake_map, (0,0), &Dir::Up) == None);
    assert!(get_new_pos(&fake_map, (0,0), &Dir::Down) == Some((0,1)));
    assert!(get_new_pos(&fake_map, (0,0), &Dir::Left) == None);
    assert!(get_new_pos(&fake_map, (0,0), &Dir::Right) == Some((1,0)));

    assert!(get_new_pos(&fake_map, (2,2), &Dir::Up) == Some((2,1)));
    assert!(get_new_pos(&fake_map, (2,2), &Dir::Down) == None);
    assert!(get_new_pos(&fake_map, (2,2), &Dir::Left) == Some((1,2)));
    assert!(get_new_pos(&fake_map, (2,2), &Dir::Right) == None);

    let fake_path = vec![vec![0,-1,-1],vec![-1,-1,-1],vec![-1,-1,-1]];
    assert!(is_traversable(&fake_map, &fake_path, (0,0), &Dir::Up) == false);
    assert!(is_traversable(&fake_map, &fake_path, (0,0), &Dir::Down) == false);
    assert!(is_traversable(&fake_map, &fake_path, (0,0), &Dir::Left) == false);
    assert!(is_traversable(&fake_map, &fake_path, (0,0), &Dir::Right) == true);

    assert!(is_traversable(&fake_map, &fake_path, (2,2), &Dir::Up) == true);
    assert!(is_traversable(&fake_map, &fake_path, (2,2), &Dir::Down) == false);
    assert!(is_traversable(&fake_map, &fake_path, (2,2), &Dir::Left) == true);
    assert!(is_traversable(&fake_map, &fake_path, (2,2), &Dir::Right) == false);

    let fake_path2 = vec![vec![2,2,2],vec![3,3,3],vec![2,2,2]];
    assert!(is_traversable(&fake_map, &fake_path2, (0,0), &Dir::Up) == false);
    assert!(is_traversable(&fake_map, &fake_path2, (0,0), &Dir::Down) == false);
    assert!(is_traversable(&fake_map, &fake_path2, (0,0), &Dir::Left) == false);
    assert!(is_traversable(&fake_map, &fake_path2, (0,0), &Dir::Right) == false);

    assert!(is_traversable(&fake_map, &fake_path2, (2,2), &Dir::Up) == false);
    assert!(is_traversable(&fake_map, &fake_path2, (2,2), &Dir::Down) == false);
    assert!(is_traversable(&fake_map, &fake_path2, (2,2), &Dir::Left) == false);
    assert!(is_traversable(&fake_map, &fake_path2, (2,2), &Dir::Right) == false);

    //assert!(find_path_by_current_distance(height_map, traversed_path, start_pos, end_pos))
}