use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;

use crate::day_12::Spring::{Damaged, Operational, Unknown};

pub fn solve_12a(input: &str) -> usize {
    let rows = input
        .lines()
        .map(Row::from)
        .collect::<Vec<_>>();

    rows
        .into_par_iter()
        .map(|r| r.get_count_resolved_rows())
        .sum()
}

pub fn solve_12b(input: &str) -> usize {
    0
}

#[derive(Clone, Debug)]
struct Row {
    springs: Vec<Spring>,
    nums: Vec<usize>,
}

impl Row {
    fn get_count_resolved_rows(&self) -> usize {
        let num_unknown_springs = self.springs.iter().filter(|s| **s == Unknown).count();

        Self::create_bit_representations(num_unknown_springs)
            .into_iter()
            .map(Self::bits_to_springs)
            .map(|springs| self.create_resolved_row(springs))
            .filter(|row| row.is_valid())
            .count()
    }

    fn create_bit_representations(num_unknown_springs: usize) -> impl IntoIterator<Item=Vec<u32>> {
        (0..2_u32.pow(num_unknown_springs as u32))
            .into_iter()
            .map(move |x| (0..num_unknown_springs)
                .into_iter()
                .map(|n| (x >> n) & 1)
                .collect::<Vec<_>>())
    }

    fn bits_to_springs(bits: Vec<u32>) -> Vec<Spring> {
        bits
            .into_iter()
            .map(|bit| if bit == 0 {
                Damaged
            } else {
                Operational
            })
            .collect()
    }

    fn create_resolved_row(&self, resolves: Vec<Spring>) -> Row {
        let mut new = self.clone();
        let mut iter = resolves.into_iter();
        new.springs.iter_mut().for_each(|s| if *s == Unknown {
            *s = iter.next().unwrap()
        });

        new
    }

    fn is_valid(&self) -> bool {
        let mut on_block = false;
        let mut current_block_count = 0;
        let mut block_sizes = vec![];

        for i in 0..self.springs.len() {
            match self.springs[i] {
                Operational => {
                    if on_block {
                        block_sizes.push(current_block_count);
                        current_block_count = 0;
                    }

                    on_block = false
                }
                Damaged => {
                    on_block = true;
                    current_block_count += 1;
                }
                Unknown => panic!("should not be unknown at this point")
            }
        }

        if on_block {
            block_sizes.push(current_block_count);
        }

        block_sizes == self.nums
    }
}

impl From<&str> for Row {
    fn from(s: &str) -> Self {
        let split = s.split(" ").collect::<Vec<_>>();

        let springs = split[0].chars().map(Spring::from).collect();
        let nums = split[1].split(",").map(|s| s.parse::<usize>().unwrap()).collect();

        Row {
            springs,
            nums,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Spring {
    fn from(c: char) -> Self {
        match c {
            '.' => Operational,
            '#' => Damaged,
            '?' => Unknown,
            _ => panic!("unknown char")
        }
    }
}