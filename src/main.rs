use std::fmt::Debug;
use crate::day_1::{solve_1a, solve_1b};
use crate::input::load_input;

mod day_1;
mod input;

fn main() {
    solve(
        1,
        solve_1a,
        142,
        solve_1b,
        281
    )
}

fn solve<A: Debug + PartialEq, B: Debug + PartialEq, AS: Fn(&str) -> A, BS: Fn(&str) -> B>(
    day: u8,
    a_solver: AS,
    a_example_solution: A,
    b_solver: BS,
    b_example_solution: B
) {
    let input = load_input(day);

    if input.example_a.is_empty() {
        println!("example a does not exist yet, skipping it");
    } else {
        assert_eq!(a_example_solution, a_solver(&input.example_a))
    }

    if input.example_b.is_empty() {
        println!("example b does not exist yet, skipping it");
    } else {
        assert_eq!(b_example_solution, b_solver(&input.example_b))
    }

    if input.puzzle_input.is_empty() {
        println!("the puzzle input does not exist yet, skipping it");
    } else {
        println!("Solution a: {:?}", a_solver(&input.puzzle_input));
        println!("Solution b: {:?}", b_solver(&input.puzzle_input));
    }
}
