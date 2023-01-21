use super::utils;
use super::utils::Print::*;

pub fn run(print : utils::Print ) {
    let day : &str = "8";

    if day == "0" {
        panic!("CHANGE THE DAY");
    }

    match print {
        Part1 => part1(day),
        Part2 => part2(day),
        BothParts => { part1(day); part2(day); }
        NoParts => (),
    }
}

fn part1(day: &str) {

    let mut rows = Vec::new();
    
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        
        rows.push(line.to_string());

    }

    let mut total_visible = 0;
    for i in 0..rows.get(0).unwrap().len() {
        for j in 0..rows.len() {
            if is_visible(&rows, i, j) {
                total_visible += 1;
            }
        }
    }

    println!("Day 8 part 1: {total_visible}");
}

fn is_visible(rows: &Vec<String>, x: usize, y: usize) -> bool {
    let height = rows.get(y).unwrap().chars().nth(x).unwrap().to_string().parse().unwrap();
    let row_length = rows.get(y).unwrap().len();
    let column_length = rows.len();
    // check left
    let mut is_left_visible = true;
    for i in 0..x {
        if get_char(&rows, i, y) >= height {
            is_left_visible = false;
            break;
        }
    }
    if is_left_visible {
        return true;
    }

    // check right
    let mut is_right_visible = true;
    for i in x+1..row_length {
        if get_char(&rows, i, y) >= height {
            is_right_visible = false;
            break;
        }
    }
    if is_right_visible {
        return true;
    }

    // check up
    let mut is_up_visible = true;
    for j in 0..y {
        if get_char(&rows, x, j) >= height {
            is_up_visible = false;
            break;
        }
    }
    if is_up_visible {
        return true;
    }

    // check down
    let mut is_down_visible = true;
    for j in y+1..column_length {
        if get_char(&rows, x, j) >= height {
            is_down_visible = false;
            break;
        }
    }
    if is_down_visible {
        return true;
    }

    false
}

fn get_char(rows: &Vec<String>, x: usize, y: usize) -> u32 {
    rows.get(y).unwrap().chars().nth(x).unwrap().to_string().parse().unwrap()
}

fn part2(day : &str) {
    
    let mut rows = Vec::new();
    
    for line in utils::read_lines(day) {
        let line = line.unwrap();
        
        rows.push(line.to_string());

    }

    let mut max_scenic = 0;
    for i in 0..rows.get(0).unwrap().len() {
        for j in 0..rows.len() {
            let scenic = check_scenic(&rows, i, j);
            if scenic > max_scenic {
                max_scenic = scenic;
            }
        }
    }

    println!("Day 8 part 2: {max_scenic}");
}

fn check_scenic(rows: &Vec<String>, x: usize, y: usize) -> u32 {
    let height = rows.get(y).unwrap().chars().nth(x).unwrap().to_string().parse().unwrap();
    let row_length = rows.get(y).unwrap().len();
    let column_length = rows.len();

    // anything on the border is immediate 0
    if x == 0 || y == 0 || x == row_length-1 || y == column_length - 1 {
        return 0
    }

    // check left
    let mut visible_left = 0;
    for i in (0..x).rev() {
        let current_height = get_char(&rows, i, y);
        visible_left += 1;
        if current_height >= height {
            break;
        }
    }

    // check right
    let mut visible_right = 0;
    for i in x+1..row_length {
        let current_height = get_char(&rows, i, y);
        visible_right += 1;
        if current_height >= height {
            break;
        }
    }

    // check up
    let mut visible_up = 0;
    for j in (0..y).rev() {
        let current_height = get_char(&rows, x, j);
        visible_up += 1;
        if current_height >= height {
            break;
        }
    }

    // check down
    let mut visible_down = 0;
    for j in y+1..column_length {
        let current_height = get_char(&rows, x, j);
        visible_down += 1;
        if current_height >= height {
            break;
        }
    }

    visible_up * visible_down * visible_left * visible_right
}

#[allow(dead_code)]
struct Tree {
    left: usize,
    right: usize,
    up: usize,
    down: usize,
}

#[allow(dead_code)]
impl Tree {
    fn get_scenic(&self) -> usize {
        self.left * self.right * self.up * self.down
    }
}

