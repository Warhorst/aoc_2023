use std::fs::read_to_string;

pub struct Input {
    pub puzzle_input: String,
    pub example_a: String,
    pub example_b: String
}

pub fn load_input(day: u8) -> Input {
    let puzzle_input = read_to_string(format!("./input/{day}.txt")).unwrap_or(String::default());
    let example_a = read_to_string(format!("./input/{day}a.txt")).unwrap_or(String::default());
    let example_b = read_to_string(format!("./input/{day}b.txt")).unwrap_or(String::default());

    Input {
        puzzle_input,
        example_a,
        example_b
    }
}