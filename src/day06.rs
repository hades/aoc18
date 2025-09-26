use std::{collections::HashSet, iter::repeat_n};

use itertools::Itertools;
use log::debug;

use crate::solver::Solver;

#[derive(Default)]
pub struct PuzzleSolver {
    input: Vec<(i64, i64)>,
}

impl Solver for PuzzleSolver {
    fn presolve(&mut self, input: &str) {
        self.input = input
            .trim()
            .lines()
            .map(|line| {
                let (x, y) = line.split_once(", ").unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .collect();
    }

    fn solve_part_one(&mut self) -> String {
        let (mut left, mut right, mut top, mut bottom) = (
            self.input[0].0,
            self.input[0].0,
            self.input[0].1,
            self.input[0].1,
        );
        for (x, y) in self.input.iter().skip(1) {
            left = std::cmp::min(left, *x);
            right = std::cmp::max(right, *x);
            top = std::cmp::min(top, *y);
            bottom = std::cmp::max(bottom, *y);
        }
        let mut areas_by_idx: Vec<_> = repeat_n(0usize, self.input.len()).collect();
        for x in left..=right {
            for y in top..=bottom {
                if let Ok((closest_idx, _)) = self
                    .input
                    .iter()
                    .enumerate()
                    .min_set_by_key(|(_, (coord_x, coord_y))| {
                        (x - *coord_x).abs() + (y - *coord_y).abs()
                    })
                    .iter()
                    .exactly_one()
                {
                    areas_by_idx[*closest_idx] += 1;
                }
            }
        }
        let mut infinites = HashSet::<usize>::new();
        for x in (left - 1)..=(right + 1) {
            for y in [top - 1, bottom + 1] {
                if let Ok((closest_idx, _)) = self
                    .input
                    .iter()
                    .enumerate()
                    .min_set_by_key(|(_, (coord_x, coord_y))| {
                        (x - *coord_x).abs() + (y - *coord_y).abs()
                    })
                    .iter()
                    .exactly_one()
                {
                    infinites.insert(*closest_idx);
                }
            }
        }
        for x in [left - 1, right + 1] {
            for y in (top - 1)..=(bottom + 1) {
                if let Ok((closest_idx, _)) = self
                    .input
                    .iter()
                    .enumerate()
                    .min_set_by_key(|(_, (coord_x, coord_y))| {
                        (x - *coord_x).abs() + (y - *coord_y).abs()
                    })
                    .iter()
                    .exactly_one()
                {
                    infinites.insert(*closest_idx);
                }
            }
        }
        debug!("{:?}", areas_by_idx);
        areas_by_idx
            .iter()
            .enumerate()
            .filter(|(idx, _)| !infinites.contains(idx))
            .map(|(_, area)| *area)
            .max()
            .unwrap()
            .to_string()
    }

    fn solve_part_two(&mut self) -> String {
        let (mut left, mut right, mut top, mut bottom) = (
            self.input[0].0,
            self.input[0].0,
            self.input[0].1,
            self.input[0].1,
        );
        for (x, y) in self.input.iter().skip(1) {
            left = std::cmp::min(left, *x);
            right = std::cmp::max(right, *x);
            top = std::cmp::min(top, *y);
            bottom = std::cmp::max(bottom, *y);
        }
        let mut region_size = 0;
        for x in left..=right {
            for y in top..=bottom {
                if self
                    .input
                    .iter()
                    .map(|(coord_x, coord_y)| (x - *coord_x).abs() + (y - *coord_y).abs())
                    .sum::<i64>()
                    < 10000
                {
                    region_size += 1;
                }
            }
        }
        let mut expand_box_by = 1;
        loop {
            let mut additional_region = 0;
            for x in [left - expand_box_by, right + expand_box_by] {
                for y in (top - expand_box_by)..=(bottom + expand_box_by) {
                    if self
                        .input
                        .iter()
                        .map(|(coord_x, coord_y)| (x - *coord_x).abs() + (y - *coord_y).abs())
                        .sum::<i64>()
                        < 10000
                    {
                        additional_region += 1;
                    }
                }
            }
            for x in (left - expand_box_by)..=(right + expand_box_by) {
                for y in [top - expand_box_by, bottom + expand_box_by] {
                    if self
                        .input
                        .iter()
                        .map(|(coord_x, coord_y)| (x - *coord_x).abs() + (y - *coord_y).abs())
                        .sum::<i64>()
                        < 10000
                    {
                        additional_region += 1;
                    }
                }
            }
            if additional_region == 0 {
                break;
            }
            region_size += additional_region;
            expand_box_by += 1;
        }
        region_size.to_string()
    }
}

pub fn solver() -> PuzzleSolver {
    PuzzleSolver::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn provided_example() {
        let mut s = solver();
        s.presolve(
            "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9
",
        );
        assert_eq!("17", s.solve_part_one());
    }

    #[test]
    fn provided_example_with_nl() {}
}
