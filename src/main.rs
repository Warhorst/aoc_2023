use std::fmt::Debug;

use crate::day_1::{solve_1a, solve_1b};
use crate::day_2::{solve_2a, solve_2b};
use crate::day_3::{solve_3a, solve_3b};
use crate::input::load_input;

mod input;
mod day_1;
mod day_2;
mod day_3;

fn main() {
    solve_day(3)
}

fn solve_day(day: usize) {
    let solve_day_funcs = [
        || solve(1, solve_1a, 142, solve_1b, 281),
        || solve(2, solve_2a, 8, solve_2b, 2286),
        || solve(3, solve_3a, 4361, solve_3b, 467835),
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
