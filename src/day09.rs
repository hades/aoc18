use regex::Regex;

use crate::solver::Solver;

#[derive(Clone, Copy, Default)]
pub struct PuzzleSolver {
    player_count: usize,
    last_marble: u64,
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
        let mut marbles_head = vec![0];
        let mut marbles_tail = vec![];
        let mut tail_pointer = 0;
        let mut current_marble = 0;
        let mut marbles_count = 1;
        for marble in 1..=self.last_marble {
            if marble % 23 != 0 {
                let insert_index = (current_marble + 1) % marbles_count + 1;
                // Insertion for dual-vector representation.
                //
                // At this point the array of marbles is as follows:
                //  [... a b c x y z ...]
                // Represented by:
                //  marbles_head: [... a b c]
                //  marbles_tail: [... x y z ...]
                //  tail_pointer:      ^
                //
                // The goal of this optimisation is to make consequtive localised insertions
                // and removals efficient.
                if insert_index > marbles_head.len() {
                    // If the insertion happens in the tail of the array, we copy the prefix of the
                    // tail to the head, and push the new element to the end of the head. This
                    // is O(1) copies as long as insertions are localised and monotonic.
                    while insert_index > marbles_head.len() {
                        assert!(tail_pointer < marbles_tail.len());
                        marbles_head.push(marbles_tail[tail_pointer]);
                        tail_pointer += 1;
                    }
                    marbles_head.push(marble);
                } else {
                    // Insertions near the end of the head can also be efficient. However, in this
                    // particular case we're dealing with monotonic insertions, and we can assume
                    // that an insertion at the head is always inefficient, so we create a new
                    // split at the insertion point.
                    let mut new_tail = Vec::from_iter(marbles_head.drain(insert_index..));
                    new_tail.extend(&marbles_tail[tail_pointer..]);
                    marbles_head.truncate(insert_index);
                    marbles_head.push(marble);
                    marbles_tail = new_tail;
                    tail_pointer = 0;
                }
                current_marble = insert_index;
                marbles_count += 1;
            } else {
                player_scores[current_player] += marble;
                let remove_index = (current_marble + marbles_count).wrapping_sub(7) % marbles_count;
                let removed_marble = if remove_index < marbles_head.len() {
                    marbles_head.remove(remove_index)
                } else {
                    // The removed index will always be >= tail_pointer, meaning that we don't have
                    // to update the tail_pointer here.
                    marbles_tail.remove(remove_index - marbles_head.len() + tail_pointer)
                };
                current_marble = remove_index;
                player_scores[current_player] += removed_marble;
                marbles_count -= 1;
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
