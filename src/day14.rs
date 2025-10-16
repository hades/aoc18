use crate::solver::Solver;

pub struct PuzzleSolver {
    recipes: usize,
}

impl Solver for PuzzleSolver {
    fn presolve(&mut self, input: &str) {
        self.recipes = input.trim().parse().unwrap();
    }

    fn solve_part_one(&mut self) -> String {
        let mut data = vec![3u8, 7u8];
        let mut elf1 = 0;
        let mut elf2 = 1;
        while data.len() < self.recipes + 10 {
            let sum = data[elf1] + data[elf2];
            if sum > 9 {
                data.push(1);
                data.push(sum - 10);
            } else {
                data.push(sum);
            }
            elf1 = (elf1 + 1usize + data[elf1] as usize) % data.len();
            elf2 = (elf2 + 1usize + data[elf2] as usize) % data.len();
        }
        String::from_utf8(
            data.iter()
                .skip(self.recipes)
                .take(10)
                .map(|v| b'0' + v)
                .collect(),
        )
        .unwrap()
    }

    fn solve_part_two(&mut self) -> String {
        let string = {
            let mut digits = vec![];
            let mut remainder = self.recipes;
            while remainder > 0 {
                digits.push((remainder % 10) as u8);
                remainder /= 10;
            }
            digits.reverse();
            digits
        };
        let mut data = vec![3u8, 7u8];
        let mut elf1 = 0;
        let mut elf2 = 1;
        let mut next_to_search = 0;
        loop {
            while data.len() >= next_to_search + string.len() {
                if string
                    .iter()
                    .zip(data[next_to_search..next_to_search + string.len()].iter())
                    .all(|(a, b)| *a == *b)
                {
                    return next_to_search.to_string();
                }
                next_to_search += 1;
            }
            let sum = data[elf1] + data[elf2];
            if sum > 9 {
                data.push(1);
                data.push(sum - 10);
            } else {
                data.push(sum);
            }
            elf1 = (elf1 + 1usize + data[elf1] as usize) % data.len();
            elf2 = (elf2 + 1usize + data[elf2] as usize) % data.len();
        }
    }
}

pub fn solver() -> PuzzleSolver {
    PuzzleSolver { recipes: 0 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn provided_example_1() {
        let mut s = solver();
        s.presolve("9");
        assert_eq!("5158916779", s.solve_part_one());
    }

    #[test]
    fn provided_example_2() {
        let mut s = solver();
        s.presolve("5");
        assert_eq!("0124515891", s.solve_part_one());
    }

    #[test]
    fn provided_example_3() {
        let mut s = solver();
        s.presolve("18");
        assert_eq!("9251071085", s.solve_part_one());
    }

    #[test]
    fn provided_example_4() {
        let mut s = solver();
        s.presolve("2018");
        assert_eq!("5941429882", s.solve_part_one());
    }

    #[test]
    fn provided_example_5() {
        let mut s = solver();
        s.presolve("51589");
        assert_eq!("9", s.solve_part_two());
    }

    #[test]
    fn provided_example_7() {
        let mut s = solver();
        s.presolve("92510");
        assert_eq!("18", s.solve_part_two());
    }

    #[test]
    fn provided_example_8() {
        let mut s = solver();
        s.presolve("59414");
        assert_eq!("2018", s.solve_part_two());
    }
}
