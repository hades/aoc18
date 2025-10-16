use std::collections::BTreeMap;

use array2d::Array2D;

use crate::solver::Solver;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct PuzzleSolver {
    map: Array2D<char>,
    initial_positions: Vec<(usize, usize, Direction)>,
}

impl Solver for PuzzleSolver {
    fn presolve(&mut self, input: &str) {
        let height = input.lines().count();
        let width = input.lines().map(|l| l.len()).max().unwrap();
        self.map = Array2D::filled_with(' ', height, width);
        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                match ch {
                    '/' | '\\' | '-' | '|' | '+' => {
                        self.map[(y, x)] = ch;
                    }
                    '^' => {
                        self.map[(y, x)] = '|';
                        self.initial_positions.push((x, y, Direction::Up));
                    }
                    'v' => {
                        self.map[(y, x)] = '|';
                        self.initial_positions.push((x, y, Direction::Down));
                    }
                    '<' => {
                        self.map[(y, x)] = '-';
                        self.initial_positions.push((x, y, Direction::Left));
                    }
                    '>' => {
                        self.map[(y, x)] = '-';
                        self.initial_positions.push((x, y, Direction::Right));
                    }
                    ' ' => {}
                    _ => panic!("unexpected character >{ch}<"),
                }
            }
        }
    }

    fn solve_part_one(&mut self) -> String {
        let mut state: BTreeMap<(usize, usize), (Direction, usize)> = self
            .initial_positions
            .iter()
            .map(|(x, y, dir)| ((*y, *x), (*dir, 0)))
            .collect();
        loop {
            let mut new_state = BTreeMap::new();
            for ((cart_y, cart_x), (cart_dir, cart_turn_count)) in state.into_iter() {
                let (new_x, new_y, new_dir) =
                    match (self.map[(cart_y, cart_x)], cart_dir, cart_turn_count % 3) {
                        ('|', Direction::Up, _) => (cart_x, cart_y - 1, cart_dir),
                        ('|', Direction::Down, _) => (cart_x, cart_y + 1, cart_dir),
                        ('-', Direction::Left, _) => (cart_x - 1, cart_y, cart_dir),
                        ('-', Direction::Right, _) => (cart_x + 1, cart_y, cart_dir),
                        ('+', Direction::Up, 0) => (cart_x - 1, cart_y, Direction::Left),
                        ('+', Direction::Down, 2) => (cart_x - 1, cart_y, Direction::Left),
                        ('+', Direction::Left, 1) => (cart_x - 1, cart_y, Direction::Left),
                        ('\\', Direction::Up, _) => (cart_x - 1, cart_y, Direction::Left),
                        ('/', Direction::Down, _) => (cart_x - 1, cart_y, Direction::Left),
                        ('+', Direction::Down, 1) => (cart_x, cart_y + 1, Direction::Down),
                        ('+', Direction::Left, 0) => (cart_x, cart_y + 1, Direction::Down),
                        ('+', Direction::Right, 2) => (cart_x, cart_y + 1, Direction::Down),
                        ('\\', Direction::Right, _) => (cart_x, cart_y + 1, Direction::Down),
                        ('/', Direction::Left, _) => (cart_x, cart_y + 1, Direction::Down),
                        ('+', Direction::Up, 2) => (cart_x + 1, cart_y, Direction::Right),
                        ('+', Direction::Down, 0) => (cart_x + 1, cart_y, Direction::Right),
                        ('+', Direction::Right, 1) => (cart_x + 1, cart_y, Direction::Right),
                        ('/', Direction::Up, _) => (cart_x + 1, cart_y, Direction::Right),
                        ('\\', Direction::Down, _) => (cart_x + 1, cart_y, Direction::Right),
                        ('+', Direction::Up, 1) => (cart_x, cart_y - 1, Direction::Up),
                        ('+', Direction::Left, 2) => (cart_x, cart_y - 1, Direction::Up),
                        ('+', Direction::Right, 0) => (cart_x, cart_y - 1, Direction::Up),
                        ('/', Direction::Right, _) => (cart_x, cart_y - 1, Direction::Up),
                        ('\\', Direction::Left, _) => (cart_x, cart_y - 1, Direction::Up),
                        s => panic!("unexpected state: {s:?}"),
                    };
                let new_turn_count = cart_turn_count
                    + if self.map[(cart_y, cart_x)] == '+' {
                        1
                    } else {
                        0
                    };
                if new_state.contains_key(&(new_y, new_x)) {
                    return format!("{new_x},{new_y}");
                }
                new_state.insert((new_y, new_x), (new_dir, new_turn_count));
            }
            state = new_state;
        }
    }

    fn solve_part_two(&mut self) -> String {
        let mut state: BTreeMap<(usize, usize), (Direction, usize)> = self
            .initial_positions
            .iter()
            .map(|(x, y, dir)| ((*y, *x), (*dir, 0)))
            .collect();
        loop {
            let mut new_state = BTreeMap::new();
            for ((cart_y, cart_x), (cart_dir, cart_turn_count)) in state.into_iter() {
                if new_state.remove(&(cart_y, cart_x)).is_some() {
                    continue;
                }
                let (new_x, new_y, new_dir) =
                    match (self.map[(cart_y, cart_x)], cart_dir, cart_turn_count % 3) {
                        ('|', Direction::Up, _) => (cart_x, cart_y - 1, cart_dir),
                        ('|', Direction::Down, _) => (cart_x, cart_y + 1, cart_dir),
                        ('-', Direction::Left, _) => (cart_x - 1, cart_y, cart_dir),
                        ('-', Direction::Right, _) => (cart_x + 1, cart_y, cart_dir),
                        ('+', Direction::Up, 0) => (cart_x - 1, cart_y, Direction::Left),
                        ('+', Direction::Down, 2) => (cart_x - 1, cart_y, Direction::Left),
                        ('+', Direction::Left, 1) => (cart_x - 1, cart_y, Direction::Left),
                        ('\\', Direction::Up, _) => (cart_x - 1, cart_y, Direction::Left),
                        ('/', Direction::Down, _) => (cart_x - 1, cart_y, Direction::Left),
                        ('+', Direction::Down, 1) => (cart_x, cart_y + 1, Direction::Down),
                        ('+', Direction::Left, 0) => (cart_x, cart_y + 1, Direction::Down),
                        ('+', Direction::Right, 2) => (cart_x, cart_y + 1, Direction::Down),
                        ('\\', Direction::Right, _) => (cart_x, cart_y + 1, Direction::Down),
                        ('/', Direction::Left, _) => (cart_x, cart_y + 1, Direction::Down),
                        ('+', Direction::Up, 2) => (cart_x + 1, cart_y, Direction::Right),
                        ('+', Direction::Down, 0) => (cart_x + 1, cart_y, Direction::Right),
                        ('+', Direction::Right, 1) => (cart_x + 1, cart_y, Direction::Right),
                        ('/', Direction::Up, _) => (cart_x + 1, cart_y, Direction::Right),
                        ('\\', Direction::Down, _) => (cart_x + 1, cart_y, Direction::Right),
                        ('+', Direction::Up, 1) => (cart_x, cart_y - 1, Direction::Up),
                        ('+', Direction::Left, 2) => (cart_x, cart_y - 1, Direction::Up),
                        ('+', Direction::Right, 0) => (cart_x, cart_y - 1, Direction::Up),
                        ('/', Direction::Right, _) => (cart_x, cart_y - 1, Direction::Up),
                        ('\\', Direction::Left, _) => (cart_x, cart_y - 1, Direction::Up),
                        s => panic!("unexpected state: {s:?} at {cart_x},{cart_y}"),
                    };
                let new_turn_count = cart_turn_count
                    + if self.map[(cart_y, cart_x)] == '+' {
                        1
                    } else {
                        0
                    };
                if new_state.remove(&(new_y, new_x)).is_none() {
                    new_state.insert((new_y, new_x), (new_dir, new_turn_count));
                }
            }
            state = new_state;
            if state.len() == 1 {
                let (cart_y, cart_x) = state.into_keys().next().unwrap();
                return format!("{cart_x},{cart_y}");
            }
        }
    }
}

pub fn solver() -> PuzzleSolver {
    PuzzleSolver {
        map: Array2D::filled_with(' ', 1, 1),
        initial_positions: vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn provided_example_1() {
        let mut s = solver();
        s.presolve(
            r"/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   ",
        );
        assert_eq!("7,3", s.solve_part_one());
    }

    #[test]
    fn provided_example_2() {
        let mut s = solver();
        s.presolve(
            r"/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/",
        );
        assert_eq!("6,4", s.solve_part_two());
    }
}
