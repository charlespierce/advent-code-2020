use aoc_runner_derive::aoc;
use std::cmp::{max, min};
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl Point {
    fn new(x: i64, y: i64, z: i64, w: i64) -> Self {
        Self { x, y, z, w }
    }

    fn neighbors(&self) -> Neighbors {
        Neighbors {
            base: *self,
            dx: -1,
            dy: -1,
            dz: -1,
            dw: -2,
        }
    }
}

struct Neighbors {
    base: Point,
    dx: i64,
    dy: i64,
    dz: i64,
    dw: i64,
}

impl Iterator for Neighbors {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        self.dw += 1;
        if self.dw > 1 {
            self.dw = -1;
            self.dz += 1;
        }

        if self.dz > 1 {
            self.dz = -1;
            self.dy += 1;
        }

        if self.dy > 1 {
            self.dy = -1;
            self.dx += 1;
        }

        if self.dx > 1 {
            return None;
        }

        if self.dx == 0 && self.dy == 0 && self.dz == 0 && self.dw == 0 {
            self.dw += 1;
        }

        Some(Point::new(
            self.base.x + self.dx,
            self.base.y + self.dy,
            self.base.z + self.dz,
            self.base.w + self.dw,
        ))
    }
}

struct Game {
    active: HashSet<Point>,
    x_bounds: (i64, i64),
    y_bounds: (i64, i64),
    z_bounds: (i64, i64),
    w_bounds: (i64, i64),
}

impl Game {
    fn cycle(&mut self) {
        let mut new_active = HashSet::with_capacity(self.active.len());
        for x in self.x_bounds.0 - 1..=self.x_bounds.1 + 1 {
            for y in self.y_bounds.0 - 1..=self.y_bounds.1 + 1 {
                for z in self.z_bounds.0 - 1..=self.z_bounds.1 + 1 {
                    let point = Point::new(x, y, z, 0);
                    let active_neighbors = self.active_neighbors(point);

                    let should_insert = if self.active.contains(&point) {
                        2 <= active_neighbors && active_neighbors <= 3
                    } else {
                        active_neighbors == 3
                    };

                    if should_insert {
                        self.x_bounds =
                            (min(self.x_bounds.0, point.x), max(self.x_bounds.1, point.x));
                        self.y_bounds =
                            (min(self.y_bounds.0, point.y), max(self.y_bounds.1, point.y));
                        self.z_bounds =
                            (min(self.z_bounds.0, point.z), max(self.z_bounds.1, point.z));
                        new_active.insert(point);
                    }
                }
            }
        }

        self.active = new_active;
    }

    fn cycle_4(&mut self) {
        let mut new_active = HashSet::with_capacity(self.active.len());
        for x in self.x_bounds.0 - 1..=self.x_bounds.1 + 1 {
            for y in self.y_bounds.0 - 1..=self.y_bounds.1 + 1 {
                for z in self.z_bounds.0 - 1..=self.z_bounds.1 + 1 {
                    for w in self.w_bounds.0 - 1..=self.w_bounds.1 + 1 {
                        let point = Point::new(x, y, z, w);
                        let active_neighbors = self.active_neighbors(point);

                        let should_insert = if self.active.contains(&point) {
                            2 <= active_neighbors && active_neighbors <= 3
                        } else {
                            active_neighbors == 3
                        };

                        if should_insert {
                            self.x_bounds =
                                (min(self.x_bounds.0, point.x), max(self.x_bounds.1, point.x));
                            self.y_bounds =
                                (min(self.y_bounds.0, point.y), max(self.y_bounds.1, point.y));
                            self.z_bounds =
                                (min(self.z_bounds.0, point.z), max(self.z_bounds.1, point.z));
                            self.w_bounds =
                                (min(self.w_bounds.0, point.w), max(self.w_bounds.1, point.w));
                            new_active.insert(point);
                        }
                    }
                }
            }
        }

        self.active = new_active;
    }

    fn active_neighbors(&self, point: Point) -> usize {
        point
            .neighbors()
            .filter(|p| self.active.contains(&p))
            .count()
    }
}

impl<'a> From<&'a str> for Game {
    fn from(input: &'a str) -> Self {
        let active = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, chr)| {
                    if chr == '#' {
                        Some(Point::new(x as i64, y as i64, 0, 0))
                    } else {
                        None
                    }
                })
            })
            .flatten()
            .collect();

        let max_x = input.lines().next().unwrap().len() as i64;
        let max_y = input.lines().count() as i64;

        Game {
            active,
            x_bounds: (0, max_x),
            y_bounds: (0, max_y),
            z_bounds: (0, 0),
            w_bounds: (0, 0),
        }
    }
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut game = Game::from(input);

    game.cycle();
    game.cycle();
    game.cycle();
    game.cycle();
    game.cycle();
    game.cycle();

    game.active.len()
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut game = Game::from(input);

    game.cycle_4();
    game.cycle_4();
    game.cycle_4();
    game.cycle_4();
    game.cycle_4();
    game.cycle_4();

    game.active.len()
}
