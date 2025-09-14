use std::collections::{HashMap, HashSet};

use regex::Regex;

use crate::solver::Solver;

#[derive(Default)]
pub struct PuzzleSolver {
    input: Vec<(i64, i64, i64, i64)>,
}

impl Solver for PuzzleSolver {
    fn presolve(&mut self, input: &str) {
        let re = Regex::new(r"#\d+ @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        self.input = input
            .trim()
            .split("\n")
            .map(|line| re.captures(line).unwrap().extract())
            .map(|(_, [x, y, w, h])| {
                (
                    x.parse().unwrap(),
                    y.parse().unwrap(),
                    w.parse().unwrap(),
                    h.parse().unwrap(),
                )
            })
            .collect();
    }

    fn solve_part_one(&mut self) -> String {
        let mut claim_count_per_inch = HashMap::<(i64, i64), i64>::new();
        for &(start_x, start_y, w, h) in self.input.iter() {
            for x in start_x..(start_x + w) {
                for y in start_y..(start_y + h) {
                    *claim_count_per_inch.entry((x, y)).or_default() += 1;
                }
            }
        }
        claim_count_per_inch
            .into_values()
            .filter(|&v| v > 1)
            .count()
            .to_string()
    }

    fn solve_part_two(&mut self) -> String {
        let mut claims_per_inch = HashMap::<(i64, i64), HashSet<usize>>::new();
        let mut candidate_claims = HashSet::<usize>::new();
        for (claim_idx, &(start_x, start_y, w, h)) in self.input.iter().enumerate() {
            for x in start_x..(start_x + w) {
                for y in start_y..(start_y + h) {
                    claims_per_inch.entry((x, y)).or_default().insert(claim_idx);
                }
            }
            candidate_claims.insert(claim_idx);
        }
        claims_per_inch
            .into_values()
            .filter(|v| v.len() > 1)
            .flat_map(|v| v.into_iter())
            .for_each(|claim_idx| {
                candidate_claims.remove(&claim_idx);
            });
        (candidate_claims.iter().next().unwrap() + 1).to_string()
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
            "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
",
        );
        assert_eq!("4", s.solve_part_one());
    }

    #[test]
    fn provided_example_part_two() {
        let mut s = solver();
        s.presolve(
            "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
",
        );
        assert_eq!("3", s.solve_part_two());
    }
}
