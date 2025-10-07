use array2d::Array2D;

use crate::solver::Solver;

pub struct PuzzleSolver {
    serial_no: i64,
    cumulative_power_levels: Array2D<i64>,
}

impl Solver for PuzzleSolver {
    fn presolve(&mut self, input: &str) {
        self.serial_no = input.trim().parse().unwrap();
        for cell_x in 1usize..=300 {
            let rack_id = cell_x + 10;
            for cell_y in 1usize..=300 {
                let mut power_level: i64 = (rack_id * cell_y).try_into().unwrap();
                power_level += self.serial_no;
                power_level *= {
                    let rack_id: i64 = rack_id.try_into().unwrap();
                    rack_id
                };
                power_level = (power_level / 100) % 10;
                power_level -= 5;

                // `power_level` is now the level for the cell (cell_x, cell_y). We want to find
                // the cumulative power for all cells with x=1..cell_x and y=1..cell_y.

                // Start by storing the current cell's power.
                self.cumulative_power_levels[(cell_x, cell_y)] = power_level;

                // If this is the first row (cell_x == 1), we can simply add the current value to the
                // already computed value for cell_y == cell_y - 1
                if cell_x == 1 {
                    if cell_y > 1 {
                        self.cumulative_power_levels[(1, cell_y)] +=
                            self.cumulative_power_levels[(1, cell_y - 1)];
                    }
                    continue;
                }

                // For subsequent rows, we can use the previously computed sums for smaller
                // subarrays: s(cell_x, cell_y) = v(cell_x, cell_y) + s(cell_x - 1, cell_y) + s(cell_x, cell_y -1) -
                //                              - s(cell_x - 1, cell_y - 1)
                self.cumulative_power_levels[(cell_x, cell_y)] +=
                    self.cumulative_power_levels[(cell_x - 1, cell_y)];
                if cell_y > 1 {
                    self.cumulative_power_levels[(cell_x, cell_y)] +=
                        self.cumulative_power_levels[(cell_x, cell_y - 1)];
                    self.cumulative_power_levels[(cell_x, cell_y)] -=
                        self.cumulative_power_levels[(cell_x - 1, cell_y - 1)];
                }
            }
        }
    }

    fn solve_part_one(&mut self) -> String {
        let mut max_power_level = i64::MIN;
        let mut max_power_level_block = None;
        for block_x in 1..=298 {
            for block_y in 1..=298 {
                let block_power_level = self.cumulative_power_levels[(block_x + 2, block_y + 2)]
                    - self.cumulative_power_levels[(block_x - 1, block_y + 2)]
                    - self.cumulative_power_levels[(block_x + 2, block_y - 1)]
                    + self.cumulative_power_levels[(block_x - 1, block_y - 1)];
                if block_power_level > max_power_level {
                    max_power_level = block_power_level;
                    max_power_level_block = Some((block_x, block_y));
                }
            }
        }
        let (x, y) = max_power_level_block.unwrap();
        format!("{},{}", x, y)
    }

    fn solve_part_two(&mut self) -> String {
        let mut max_power_level = i64::MIN;
        let mut max_power_level_block = None;
        for block_x in 1..=300 {
            for block_y in 1..=300 {
                for block_size in 1..=300 {
                    if block_x + block_size > 301 || block_y + block_size > 301 {
                        continue;
                    }
                    let block_power_level = self.cumulative_power_levels
                        [(block_x + block_size - 1, block_y + block_size - 1)]
                        - self.cumulative_power_levels[(block_x - 1, block_y + block_size - 1)]
                        - self.cumulative_power_levels[(block_x + block_size - 1, block_y - 1)]
                        + self.cumulative_power_levels[(block_x - 1, block_y - 1)];
                    if block_power_level > max_power_level {
                        max_power_level = block_power_level;
                        max_power_level_block = Some((block_x, block_y, block_size));
                    }
                }
            }
        }
        let (x, y, size) = max_power_level_block.unwrap();
        format!("{},{},{}", x, y, size)
    }
}

pub fn solver() -> PuzzleSolver {
    PuzzleSolver {
        serial_no: 0,
        cumulative_power_levels: Array2D::filled_with(0, 301, 301),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn provided_example_1() {
        let mut s = solver();
        s.presolve("18");
        assert_eq!("33,45", s.solve_part_one());
        assert_eq!("90,269,16", s.solve_part_two());
    }

    #[test]
    fn provided_example_2() {
        let mut s = solver();
        s.presolve("42");
        assert_eq!("21,61", s.solve_part_one());
        assert_eq!("232,251,12", s.solve_part_two());
    }
}
