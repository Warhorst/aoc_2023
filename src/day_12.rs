use std::cell::RefCell;
use std::collections::HashSet;
use itertools::Itertools;
use crate::day_12::Spring::{Damaged, Operational, Unknown};
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;

pub fn solve_12a(input: &str) -> usize {
    let rows = input
        .lines()
        .map(Row::from)
        .collect::<Vec<_>>();

    // rows.iter().for_each(|r| println!("{:?}", r));
    // println!("{}", springs_matches_nums(&vec![Damaged, Operational, Damaged, Operational, Damaged, Damaged, Damaged], &vec![1, 1, 3]));

    rows
        .into_iter()
        .map(|r| r.get_count_resolved_rows_new())
        .sum()
}

pub fn solve_12b(input: &str) -> usize {
    0
}

#[derive(Clone, Debug)]
struct Row {
    springs: Vec<Spring>,
    nums: Vec<usize>
}

impl Row {
    // idea: Create sliding windows depending on the numbers of broken
    // springs, each with one tile distance. Move the windows along the
    // row, replace ? with broken springs where possible and count
    // every occurrence of valid rows.
    fn get_count_resolved_rows_new(&self) -> usize {
        let mut windows = self.create_windows();

        // println!("{:?}", windows);
        //
        // for w in windows {
        //     println!("{:?}", self.get_springs_at_window(w))
        // }

        for i in (0..windows.len()).rev() {
            let mut w = windows[i];

            while self.window_in_row(*w) && self.get_springs_at_window(*w).into_iter().any(|s| s == Operational) {
                w.0 += 1
            }
        }

        0
    }

    // Vec<(index, size)>
    fn create_windows(&self) -> Vec<(usize, usize)> {
        let mut result = vec![];
        let mut current_index = 0;

        for i in 0..self.nums.len() {
            result.push((current_index, self.nums[i]));

            current_index += 1;
            current_index += self.nums[i];
        }

        result
    }

    fn window_in_row(&self, (index, size): (usize, usize)) -> bool {
        index + size < self.springs.len()
    }

    fn get_springs_at_window(&self, (index, size): (usize, usize)) -> impl IntoIterator<Item=Spring> + '_ {
        self.springs.iter().copied().skip(index).take(size)
    }

    fn get_count_resolved_rows(&self) -> usize {
        let num_currently_damaged_springs = self.springs.iter().filter(|s| **s == Damaged).count();
        let num_total_damaged_springs = self.nums.iter().sum::<usize>();
        let num_unknown_springs = self.springs.iter().filter(|s| **s == Unknown).count();
        let diff = num_total_damaged_springs - num_currently_damaged_springs;

        let mut base = vec![];

        for _ in 0..diff {
            base.push(Damaged)
        }

        for _ in 0..(num_unknown_springs - diff) {
            base.push(Operational)
        }

        let base_len = base.len();
        let mut visited = RefCell::new(HashSet::new());

        base
            .into_iter()
            .permutations(base_len)
            .filter(|p| !visited.borrow().contains(p))
            .map(|p| {
                // sometimes, rust sucks
                visited.replace({
                    let mut tmp = visited.borrow().clone();
                    tmp.insert(p.clone());
                    tmp
                });
                self.create_resolved_row(p)
            })
            .filter(|r| r.is_valid())
            .count()
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
                Operational =>  {
                    if on_block {
                        block_sizes.push(current_block_count);
                        current_block_count = 0;
                    }

                    on_block = false
                },
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
            springs, nums
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown
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