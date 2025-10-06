use std::collections::HashSet;

use log::info;
use regex::Regex;

use crate::solver::Solver;

#[derive(Default)]
pub struct PuzzleSolver {
    initial_positions: Vec<(i64, i64)>,
    velocities: Vec<(i64, i64)>,
}

fn diameter(positions: &[(i64, i64)]) -> i64 {
    let first = positions[0];
    positions.iter().map(|(x, y)| (first.0 - x).abs() + (first.1 - y).abs()).max().unwrap()
}

fn print_screen(positions: &Vec<(i64, i64)>) {
    let start_x = positions.iter().map(|(x, _)| *x).min().unwrap();
    let end_x = positions.iter().map(|(x, _)| *x).max().unwrap();
    let start_y = positions.iter().map(|(_, y)| *y).min().unwrap();
    let end_y = positions.iter().map(|(_, y)| *y).max().unwrap();
    let set: HashSet<_> = HashSet::from_iter(positions);
    for y in start_y..=end_y {
        info!("> {}", String::from_iter((start_x..=end_x).map(|x| if set.contains(&(x, y)) { '#' } else { '.' })));
    }
}

impl Solver for PuzzleSolver {
    fn presolve(&mut self, input: &str) {
        let re = Regex::new(r"position=<\s*([-0-9]+),\s*([-0-9]+)> velocity=<\s*([-0-9]+),\s*([-0-9]+)>").unwrap();
        for (_, [px, py, vx, vy]) in re.captures_iter(input).map(|capture| capture.extract()) {
            self.initial_positions.push((px.parse().unwrap(), py.parse().unwrap()));
            self.velocities.push((vx.parse().unwrap(), vy.parse().unwrap()));
        }
    }

    fn solve_part_one(&mut self) -> String {
        let mut positions = self.initial_positions.clone();
        let mut peak_achieved = false;
        let mut t = 0;
        loop {
            if diameter(&positions) < 80 {
                info!("t == {t}");
                print_screen(&positions);
                peak_achieved = true;
            } else if peak_achieved {
                break;
            }
            for ((px, py), (vx, vy)) in positions.iter_mut().zip(self.velocities.iter()) {
                *px += *vx;
                *py += *vy;
            }
            t += 1;
        }
        "".into()
    }

    fn solve_part_two(&mut self) -> String {
        self.solve_part_one()
    }
}

pub fn solver() -> PuzzleSolver {
    PuzzleSolver::default()
}