#![allow(unused)]

use super::utils;

struct Tree {
    left: usize,
    right: usize,
    up: usize,
    down: usize,
}

impl Tree {
    fn get_scenic(&self) -> usize {
        self.left * self.right * self.up * self.down
    }
}

pub fn test() {

    let mut can_see_up: Vec<[usize;10]> = Vec::new(); // a vector of "arrays of 10 zeroes", we'll need one array per column of data
    let mut last_tree_of_height_down : Vec<[usize;10]> = Vec::new(); // [col][height]

    let mut trees : Vec<Vec<Tree>> = Vec::new(); // [row][col]

    let mut row = 0;
    let mut max_scenic = 0;

    let mut row_len = 0;

    for line in utils::read_lines("8") {
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