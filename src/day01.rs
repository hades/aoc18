use std::collections::HashSet;

use crate::solver::Solver;

#[derive(Default)]
pub struct Day1Solver {
    input: Vec<isize>,
}

impl Solver for Day1Solver {
    fn presolve(&mut self, input: &str) {
        self.input = input
            .trim()
            .split("\n")
            .map(|line| line.parse::<isize>().unwrap())
            .collect();
    }

    fn solve_part_one(&mut self) -> String {
        self.input.iter().sum::<isize>().to_string()
    }

    fn solve_part_two(&mut self) -> String {
        let mut set = HashSet::<isize>::new();
        let mut frequency = 0;
        loop {
            for v in self.input.iter() {
                frequency += v;
                if set.contains(&frequency) {
                    return frequency.to_string();
                }
                set.insert(frequency);
            }
        }
    }
}

pub fn solver() -> Day1Solver {
    Day1Solver::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn provided_example() {
        let example = "+1
+1
+1
";
        let mut s = solver();
        s.presolve(example);
        assert_eq!("3", s.solve_part_one());
    }

    #[test]
    fn provided_example_two() {
        let example = "+7
+7
-2
-7
-4
";
        let mut s = solver();
        s.presolve(example);
        assert_eq!("14", s.solve_part_two());
    }
}
