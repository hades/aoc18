use std::collections::HashMap;

use crate::solver::Solver;

#[derive(Default)]
pub struct Day1Solver {
    input: Vec<String>,
}

impl Solver for Day1Solver {
    fn presolve(&mut self, input: &str) {
        self.input = input
            .trim()
            .split("\n")
            .map(|line| line.to_string())
            .collect();
    }

    fn solve_part_one(&mut self) -> String {
        let mut with_two = 0;
        let mut with_three = 0;
        for box_id in self.input.iter() {
            let mut counts = HashMap::<char, usize>::new();
            for ch in box_id.chars() {
                *counts.entry(ch).or_insert(0) += 1;
            }
            let counts: Vec<_> = counts.into_values().collect();
            if counts.iter().any(|v| *v == 2) {
                with_two += 1;
            }
            if counts.iter().any(|v| *v == 3) {
                with_three += 1;
            }
        }
        (with_three * with_two).to_string()
    }

    fn solve_part_two(&mut self) -> String {
        for a in self.input.iter() {
            for b in self.input.iter() {
                let common_chars = String::from_iter(
                    a.chars()
                        .zip(b.chars())
                        .filter(|(a, b)| *a == *b)
                        .map(|(a, _)| a),
                );
                if common_chars.len() == b.len() - 1 {
                    return common_chars;
                }
            }
        }
        unreachable!()
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
        let example = "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab
";
        let mut s = solver();
        s.presolve(example);
        assert_eq!("12", s.solve_part_one());
    }

    #[test]
    fn provided_example_two() {
        let example = "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz
";
        let mut s = solver();
        s.presolve(example);
        assert_eq!("fgij", s.solve_part_two());
    }
}
