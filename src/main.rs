mod utils;
use utils::Print::*;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

mod test;

fn main() {
    utils::utils_test();

    day1::run(NoParts);
    day2::run(NoParts);
    day3::run(NoParts);
    day4::run(NoParts);
    day5::run(NoParts);
    day6::run(NoParts);
    day7::run(NoParts);
    day8::run(NoParts);
    day9::run(NoParts);
    day10::run(NoParts);
    day11::run(NoParts);
    day12::run(NoParts);
    day13::run(NoParts);
    day14::run(NoParts);
    day15::run(NoParts);
    day16::run(NoParts); // note this takes a substantial amount of time ~ 30min for part 2
    day17::run(NoParts); // note this takes a substantial amount of time ~ 2min for part 2
    day18::run(NoParts); 
    day19::run(NoParts); // note this takes a substantial amount of time ~ 10sec for part 1, 3min for part 2, unless you use pat strat then its just seconds
    day20::run(NoParts); 
    day21::run(NoParts); 
    day22::run(NoParts); 
    day23::run(NoParts); 
    day24::run(NoParts); 
    day25::run(BothParts); 

    //test::test();
}