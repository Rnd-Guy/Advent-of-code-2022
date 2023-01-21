use super::utils;
use super::utils::Print::*;

/**
 * Day 2:
 * Data: [ABC] [XYZ]
 * A = Rock, B = Paper, C = Scissors
 * X = Rock, Y = Paper, Z = Scissors
 * Rock is worth 1, Paper 2, Scissors 3 when you pick it (ie X = 1, Y = 2, Z = 3)
 * Win = 6, draw = 3, lose = 0
 * 
 * Part 1: Opponent chooses ABC, you choose XYZ
 * What is your total score?
 * 
 * Part 2: This time, X = Lose, Y = Draw, Z = Win
 * What is your total score?
 */
pub fn run(print : utils::Print) {
    const DAY: &str = "2";

    let mut score = 0;
    for line in utils::read_lines(DAY) {
        let line = line.unwrap();
        
        if line.ends_with('X') {                            // i pick rock
            score += 1;

            if      line.starts_with("A") { score += 3 }  // they pick rock
            else if line.starts_with("B") { score += 0 }  // they pick paper
            else if line.starts_with("C") { score += 6 }  // they pick scissors

        } else if line.ends_with("Y") {                     // i pick paper
            score += 2;

            if      line.starts_with("A") { score += 6 }  // they pick rock
            else if line.starts_with("B") { score += 3 }  // they pick paper
            else if line.starts_with("C") { score += 0 }  // they pick scissors

        } else if line.ends_with("Z") {                     // i pick scissors
            score += 3;

            if      line.starts_with("A") { score += 0 }  // they pick rock
            else if line.starts_with("B") { score += 6 }  // they pick paper
            else if line.starts_with("C") { score += 3 }  // they pick scissors
        }


    }

    match print {
        BothParts | Part1 => println!("{}", score),
        _ => (),
    }

    let mut score2 = 0;
    for line in utils::read_lines(DAY) {
        let line = line.unwrap();
        
        // remember rock is 1, paper is 2, scissors is 3
        if line.ends_with('X') {                            // i lose
            score2 += 0;

            if      line.starts_with("A") { score2 += 3 }  // they pick rock then i pick scissors
            else if line.starts_with("B") { score2 += 1 }  // they pick paper then i pick rock
            else if line.starts_with("C") { score2 += 2 }  // they pick scissors then i pick paper

        } else if line.ends_with("Y") {                     // i draw
            score2 += 3;

            if      line.starts_with("A") { score2 += 1 }  // they pick rock and so do i
            else if line.starts_with("B") { score2 += 2 }  // they pick paper and so do i
            else if line.starts_with("C") { score2 += 3 }  // they pick scissors and so do i

        } else if line.ends_with("Z") {                     // i win
            score2 += 6;

            if      line.starts_with("A") { score2 += 2 }  // they pick rock then i pick paper
            else if line.starts_with("B") { score2 += 3 }  // they pick paper then i pick scissors
            else if line.starts_with("C") { score2 += 1 }  // they pick scissors then i pick rock
        }

    }

    match print {
        BothParts | Part2 => println!("{}", score2),
        _ => (),
    }
    
}