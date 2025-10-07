use std::collections::HashSet;

use crate::solver::Solver;

#[derive(Default)]
pub struct PuzzleSolver {
    initial_state: String,
    rules: HashSet<String>,
}

impl Solver for PuzzleSolver {
    fn presolve(&mut self, input: &str) {
        let mut lines = input.lines();
        self.initial_state = lines
            .next()
            .unwrap()
            .chars()
            .skip("initial state: ".len())
            .collect();
        lines.next();
        for rule_line in lines {
            if rule_line.chars().nth("..... => ".len()).unwrap() != '#' {
                continue;
            }
            self.rules.insert(rule_line.chars().take(5).collect());
        }
    }

    fn solve_part_one(&mut self) -> String {
        let mut state: HashSet<isize> = self
            .initial_state
            .chars()
            .enumerate()
            .filter(|(_, ch)| *ch == '#')
            .map(|(no, _)| no as isize)
            .collect();
        for _ in 0..20 {
            let minimum_position = state.iter().min().unwrap() - 2;
            let maximum_position = state.iter().max().unwrap() + 2;
            let new_state = (minimum_position..=maximum_position)
                .filter(|plant_position| {
                    let pattern: String = (-2..=2)
                        .map(|dp| state.contains(&(plant_position + dp)))
                        .map(|b| if b { '#' } else { '.' })
                        .collect();

                    self.rules.contains(&pattern)
                })
                .collect();
            state = new_state;
        }
        state.iter().sum::<isize>().to_string()
    }

    fn solve_part_two(&mut self) -> String {
        let mut state: HashSet<isize> = self
            .initial_state
            .chars()
            .enumerate()
            .filter(|(_, ch)| *ch == '#')
            .map(|(no, _)| no as isize)
            .collect();
        let total_steps = 50_000_000_000i64;
        for step in 0..total_steps {
            let minimum_position = state.iter().min().unwrap() - 2;
            let maximum_position = state.iter().max().unwrap() + 2;
            let new_state = (minimum_position..=maximum_position)
                .filter(|plant_position| {
                    let pattern: String = (-2..=2)
                        .map(|dp| state.contains(&(plant_position + dp)))
                        .map(|b| if b { '#' } else { '.' })
                        .collect();

                    self.rules.contains(&pattern)
                })
                .collect();

            // In this puzzle, a semi-stable state is achieved after a certain (~200) number of steps.
            // After this happens, the next state is always the previous state shifted to the right
            // by 1 "unit". (Of course, this could be specific to my puzzle instance, other people
            // might get different semi-stable states.)

            // Check if the new state is equal to previous state shifted by one unit.
            let shift_previous_state: HashSet<_> = state.iter().map(|i| i + 1).collect();
            if shift_previous_state == new_state {
                // Generate the final state by shifting the previous state by the number of remaining
                // steps.
                state = new_state
                    .iter()
                    .map(|i| i + (total_steps - step - 1) as isize)
                    .collect();
                break;
            }
            state = new_state;
        }
        state.iter().sum::<isize>().to_string()
    }
}

pub fn solver() -> PuzzleSolver {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn provided_example_1() {
        let mut s = solver();
        s.presolve(
            "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #
",
        );
        assert_eq!("325", s.solve_part_one());
    }
}
