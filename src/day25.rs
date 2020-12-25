use aoc_runner_derive::aoc;

const DIVISOR: u64 = 20201227u64;

struct Encrypter {
    subject: u64,
}

impl Encrypter {
    fn new(subject: u64) -> Self {
        let subject = subject % DIVISOR;
        Encrypter { subject }
    }

    fn detect_count(&self, target: u64) -> u64 {
        let mut value = 1;

        for count in 1.. {
            value = (value * self.subject) % DIVISOR;

            if value == target {
                return count;
            }
        }

        0
    }

    fn transform(&self, count: u64) -> u64 {
        let mut value = 1;

        for _ in 0..count {
            value = (value * self.subject) % DIVISOR;
        }

        value
    }
}

#[aoc(day25, part1)]
pub fn solve_part1(input: &str) -> u64 {
    let mut parts = input.lines().map(|l| l.parse().unwrap());
    let card_key: u64 = parts.next().unwrap();
    let door_key: u64 = parts.next().unwrap();

    let door_count = Encrypter::new(7).detect_count(door_key);

    Encrypter::new(card_key).transform(door_count)
}
