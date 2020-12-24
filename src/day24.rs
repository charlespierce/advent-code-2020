use aoc_runner_derive::aoc;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::value;
use nom::multi::many1;
use nom::IResult;
use std::collections::HashSet;

fn directions(input: &str) -> IResult<&str, Vec<Direction>> {
    many1(single_direction)(input)
}

fn single_direction(input: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::East, tag("e")),
        value(Direction::NorthEast, tag("ne")),
        value(Direction::SouthEast, tag("se")),
        value(Direction::West, tag("w")),
        value(Direction::NorthWest, tag("nw")),
        value(Direction::SouthWest, tag("sw")),
    ))(input)
}

#[derive(Clone, Copy)]
enum Direction {
    East,
    NorthEast,
    SouthEast,
    West,
    NorthWest,
    SouthWest,
}

struct Tile {
    position: (i64, i64),
}

impl Tile {
    fn new() -> Self {
        Tile { position: (0, 0) }
    }

    fn shift(&mut self, dir: Direction) {
        match dir {
            Direction::East => self.position.0 += 2,
            Direction::West => self.position.0 -= 2,
            Direction::NorthEast => {
                self.position.0 += 1;
                self.position.1 += 1;
            }
            Direction::NorthWest => {
                self.position.0 -= 1;
                self.position.1 += 1;
            }
            Direction::SouthEast => {
                self.position.0 += 1;
                self.position.1 -= 1;
            }
            Direction::SouthWest => {
                self.position.0 -= 1;
                self.position.1 -= 1;
            }
        }
    }
}

#[aoc(day24, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut black_tiles = HashSet::new();
    for line in input.lines() {
        let mut tile = Tile::new();

        for dir in directions(line).unwrap().1 {
            tile.shift(dir);
        }

        if !black_tiles.remove(&tile.position) {
            black_tiles.insert(tile.position);
        }
    }

    black_tiles.len()
}

struct Floor {
    black_tiles: HashSet<(i64, i64)>,
    x_bounds: (i64, i64),
    y_bounds: (i64, i64),
}

impl Floor {
    fn new() -> Self {
        Floor {
            black_tiles: HashSet::new(),
            x_bounds: (0, 0),
            y_bounds: (0, 0),
        }
    }

    fn flip_tile(&mut self, position: (i64, i64)) {
        if !self.black_tiles.remove(&position) {
            self.black_tiles.insert(position);

            if position.0 < self.x_bounds.0 {
                self.x_bounds.0 = position.0;
            } else if position.0 > self.x_bounds.1 {
                self.x_bounds.1 = position.0;
            }

            if position.1 < self.y_bounds.0 {
                self.y_bounds.0 = position.1;
            } else if position.1 > self.y_bounds.1 {
                self.y_bounds.1 = position.1;
            }
        }
    }

    fn run_day(&mut self) {
        let mut new_floor = HashSet::new();
        let mut new_x_bounds = self.x_bounds;
        let mut new_y_bounds = self.y_bounds;

        for x in self.x_bounds.0 - 2..self.x_bounds.1 + 2 {
            for y in self.y_bounds.0 - 2..self.y_bounds.1 + 2 {
                let tile = (x, y);

                let black_adjacent = self.black_adjacent(tile);
                let should_insert = if self.black_tiles.contains(&tile) {
                    black_adjacent == 1 || black_adjacent == 2
                } else {
                    black_adjacent == 2
                };

                if should_insert {
                    new_floor.insert(tile);
                    if tile.0 < new_x_bounds.0 {
                        new_x_bounds.0 = tile.0;
                    } else if tile.0 > new_x_bounds.1 {
                        new_x_bounds.1 = tile.0;
                    }

                    if tile.1 < new_y_bounds.0 {
                        new_y_bounds.0 = tile.1;
                    } else if tile.1 > new_y_bounds.1 {
                        new_y_bounds.1 = tile.1;
                    }
                }
            }
        }

        self.black_tiles = new_floor;
        self.x_bounds = new_x_bounds;
        self.y_bounds = new_y_bounds;
    }

    fn black_adjacent(&self, tile: (i64, i64)) -> usize {
        neighbors(tile)
            .filter(|t| self.black_tiles.contains(&t))
            .count()
    }
}

fn neighbors(tile: (i64, i64)) -> impl Iterator<Item = (i64, i64)> {
    vec![
        (tile.0 - 2, tile.1),
        (tile.0 - 1, tile.1 - 1),
        (tile.0 - 1, tile.1 + 1),
        (tile.0 + 1, tile.1 - 1),
        (tile.0 + 1, tile.1 + 1),
        (tile.0 + 2, tile.1),
    ]
    .into_iter()
}

#[aoc(day24, part2)]
fn solve_part2(input: &str) -> usize {
    let mut floor = Floor::new();
    for line in input.lines() {
        let mut tile = Tile::new();

        for dir in directions(line).unwrap().1 {
            tile.shift(dir);
        }

        floor.flip_tile(tile.position);
    }

    for _ in 0..100 {
        floor.run_day();
    }

    floor.black_tiles.len()
}
