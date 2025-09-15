use crate::solver::Solver;

#[derive(Default)]
pub struct PuzzleSolver {
    input: String,
}

impl Solver for PuzzleSolver {
    fn presolve(&mut self, input: &str) {
        self.input = input.trim().to_string();
    }

    fn solve_part_one(&mut self) -> String {
        let mut string: Vec<_> = self.input.chars().collect();
        loop {
            let mut modified = false;
            for i in 0..(string.len() - 1) {
                if string[i].is_ascii_lowercase() && string[i].to_ascii_uppercase() == string[i + 1]
                    || string[i + 1].is_ascii_lowercase()
                        && string[i + 1].to_ascii_uppercase() == string[i]
                {
                    string.remove(i + 1);
                    string.remove(i);
                    modified = true;
                    break;
                }
            }
            if !modified {
                break;
            }
        }
        string.len().to_string()
    }

    fn solve_part_two(&mut self) -> String {
        let mut min = std::usize::MAX;
        for remove in 'a'..'z' {
            let mut string: Vec<_> = self
                .input
                .chars()
                .filter(|&ch| ch != remove && ch.to_ascii_lowercase() != remove)
                .collect();
            loop {
                let mut modified = false;
                for i in 0..(string.len() - 1) {
                    if string[i].is_ascii_lowercase()
                        && string[i].to_ascii_uppercase() == string[i + 1]
                        || string[i + 1].is_ascii_lowercase()
                            && string[i + 1].to_ascii_uppercase() == string[i]
                    {
                        string.remove(i + 1);
                        string.remove(i);
                        modified = true;
                        break;
                    }
                }
                if !modified {
                    break;
                }
            }
            min = std::cmp::min(min, string.len());
        }
        min.to_string()
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
        s.presolve("dabAcCaCBAcCcaDA");
        assert_eq!("10", s.solve_part_one());
    }

    #[test]
    fn provided_example_with_nl() {
        let mut s = solver();
        s.presolve("dabAcCaCBAcCcaDA\n");
        assert_eq!("4", s.solve_part_two());
    }
}
