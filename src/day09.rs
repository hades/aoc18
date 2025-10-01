use regex::Regex;

use crate::solver::Solver;

#[derive(Clone, Copy, Default)]
pub struct PuzzleSolver {
    player_count: usize,
    last_marble: u64,
}

// Represents a vector with an efficient implementation for "batch insertions", i.e.
// sequence of insertions at monotonously increasing insert positions.
// E.g.
//  [a, b, c, d, e]
//    insert(1, x)
//  [a, x, b, c, d, e]
//    insert(2, y)
//  [a, x, y, b, c, d, e]
pub struct BatchedInsertVec<T> {
    base: Vec<T>,
    queue: Vec<(usize, T)>,
}

impl<T> BatchedInsertVec<T> {
    fn _merge(&mut self) {
        let mut base_iter = self.base.drain(..);
        let mut new_base = vec![];
        let mut queue_iter = self.queue.drain(..).peekable();
        let mut current_idx = 0usize;
        loop {
            // First check if there is a queued insertion at the current index.
            // In other words, if there exists an insertion at (1, x), we will
            // process that insertion when we've arrived at index 1 of the
            // output vector.
            if let Some((_, item)) =
                queue_iter.next_if(|(next_queued_idx, _)| *next_queued_idx == current_idx)
            {
                new_base.push(item);
                current_idx += 1;
                continue;
            }
            // Otherwise, continue inserting items from the original base vector.
            if let Some(item) = base_iter.next() {
                new_base.push(item);
                current_idx += 1;
                continue;
            }
            break;
        }
        drop(base_iter);
        self.base = new_base;
    }

    fn len(&self) -> usize {
        self.base.len() + self.queue.len()
    }

    fn insert(&mut self, position: usize, element: T) {
        // If the insertion queue is not empty, check the monotonicity of the new
        // insert.
        let needs_merge = match self.queue.last() {
            Some((max_queue_position, _)) => *max_queue_position >= position,
            None => false,
        };
        if needs_merge {
            self._merge();
        }
        // Now either the queue has been emptied, or we've checked that position
        // is greater than the maximum inserted position in the queue.

        // Check if the new position is within the bounds of the insertion range
        // 0..=len()
        assert!(position <= self.len());

        // All good, we can insert the operation in the queue.
        self.queue.push((position, element));
    }

    fn remove(&mut self, position: usize) -> T {
        self._merge();
        self.base.remove(position)
    }
}

impl<T> From<Vec<T>> for BatchedInsertVec<T> {
    fn from(value: Vec<T>) -> Self {
        Self {
            base: value,
            queue: vec![],
        }
    }
}

impl Solver for PuzzleSolver {
    fn presolve(&mut self, input: &str) {
        let re = Regex::new(r"\d+").unwrap();
        let mut parse_input = re.find_iter(input);
        self.player_count = parse_input.next().unwrap().as_str().parse().unwrap();
        self.last_marble = parse_input.next().unwrap().as_str().parse().unwrap();
    }

    fn solve_part_one(&mut self) -> String {
        let mut player_scores = vec![0; self.player_count];
        let mut current_player = 0;
        let mut marbles: BatchedInsertVec<u64> = vec![0].into();
        let mut current_marble = 0;
        for marble in 1..=self.last_marble {
            if marble % 23 != 0 {
                let insert_index = (current_marble + 1) % marbles.len() + 1;
                marbles.insert(insert_index, marble);
                current_marble = insert_index;
            } else {
                player_scores[current_player] += marble;
                let remove_index = (current_marble + marbles.len()).wrapping_sub(7) % marbles.len();
                let removed_marble = marbles.remove(remove_index);
                current_marble = remove_index;
                player_scores[current_player] += removed_marble;
            }
            current_player = (current_player + 1) % self.player_count;
        }
        player_scores.iter().max().unwrap().to_string()
    }

    fn solve_part_two(&mut self) -> String {
        PuzzleSolver {
            last_marble: self.last_marble * 100,
            ..*self
        }
        .solve_part_one()
    }
}

pub fn solver() -> PuzzleSolver {
    PuzzleSolver::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;
    use yare::parameterized;

    #[parameterized(
    example_1 = {"9 players; last marble is worth 25 points", "32"},
    example_2 = {"10 players; last marble is worth 1618 points", "8317"},
    example_3 = {"13 players; last marble is worth 7999 points", "146373"},
    example_4 = {"17 players; last marble is worth 1104 points", "2764"},
    example_5 = {"21 players; last marble is worth 6111 points", "54718"},
    example_6 = {"30 players; last marble is worth 5807 points", "37305"},
)]
    #[test_macro(test_log::test)]
    fn provided_example(input: &str, expected_result: &str) {
        let mut s = solver();
        s.presolve(input);
        assert_eq!(expected_result, s.solve_part_one());
    }
}
