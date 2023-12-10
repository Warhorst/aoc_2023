use std::fmt::Debug;

use crate::day_1::{solve_1a, solve_1b};
use crate::day_10::{solve_10a, solve_10b};
use crate::day_11::{solve_11a, solve_11b};
use crate::day_2::{solve_2a, solve_2b};
use crate::day_3::{solve_3a, solve_3b};
use crate::day_4::{solve_4a, solve_4b};
use crate::day_5::{solve_5a, solve_5b};
use crate::day_6::{solve_6a, solve_6b};
use crate::day_7::{solve_7a, solve_7b};
use crate::day_8::{solve_8a, solve_8b};
use crate::day_9::{solve_9a, solve_9b};
use crate::input::load_input;

mod input;
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod day_11;

fn main() {
    solve_day(11)
}

fn solve_day(day: usize) {
    let solve_day_funcs = [
        || solve(1, solve_1a, 142, solve_1b, 281),
        || solve(2, solve_2a, 8, solve_2b, 2286),
        || solve(3, solve_3a, 4361, solve_3b, 467835),
        || solve(4, solve_4a, 13, solve_4b, 30),
        || solve(5, solve_5a, 35, solve_5b, 46),
        || solve(6, solve_6a, 288, solve_6b, 71503),
        || solve(7, solve_7a, 6440, solve_7b, 5905),
        || solve(8, solve_8a, 6, solve_8b, 6),
        || solve(9, solve_9a, 114, solve_9b, 2),
        || solve(10, solve_10a, 8, solve_10b, 10),
        || solve(11, solve_11a, 0, solve_11b, 0),
    ];

    solve_day_funcs[day - 1]()
}

fn solve<A: Debug + PartialEq, B: Debug + PartialEq, AS: Fn(&str) -> A, BS: Fn(&str) -> B>(
    day: u8,
    a_solver: AS,
    a_example_solution: A,
    b_solver: BS,
    b_example_solution: B,
) {
    println!("Solving day {day}");

    let input = load_input(day);

    if input.example_a.is_empty() {
        println!("example a does not exist yet, skipping it");
    } else {
        assert_eq!(a_example_solution, a_solver(&input.example_a));
        println!("Example a works");
    }

    if input.example_b.is_empty() {
        println!("example b does not exist yet, skipping it");
    } else {
        assert_eq!(b_example_solution, b_solver(&input.example_b));
        println!("Example b works");
    }

    if input.puzzle_input.is_empty() {
        println!("the puzzle input does not exist yet, skipping it");
    } else {
        println!("Solution a: {:?}", a_solver(&input.puzzle_input));
        println!("Solution b: {:?}", b_solver(&input.puzzle_input));
    }
}
