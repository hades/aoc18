use std::{collections::BTreeSet, iter::repeat_n};

use itertools::Itertools;
use regex::Regex;

use crate::solver::Solver;

#[derive(Default)]
pub struct PuzzleSolver {
    input: Vec<(String, String)>,
    worker_count: usize,
    work_time: usize,
}

impl Solver for PuzzleSolver {
    fn presolve(&mut self, input: &str) {
        let re = Regex::new(r"Step (\w+) must be finished before step (\w+) can begin.").unwrap();
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
                let prerequisite = m.get(1).unwrap().as_str().to_string();
                let for_step = m.get(2).unwrap().as_str().to_string();
                (prerequisite, for_step)
            })
            .collect();
        self.worker_count = 5;
        self.work_time = 60;
    }

    fn solve_part_one(&mut self) -> String {
        let mut remaining_edges = self.input.clone();
        let mut ready = BTreeSet::from_iter(
            self.input
                .iter()
                .flat_map(|(a, b)| [a.as_str(), b.as_str()].into_iter()),
        );
        let all_nodes = ready.clone();
        self.input.iter().for_each(|(_, dependent)| {
            ready.remove(&dependent.as_str());
        });
        let mut operation_order = vec![];
        while let Some(next) = ready.pop_first() {
            operation_order.push(next);
            remaining_edges.retain(|(dependency, _)| dependency.as_str() != next);
            ready.extend(
                all_nodes
                    .iter()
                    .filter(|&&node| {
                        remaining_edges
                            .iter()
                            .filter(|(_, dependent)| dependent.as_str() == node)
                            .count()
                            == 0
                    })
                    .copied()
                    .filter(|s| !operation_order.contains(s)),
            );
        }
        operation_order.into_iter().join("")
    }

    fn solve_part_two(&mut self) -> String {
        let remaining_edges = self.input.clone();
        let all_nodes = BTreeSet::from_iter(
            self.input
                .iter()
                .flat_map(|(a, b)| [a.as_str(), b.as_str()].into_iter()),
        );
        let mut worker_jobs: Vec<_> = repeat_n(None, self.worker_count).collect();
        let mut scheduled_nodes: BTreeSet<&str> = BTreeSet::new();
        for node in all_nodes.iter().cloned() {
            if remaining_edges
                .iter()
                .filter(|(_, dependent)| dependent.as_str() == node)
                .count()
                == 0
                && let Some(next_free_worker) = worker_jobs.iter_mut().find(|job| job.is_none())
            {
                *next_free_worker = Some((node, 0));
                scheduled_nodes.insert(node);
            }
        }
        let mut t = 0;
        let mut finished_nodes: BTreeSet<&str> = BTreeSet::new();
        while worker_jobs.iter().any(|j| j.is_some()) {
            t += 1;
            for job in worker_jobs.iter_mut() {
                if job.is_none() {
                    continue;
                }
                let (job_node, job_start_time) = job.unwrap();
                let job_finish_time = 1isize.wrapping_add_unsigned(self.work_time)
                    + job_start_time
                    + job_node.chars().next().unwrap() as isize
                    - 'A' as isize;
                if job_finish_time <= t {
                    *job = None;
                    finished_nodes.insert(job_node);
                }
            }
            for &node in all_nodes.iter() {
                if finished_nodes.contains(node) || scheduled_nodes.contains(node) {
                    continue;
                }
                if remaining_edges
                    .iter()
                    .filter(|(_, dependent)| dependent.as_str() == node)
                    .filter(|(dependency, _)| !finished_nodes.contains(dependency.as_str()))
                    .count()
                    == 0
                    && let Some(next_free_worker) = worker_jobs.iter_mut().find(|job| job.is_none())
                {
                    *next_free_worker = Some((node, t));
                    scheduled_nodes.insert(node);
                }
            }
        }
        t.to_string()
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
            "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
",
        );
        assert_eq!("CABDFE", s.solve_part_one());
    }

    #[test]
    fn provided_example_part_two() {
        let mut s = solver();
        s.presolve(
            "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
",
        );
        s.work_time = 0;
        s.worker_count = 2;
        assert_eq!("15", s.solve_part_two());
    }
}
