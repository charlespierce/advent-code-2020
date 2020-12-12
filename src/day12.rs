use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Copy, Clone)]
pub enum Action {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

#[aoc_generator(day12)]
pub fn parser(input: &str) -> Vec<Action> {
    input
        .lines()
        .map(|line| {
            let amount = line[1..].parse().unwrap();
            match &line[0..1] {
                "N" => Action::North(amount),
                "S" => Action::South(amount),
                "E" => Action::East(amount),
                "W" => Action::West(amount),
                "L" => Action::Left(amount),
                "R" => Action::Right(amount),
                "F" => Action::Forward(amount),
                _ => unreachable!(),
            }
        })
        .collect()
}

struct Ship {
    position: (i32, i32),
    heading: (i32, i32),
}

impl Ship {
    fn new() -> Self {
        Ship {
            position: (0, 0),
            heading: (1, 0),
        }
    }

    fn act(&mut self, action: Action) {
        match action {
            Action::North(amount) => {
                self.position = (self.position.0, self.position.1 + amount);
            }
            Action::South(amount) => {
                self.position = (self.position.0, self.position.1 - amount);
            }
            Action::East(amount) => {
                self.position = (self.position.0 + amount, self.position.1);
            }
            Action::West(amount) => {
                self.position = (self.position.0 - amount, self.position.1);
            }
            Action::Left(amount) => match amount {
                90 => {
                    self.heading = (-self.heading.1, self.heading.0);
                }
                180 => {
                    self.heading = (-self.heading.0, -self.heading.1);
                }
                270 => {
                    self.heading = (self.heading.1, -self.heading.0);
                }
                _ => {}
            },
            Action::Right(amount) => match amount {
                90 => {
                    self.heading = (self.heading.1, -self.heading.0);
                }
                180 => {
                    self.heading = (-self.heading.0, -self.heading.1);
                }
                270 => {
                    self.heading = (-self.heading.1, self.heading.0);
                }
                _ => {}
            },
            Action::Forward(amount) => {
                let dx = self.heading.0 * amount;
                let dy = self.heading.1 * amount;
                self.position = (self.position.0 + dx, self.position.1 + dy);
            }
        }
    }

    fn distance(&self) -> i32 {
        self.position.0.abs() + self.position.1.abs()
    }
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &[Action]) -> i32 {
    input
        .iter()
        .fold(Ship::new(), |mut ship, &action| {
            ship.act(action);
            ship
        })
        .distance()
}

struct ShipWaypoint {
    position: (i32, i32),
    waypoint: (i32, i32),
}

impl ShipWaypoint {
    fn new() -> Self {
        Self {
            position: (0, 0),
            waypoint: (10, 1),
        }
    }

    fn act(&mut self, action: Action) {
        match action {
            Action::North(amount) => {
                self.waypoint = (self.waypoint.0, self.waypoint.1 + amount);
            }
            Action::South(amount) => {
                self.waypoint = (self.waypoint.0, self.waypoint.1 - amount);
            }
            Action::East(amount) => {
                self.waypoint = (self.waypoint.0 + amount, self.waypoint.1);
            }
            Action::West(amount) => {
                self.waypoint = (self.waypoint.0 - amount, self.waypoint.1);
            }
            Action::Left(amount) => match amount {
                90 => {
                    self.waypoint = (-self.waypoint.1, self.waypoint.0);
                }
                180 => {
                    self.waypoint = (-self.waypoint.0, -self.waypoint.1);
                }
                270 => {
                    self.waypoint = (self.waypoint.1, -self.waypoint.0);
                }
                _ => {}
            },
            Action::Right(amount) => match amount {
                90 => {
                    self.waypoint = (self.waypoint.1, -self.waypoint.0);
                }
                180 => {
                    self.waypoint = (-self.waypoint.0, -self.waypoint.1);
                }
                270 => {
                    self.waypoint = (-self.waypoint.1, self.waypoint.0);
                }
                _ => {}
            },
            Action::Forward(amount) => {
                let dx = self.waypoint.0 * amount;
                let dy = self.waypoint.1 * amount;
                self.position = (self.position.0 + dx, self.position.1 + dy);
            }
        }
    }

    fn distance(&self) -> i32 {
        self.position.0.abs() + self.position.1.abs()
    }
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &[Action]) -> i32 {
    input
        .iter()
        .fold(ShipWaypoint::new(), |mut ship, &action| {
            ship.act(action);
            ship
        })
        .distance()
}
