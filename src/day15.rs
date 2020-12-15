use aoc_runner_derive::aoc;
use std::collections::HashMap;

struct Game {
    last: u32,
    round: u32,
    used: HashMap<u32, u32>,
}

impl Game {
    fn new(input: &str) -> Self {
        let mut nums = input.split(',').map(|n| n.parse().unwrap());
        let mut game = Game {
            last: nums.next().unwrap(),
            round: 1,
            used: HashMap::new(),
        };

        for num in nums {
            game.add(num);
        }

        game
    }

    fn add(&mut self, num: u32) {
        println!("#{} - {}", self.round, self.last);
        self.used.insert(self.last, self.round);
        self.last = num;
        self.round += 1;
    }

    fn step(&mut self) {
        self.add(match self.used.get(&self.last) {
            Some(&old) => self.round - old,
            None => 0,
        });
    }
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let mut game = Game::new(input);

    while game.round != 2020 {
        game.step();
    }

    game.last
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let mut game = Game::new(input);

    while game.round != 30000000 {
        game.step();
    }

    game.last
}
