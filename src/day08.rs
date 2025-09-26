use crate::solver::Solver;

#[derive(Default)]
pub struct PuzzleSolver {
    input: Vec<isize>,
}

fn sum(input: &[isize]) -> (isize, usize) {
    let children_count = input[0];
    let metadata_count = input[1] as usize;
    let mut children_size = 0;
    let mut children_metadata_sum = 0;
    for _ in 0..children_count {
        let (child_sum, child_size) = sum(&input[(2 + children_size)..]);
        children_size += child_size;
        children_metadata_sum += child_sum;
    }
    let mut metadata_sum = children_metadata_sum;
    for i in 0..metadata_count {
        metadata_sum += input[2 + children_size + i];
    }
    (metadata_sum, 2 + children_size + metadata_count)
}

fn sum_b(input: &[isize]) -> (isize, usize) {
    let children_count = input[0];
    let metadata_count = input[1] as usize;
    let mut children_size = 0;
    let mut metadata_sum = 0;
    if children_count == 0 {
        for i in 0..metadata_count {
            metadata_sum += input[2 + i];
        }
    } else {
        let mut children_values = vec![];
        for _ in 0..children_count {
            let (child_value, child_size) = sum_b(&input[(2 + children_size)..]);
            children_size += child_size;
            children_values.push(child_value);
        }
        for metadata_idx in 0..metadata_count {
            let child_idx = (input[2 + children_size + metadata_idx] - 1) as usize;
            if child_idx < children_values.len() {
                metadata_sum += children_values[child_idx];
            }
        }
    }
    (metadata_sum, 2 + children_size + metadata_count)
}

impl Solver for PuzzleSolver {
    fn presolve(&mut self, input: &str) {
        self.input = input
            .trim()
            .split(' ')
            .map(|s| s.parse().unwrap())
            .collect();
    }

    fn solve_part_one(&mut self) -> String {
        sum(self.input.as_slice()).0.to_string()
    }

    fn solve_part_two(&mut self) -> String {
        sum_b(self.input.as_slice()).0.to_string()
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
        s.presolve("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
        assert_eq!("138", s.solve_part_one());
        assert_eq!("66", s.solve_part_two());
    }
}
