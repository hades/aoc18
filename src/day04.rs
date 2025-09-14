use std::collections::{HashMap, HashSet};

use regex::Regex;

use crate::solver::Solver;

enum Event {
    FallsAsleep,
    WakesUp,
    BeginsShift(i64),
}

#[derive(Default)]
pub struct PuzzleSolver {
    input: Vec<(String, String, String, Event)>,
}

impl Solver for PuzzleSolver {
    fn presolve(&mut self, input: &str) {
        let re = Regex::new(
            r"\[([^ ]+) (\d\d):(\d\d)\] (wakes up|falls asleep|Guard #(\d+) begins shift)",
        )
        .unwrap();
        self.input = input
            .trim()
            .split("\n")
            .map(|line| match re.captures(line) {
                None => {
                    panic!("unable to parse line: >{line}<");
                }
                Some(c) => c,
            })
            .map(|m| {
                let date = m.get(1).unwrap().as_str().to_string();
                let hour = m.get(2).unwrap().as_str().to_string();
                let minute = m.get(3).unwrap().as_str().to_string();
                let event = match m.get(4).unwrap().as_str() {
                    "wakes up" => Event::WakesUp,
                    "falls asleep" => Event::FallsAsleep,
                    _ => Event::BeginsShift(m.get(5).unwrap().as_str().parse().unwrap()),
                };
                (date, hour, minute, event)
            })
            .collect();
        self.input.sort_by(
            |(a_date, a_hour, a_minute, _), (b_date, b_hour, b_minute, _)| {
                (a_date, a_hour, a_minute).cmp(&(b_date, b_hour, b_minute))
            },
        );
    }

    fn solve_part_one(&mut self) -> String {
        let mut current_guard = -1;
        let mut current_sleep_started_at = -1;
        let mut spent_sleeping_per_guard_per_minute = HashMap::<(i64, i64), i64>::new();
        let mut guard_ids = HashSet::new();
        for (_, _, minute, event) in self.input.iter() {
            match event {
                Event::BeginsShift(guard) => {
                    current_guard = *guard;
                    guard_ids.insert(*guard);
                }
                Event::FallsAsleep => {
                    current_sleep_started_at = minute.parse().unwrap();
                }
                Event::WakesUp => {
                    for minute in current_sleep_started_at..(minute.parse().unwrap()) {
                        *spent_sleeping_per_guard_per_minute
                            .entry((current_guard, minute))
                            .or_default() += 1;
                    }
                }
            }
        }
        let sleepiest_guard = *guard_ids
            .iter()
            .max_by_key(|&guard_id| {
                spent_sleeping_per_guard_per_minute
                    .iter()
                    .filter(|((g, _), _)| *g == *guard_id)
                    .map(|(_, v)| *v)
                    .sum::<i64>()
            })
            .unwrap();
        let sleepiest_minute = spent_sleeping_per_guard_per_minute
            .iter()
            .filter(|((g, _), _)| *g == sleepiest_guard)
            .max_by_key(|(_, days_on_which_slept)| **days_on_which_slept)
            .unwrap()
            .0
            .1;
        (sleepiest_guard * sleepiest_minute).to_string()
    }

    fn solve_part_two(&mut self) -> String {
        let mut current_guard = -1;
        let mut current_sleep_started_at = -1;
        let mut spent_sleeping_per_guard_per_minute = HashMap::<(i64, i64), i64>::new();
        let mut guard_ids = HashSet::new();
        for (_, _, minute, event) in self.input.iter() {
            match event {
                Event::BeginsShift(guard) => {
                    current_guard = *guard;
                    guard_ids.insert(*guard);
                }
                Event::FallsAsleep => {
                    current_sleep_started_at = minute.parse().unwrap();
                }
                Event::WakesUp => {
                    for minute in current_sleep_started_at..(minute.parse().unwrap()) {
                        *spent_sleeping_per_guard_per_minute
                            .entry((current_guard, minute))
                            .or_default() += 1;
                    }
                }
            }
        }
        let (sleepiest_guard, sleepiest_minute) = spent_sleeping_per_guard_per_minute
            .iter()
            .max_by_key(|(_, days_on_which_slept)| **days_on_which_slept)
            .unwrap()
            .0;
        (*sleepiest_guard * *sleepiest_minute).to_string()
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
            "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up
",
        );
        assert_eq!("240", s.solve_part_one());
    }

    #[test]
    fn provided_example_part_two() {
        let mut s = solver();
        s.presolve(
            "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up
",
        );
        assert_eq!("4455", s.solve_part_two());
    }
}
