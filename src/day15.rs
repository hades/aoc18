use std::collections::{BTreeSet, HashMap};

use log::debug;

use crate::solver::Solver;

#[derive(Default)]
pub struct PuzzleSolver {
    goblins: Vec<(usize, usize)>,
    elves: Vec<(usize, usize)>,
    spaces: Vec<(usize, usize)>,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Debug, Hash)]
struct Coords {
    y: usize,
    x: usize,
}

impl Coords {
    fn neighbours(&self) -> [Coords; 4] {
        [
            Coords {
                x: self.x,
                y: self.y - 1,
            },
            Coords {
                x: self.x - 1,
                y: self.y,
            },
            Coords {
                x: self.x + 1,
                y: self.y,
            },
            Coords {
                x: self.x,
                y: self.y + 1,
            },
        ]
    }

    fn is_free_space<T>(&self, positions: &HashMap<Coords, Option<T>>) -> bool {
        matches!(positions.get(self), Some(None))
    }

    /// Finds the set of the closest grid points that are accessible and satisfy
    /// the given predicate.
    fn find<T, F: FnMut(&Coords) -> bool>(
        &self,
        neighbours: &HashMap<Coords, Vec<Coords>>,
        positions: &HashMap<Coords, Option<T>>,
        mut predicate: F,
        limit: usize,
    ) -> (usize, BTreeSet<(Coords, Option<Coords>)>) {
        let mut front = vec![(self.clone(), None::<Coords>)];
        let mut visited: BTreeSet<Coords> = BTreeSet::new();
        let mut distance = 0;
        while !front.is_empty() && !front.iter().any(|(c, _)| predicate(c)) {
            if distance > limit {
                return (usize::MAX, BTreeSet::new());
            }
            let mut new_front = vec![];
            for (c, first_step) in front {
                visited.insert(c.clone());
                new_front.extend(
                    neighbours[&c]
                        .iter()
                        .filter(|c| c.is_free_space(positions) && !visited.contains(c))
                        .map(|c| match &first_step {
                            None => (c.clone(), Some(c.clone())),
                            Some(first_step) => (c.clone(), Some(first_step.clone())),
                        }),
                );
            }
            front = new_front;
            distance += 1;
        }
        (
            distance,
            front.into_iter().filter(|(c, _)| predicate(c)).collect(),
        )
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Faction {
    Elf,
    Goblin,
}

#[derive(Debug)]
struct Unit {
    faction: Faction,
    hp: i16,
}

impl PuzzleSolver {
    fn simulate_battle(&self, elf_attack_power: i16) -> (Faction, String) {
        let mut units = vec![];
        let mut positions: HashMap<Coords, Option<usize>> = HashMap::new();
        for (x, y) in self.elves.iter().copied() {
            positions.insert(Coords { x, y }, Some(units.len()));
            units.push(Unit {
                faction: Faction::Elf,
                hp: 200,
            });
        }
        for (x, y) in self.goblins.iter().copied() {
            positions.insert(Coords { x, y }, Some(units.len()));
            units.push(Unit {
                faction: Faction::Goblin,
                hp: 200,
            });
        }
        for (x, y) in self.spaces.iter().copied() {
            positions.entry(Coords { x, y }).or_default();
        }
        let neighbours = {
            let mut neighbours: HashMap<Coords, Vec<Coords>> = HashMap::new();
            for position in positions.keys() {
                let neighbours_for_position: Vec<_> = position
                    .neighbours()
                    .into_iter()
                    .filter(|n| positions.contains_key(n))
                    .collect();
                neighbours.insert(position.clone(), neighbours_for_position);
            }
            neighbours
        };
        let mut rounds = 0;
        'outerloop: loop {
            debug!("{rounds}");
            let mut act_order: Vec<_> = positions
                .iter()
                .filter_map(|(coords, unit_ref)| {
                    unit_ref.map(|unit_ref| (coords.clone(), unit_ref))
                })
                .collect();
            act_order.sort();
            let act_order = act_order;
            for (mut coords, unit_ref) in act_order {
                if units[unit_ref].hp <= 0 {
                    continue;
                }
                let faction = units[unit_ref].faction;
                let enemies_alive = units
                    .iter()
                    .filter(|u| u.faction != faction && u.hp > 0)
                    .count();
                if enemies_alive == 0 {
                    break 'outerloop;
                }
                // Step 1: check if we're standing next to an enemy.
                let standing_next_to_enemy = coords
                    .neighbours()
                    .into_iter()
                    .flat_map(|c| positions.get(&c))
                    .flatten()
                    .any(|enemy_ref| units[*enemy_ref].faction != faction);
                if !standing_next_to_enemy {
                    // Step 2: find all free spaces next to all enemies.
                    let free_spaces_next_to_enemies: BTreeSet<_> = positions
                        .iter()
                        .filter_map(|(coords, unit_ref)| {
                            unit_ref.and_then(|enemy_ref| {
                                if units[enemy_ref].faction != faction {
                                    Some(coords)
                                } else {
                                    None
                                }
                            })
                        })
                        .flat_map(|c| c.neighbours())
                        .filter(|c| c.is_free_space(&positions))
                        .collect();

                    // Step 3: choose the closest free space to move to.
                    let (_, mut spaces_to_move_to) = if free_spaces_next_to_enemies.is_empty() {
                        (usize::MAX, BTreeSet::new())
                    } else {
                        coords.find(
                            &neighbours,
                            &positions,
                            |c| free_spaces_next_to_enemies.contains(c),
                            usize::MAX,
                        )
                    };

                    if let Some(space_to_move_to) = spaces_to_move_to.pop_first() {
                        // Step 4: move towards that free space.
                        let next_step = space_to_move_to.1.unwrap();
                        *positions.get_mut(&next_step).unwrap() = Some(unit_ref);
                        *positions.get_mut(&coords).unwrap() = None;
                        //debug!("{coords:?} -> {next_step:?}");
                        coords = next_step;
                    }
                }
                // Step 5: check again if we're standing next to an enemy (we might have moved).
                let mut targets: Vec<_> = coords
                    .neighbours()
                    .into_iter()
                    .filter(|c| match positions.get(c) {
                        Some(Some(enemy_ref)) => units[*enemy_ref].faction != faction,
                        _ => false,
                    })
                    .collect();
                targets.sort_by_key(|c| (units[positions[c].unwrap()].hp, c.clone()));
                if let Some(enemy_position) = targets.first() {
                    //debug!("{coords:?} => {enemy_position:?}");
                    let enemy_ref = positions[enemy_position].unwrap();
                    let attack_power = match faction {
                        Faction::Elf => elf_attack_power,
                        Faction::Goblin => 3,
                    };
                    units[enemy_ref].hp -= attack_power;
                    if units[enemy_ref].hp <= 0 {
                        *positions.get_mut(enemy_position).unwrap() = None;
                    }
                }
            }
            rounds += 1;
        }
        let outcome = (rounds
            * units
                .iter()
                .map(|unit| if unit.hp > 0 { unit.hp as i32 } else { 0 })
                .sum::<i32>())
        .to_string();
        let winning_faction = if units
            .iter()
            .filter(|u| u.faction == Faction::Elf)
            .all(|u| u.hp > 0)
        {
            Faction::Elf
        } else {
            Faction::Goblin
        };
        (winning_faction, outcome)
    }
}

impl Solver for PuzzleSolver {
    fn presolve(&mut self, input: &str) {
        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                match ch {
                    '.' => {
                        self.spaces.push((x, y));
                    }
                    'G' => {
                        self.spaces.push((x, y));
                        self.goblins.push((x, y));
                    }
                    'E' => {
                        self.spaces.push((x, y));
                        self.elves.push((x, y));
                    }
                    _ => {}
                }
            }
        }
    }

    fn solve_part_one(&mut self) -> String {
        self.simulate_battle(3).1
    }

    fn solve_part_two(&mut self) -> String {
        let mut l = 3;
        let mut r = 201;
        let mut outcomes: HashMap<_, _> = HashMap::new();
        while r > l + 1 {
            let m = (r + l) / 2;
            let (winning_faction, outcome) = self.simulate_battle(m);
            outcomes.insert(m, outcome);
            match winning_faction {
                Faction::Elf => {
                    r = m;
                }
                Faction::Goblin => {
                    l = m;
                }
            }
        }
        outcomes.remove(&r).unwrap()
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
            "#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######
",
        );
        assert_eq!("27730", s.solve_part_one());
        assert_eq!("4988", s.solve_part_two());
    }

    #[test]
    fn provided_example_2() {
        let mut s = solver();
        s.presolve(
            "#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######
",
        );
        assert_eq!("36334", s.solve_part_one());
    }

    #[test]
    fn provided_example_3() {
        let mut s = solver();
        s.presolve(
            "#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######
",
        );
        assert_eq!("39514", s.solve_part_one());
        assert_eq!("31284", s.solve_part_two());
    }

    #[test]
    fn provided_example_4() {
        let mut s = solver();
        s.presolve(
            "#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######
",
        );
        assert_eq!("27755", s.solve_part_one());
        assert_eq!("3478", s.solve_part_two());
    }

    #[test]
    fn provided_example_5() {
        let mut s = solver();
        s.presolve(
            "#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######
",
        );
        assert_eq!("28944", s.solve_part_one());
        assert_eq!("6474", s.solve_part_two());
    }

    #[test]
    fn provided_example_6() {
        let mut s = solver();
        s.presolve(
            "#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########
",
        );
        assert_eq!("18740", s.solve_part_one());
        assert_eq!("1140", s.solve_part_two());
    }
}
