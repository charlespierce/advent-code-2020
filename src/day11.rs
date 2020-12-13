use aoc_runner_derive::{aoc, aoc_generator};
use std::fmt;

#[derive(PartialEq, Copy, Clone)]
enum Spot {
    Floor,
    Empty,
    Occupied,
}

impl fmt::Display for Spot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Spot::Floor => ".",
            Spot::Empty => "L",
            Spot::Occupied => "#",
        })
    }
}

#[derive(PartialEq)]
pub struct Lobby {
    layout: Vec<Vec<Spot>>,
}

impl fmt::Display for Lobby {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.layout {
            for value in row {
                write!(f, "{}", value)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Lobby {
    fn count_surrounding(&self, row: usize, col: usize) -> u8 {
        let lrow = row.saturating_sub(1);
        let urow = if row + 1 >= self.layout.len() {
            row
        } else {
            row + 1
        };
        let lcol = col.saturating_sub(1);
        let ucol = if col + 1 >= self.layout[0].len() {
            col
        } else {
            col + 1
        };

        let mut matching = 0;
        for r in lrow..=urow {
            for c in lcol..=ucol {
                if self.layout[r][c] == Spot::Occupied && !(r == row && c == col) {
                    matching += 1;
                }
            }
        }

        matching
    }

    fn count_visible(&self, row: usize, col: usize) -> u8 {
        self.visible_direction(row, col, (-1, -1))
            + self.visible_direction(row, col, (-1, 0))
            + self.visible_direction(row, col, (-1, 1))
            + self.visible_direction(row, col, (0, -1))
            + self.visible_direction(row, col, (0, 1))
            + self.visible_direction(row, col, (1, -1))
            + self.visible_direction(row, col, (1, 0))
            + self.visible_direction(row, col, (1, 1))
    }

    fn visible_direction(&self, row: usize, col: usize, direction: (i8, i8)) -> u8 {
        let mut row = row as i8;
        let mut col = col as i8;

        let max_row = self.layout.len() as i8;
        let max_col = self.layout[0].len() as i8;

        loop {
            row += direction.0;
            col += direction.1;

            if row < 0 || col < 0 || row >= max_row || col >= max_col {
                return 0;
            }

            match self.layout[row as usize][col as usize] {
                Spot::Occupied => return 1,
                Spot::Empty => return 0,
                Spot::Floor => {}
            }
        }
    }

    fn count_occupied(&self) -> usize {
        self.layout
            .iter()
            .map(|row| row.iter().filter(|col| **col == Spot::Occupied).count())
            .sum()
    }

    fn step(&self) -> Lobby {
        let layout = self
            .layout
            .iter()
            .enumerate()
            .map(|(row, data)| {
                data.iter()
                    .enumerate()
                    .map(|(col, spot)| match spot {
                        Spot::Floor => Spot::Floor,
                        Spot::Empty => {
                            if self.count_surrounding(row, col) == 0 {
                                Spot::Occupied
                            } else {
                                Spot::Empty
                            }
                        }
                        Spot::Occupied => {
                            if self.count_surrounding(row, col) >= 4 {
                                Spot::Empty
                            } else {
                                Spot::Occupied
                            }
                        }
                    })
                    .collect()
            })
            .collect();

        Lobby { layout }
    }

    fn step_part2(&self) -> Lobby {
        let layout = self
            .layout
            .iter()
            .enumerate()
            .map(|(row, data)| {
                data.iter()
                    .enumerate()
                    .map(|(col, spot)| match spot {
                        Spot::Floor => Spot::Floor,
                        Spot::Empty => {
                            if self.count_visible(row, col) == 0 {
                                Spot::Occupied
                            } else {
                                Spot::Empty
                            }
                        }
                        Spot::Occupied => {
                            if self.count_visible(row, col) >= 5 {
                                Spot::Empty
                            } else {
                                Spot::Occupied
                            }
                        }
                    })
                    .collect()
            })
            .collect();

        Lobby { layout }
    }
}

#[aoc_generator(day11)]
pub fn parser(input: &str) -> Lobby {
    let layout = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|chr| match chr {
                    'L' => Spot::Empty,
                    '#' => Spot::Occupied,
                    '.' => Spot::Floor,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    Lobby { layout }
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &Lobby) -> usize {
    let mut current = input.step();

    loop {
        let next_step = current.step();
        if current == next_step {
            return current.count_occupied();
        }
        current = next_step;
    }
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &Lobby) -> usize {
    let mut current = input.step_part2();

    loop {
        let next_step = current.step_part2();
        if current == next_step {
            return current.count_occupied();
        }
        current = next_step;
    }
}