/**
 * We can calculate this in one pass
 * 
 * In one direction we can keep track of "how many trees we can see if we were of size x"
 * In the other direction we can keep track of "where was the last tree of size x"
 * 
 * Doing this in 2 directions (left->right and up->down) gives us all 4 direction values so we can calculate the scenic for all trees in one pass
 * 
 * Left->Right
 * After each tree, we keep track of what the next tree can see if it was a certain height
 * Suppose a height of y is next.
 * Then we can take the value corresponding to what trees of height y can see.
 * Then we set all values of y and below to 1, as trees of height y and below cannot see over y
 * Then we set all values above y to increment by 1, as they can see over y and anything beyond
 * 
 * Right->Left
 * After each tree, for each height we keep track of how many trees ago was the last tree of that size
 * When we encounter a tree of height x, any tree of height x and below is terminated and must only be able to see our current tree and in between
 * So for every height x and below, we check if we last encountered a tree of that size and go to that tree and populate it
 * Then we reset these smaller trees back to 0
 * We set the tree of our height to 1 (as we will now be 1 step away from our tree of height x)
 * Then we increment any taller values, as they can still see beyond our tree
 */
#[allow(dead_code)]
fn more_efficient() {
    let mut can_see_up: Vec<[usize;10]> = Vec::new(); // a vector of "arrays of 10 zeroes", we'll need one array per column of data
    let mut last_tree_of_height_down : Vec<[usize;10]> = Vec::new(); // last_tree[col][height]

    let mut trees : Vec<Vec<Tree>> = Vec::new(); // trees[row][col]

    let mut row = 0;
    let mut max_scenic = 0;

    let mut row_len = 0;

    for line in utils::read_lines("8") { // replace this with whatever you're using to read the input
        let line = line.unwrap();
        row_len = line.len();

        // first row setup
        if can_see_up.len() == 0 {
            for _ in 0..line.len() {
                can_see_up.push([0;10]);
                last_tree_of_height_down.push([0;10]);
            }
        }

        // new row setup
        let mut can_see_left = [0; 10];
        let mut last_tree_of_height_right = [0; 10];
        trees.push(Vec::new());

        for i in 0..line.len() {
            let height: usize = line.chars().nth(i).unwrap().to_string().parse().unwrap();

            // left
            let left = can_see_left[height];
            for j in 0..=height {
                can_see_left[j] = 1;
            }
            for j in height+1..10 {
                can_see_left[j] += 1;
            }

            // up
            let up = can_see_up[i][height];
            for j in 0..=height {
                can_see_up[i][j] = 1;
            }
            for j in height+1..10 {
                can_see_up[i][j] += 1;
            }

            // can populate tree now as right and down can only be done for future trees
            trees[row].push(Tree{left, up, right: 0, down: 0});

            // right
            for h in 0..=height {
                let last_tree_of_height = last_tree_of_height_right[h];
                if last_tree_of_height != 0 {
                    trees[row][i-last_tree_of_height].right = last_tree_of_height;
                }
                last_tree_of_height_right[h] = 0;
            }
            for h in height+1..10 {
                let last_tree_of_height = last_tree_of_height_right[h];
                if last_tree_of_height != 0 {
                    last_tree_of_height_right[h] += 1;
                }
            }
            last_tree_of_height_right[height] = 1;

            // down
            for h in 0..=height {
                let last_tree_of_height = last_tree_of_height_down[i][h];
                if last_tree_of_height != 0 {
                    trees[row-last_tree_of_height][i].down = last_tree_of_height;

                    // down is guaranteed to be last due to the nature we read the data, so we can calculate the scenic here
                    let scenic = trees[row-last_tree_of_height][i].get_scenic();
                    if scenic > max_scenic {
                        max_scenic = scenic;
                    }
                }
                last_tree_of_height_down[i][h] = 0;
            }
            for h in height+1..10 {
                let last_tree_of_height = last_tree_of_height_down[i][h];
                if last_tree_of_height != 0 {
                    last_tree_of_height_down[i][h] += 1;
                }
            }
            last_tree_of_height_down[i][height] = 1;
        }
        
        // end of line, need to evaluate the remaining rights
        for h in 0..10 {
            let last_tree_of_height = last_tree_of_height_right[h];
            if last_tree_of_height != 0 {
                trees[row][line.len()-last_tree_of_height].right = last_tree_of_height-1; // overcounted 1
            }
        }

        row += 1;
    }

    // end of rows, need to evaluate the remaining downs
    for c in 0..row_len {
        for h in 0..10 {
            let last_tree_of_height = last_tree_of_height_down[c][h];
            if last_tree_of_height != 0 {
                trees[row-last_tree_of_height][c].down = last_tree_of_height - 1; // overcounted 1
                let scenic = trees[row-last_tree_of_height][c].get_scenic();
                if scenic > max_scenic {
                    max_scenic = scenic;
                }
            }
        }
    }

    println!("Test: {}", max_scenic);
}