use aoc_runner_derive::aoc;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

struct Game {
    cups: VecDeque<u32>,
}

impl Game {
    fn do_move(&mut self) {
        // Take the first 3 elements after current
        self.cups.rotate_left(1);
        let removed = (
            self.cups.pop_front().unwrap(),
            self.cups.pop_front().unwrap(),
            self.cups.pop_front().unwrap(),
        );
        self.cups.rotate_right(1);

        let mut destination = self.cups[0];
        let destination_index = loop {
            if destination == 1 {
                destination = 9;
            } else {
                destination -= 1;
            }

            if let Some(next) = self
                .cups
                .iter()
                .enumerate()
                .filter_map(|(index, value)| {
                    if *value == destination {
                        Some(index)
                    } else {
                        None
                    }
                })
                .next()
            {
                break next;
            }
        };

        self.cups.rotate_left(destination_index + 1);
        self.cups.push_front(removed.2);
        self.cups.push_front(removed.1);
        self.cups.push_front(removed.0);

        self.cups.rotate_right(destination_index);
    }
}

impl<'a> From<&'a str> for Game {
    fn from(input: &'a str) -> Self {
        let cups = input.chars().map(|c| c.to_digit(10).unwrap()).collect();

        Game { cups }
    }
}

#[aoc(day23, part1)]
pub fn solve_part1(input: &str) -> String {
    let mut game: Game = input.into();

    for _ in 0..100 {
        game.do_move();
    }

    while game.cups[0] != 1 {
        game.cups.rotate_left(1);
    }

    game.cups
        .into_iter()
        .skip(1)
        .map(|d| d.to_string())
        .collect()
}

struct Cup {
    value: u64,
    next: Option<Rc<RefCell<Cup>>>,
}

impl Cup {
    fn new(value: u64) -> Self {
        Cup { value, next: None }
    }
}

struct BigGame {
    head: Rc<RefCell<Cup>>,
    links: HashMap<u64, Rc<RefCell<Cup>>>,
}

impl BigGame {
    fn do_move(&mut self) {
        // Grab the 3 removed cups
        let removed_1 = self.head.borrow().next.clone().unwrap();
        let removed_2 = removed_1.borrow().next.clone().unwrap();
        let removed_3 = removed_2.borrow().next.clone().unwrap();
        let kept = removed_3.borrow().next.clone().unwrap();
        self.head.borrow_mut().next = Some(kept);

        let mut dest_value = self.head.borrow().value;
        loop {
            if dest_value == 1 {
                dest_value = 1_000_000;
            } else {
                dest_value -= 1;
            }

            if removed_1.borrow().value != dest_value
                && removed_2.borrow().value != dest_value
                && removed_3.borrow().value != dest_value
            {
                break;
            }
        }

        let destination = self.links.get(&dest_value).unwrap();
        let next_dest = destination.borrow().next.clone().unwrap();
        destination.borrow_mut().next = Some(removed_1);
        removed_3.borrow_mut().next = Some(next_dest);

        let next = self.head.borrow().next.clone().unwrap();
        self.head = next;
    }
}

impl<'a> From<&'a str> for BigGame {
    fn from(input: &'a str) -> Self {
        let mut start_order = input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .chain(10u64..=1_000_000u64);

        let mut links = HashMap::new();
        let first = start_order.next().unwrap();
        let start = Rc::new(RefCell::new(Cup::new(first)));
        links.insert(first, start.clone());
        let mut prev = start.clone();

        for value in start_order {
            let curr = Rc::new(RefCell::new(Cup::new(value)));
            links.insert(value, curr.clone());
            prev.borrow_mut().next = Some(curr.clone());
            prev = curr;
        }

        prev.borrow_mut().next = Some(start.clone());

        BigGame { head: start, links }
    }
}

#[aoc(day23, part2)]
fn solve_part2(input: &str) -> u64 {
    let mut game: BigGame = input.into();

    for _ in 0..10_000_000 {
        game.do_move();
    }

    let one = game.links.get(&1).unwrap();
    let cup_1 = one.borrow().next.clone().unwrap();
    let cup_2 = cup_1.borrow().next.clone().unwrap();

    let operand_1 = cup_1.borrow().value;
    let operand_2 = cup_2.borrow().value;

    operand_1 * operand_2
}
