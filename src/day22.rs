use aoc_runner_derive::aoc;
use std::collections::HashSet;
use std::collections::VecDeque;

struct Player {
    deck: VecDeque<usize>,
}

impl<'a> From<&'a str> for Player {
    fn from(input: &'a str) -> Self {
        let cards = input.lines().skip(1).map(|l| l.parse().unwrap());

        Player {
            deck: cards.collect(),
        }
    }
}

struct Game {
    player_1: Player,
    player_2: Player,
}

impl Game {
    fn round(&mut self) {
        let card_1 = self.player_1.deck.pop_front().unwrap();
        let card_2 = self.player_2.deck.pop_front().unwrap();

        if card_1 > card_2 {
            self.player_1.deck.push_back(card_1);
            self.player_1.deck.push_back(card_2);
        } else {
            self.player_2.deck.push_back(card_2);
            self.player_2.deck.push_back(card_1);
        }
    }

    fn run(mut self) -> VecDeque<usize> {
        loop {
            if self.player_1.deck.is_empty() {
                return self.player_2.deck;
            } else if self.player_2.deck.is_empty() {
                return self.player_1.deck;
            }

            self.round();
        }
    }
}

impl<'a> From<&'a str> for Game {
    fn from(input: &'a str) -> Self {
        let mut decks = input.split("\n\n");
        let (player_1, player_2) = (decks.next().unwrap().into(), decks.next().unwrap().into());

        Game { player_1, player_2 }
    }
}

#[aoc(day22, part1)]
pub fn solve_part1(input: &str) -> usize {
    let game: Game = input.into();
    let winner = game.run();

    winner
        .into_iter()
        .rev()
        .enumerate()
        .fold(0, |sum, (index, value)| sum + ((index + 1) * value))
}

struct RecursiveGame {
    cache: HashSet<String>,
    player_1: Player,
    player_2: Player,
}

enum Winner {
    Player1,
    Player2,
}

impl RecursiveGame {
    fn cache_key(&self) -> String {
        let player_1: String = self
            .player_1
            .deck
            .iter()
            .map(|n| format!("{}-", n))
            .collect();
        let player_2: String = self
            .player_2
            .deck
            .iter()
            .map(|n| format!("{}-", n))
            .collect();

        format!("P1_{}P2_{}", player_1, player_2)
    }

    fn round(&mut self) {
        let card_1 = self.player_1.deck.pop_front().unwrap();
        let card_2 = self.player_2.deck.pop_front().unwrap();

        let winner = if self.player_1.deck.len() >= card_1 && self.player_2.deck.len() >= card_2 {
            let deck_1 = self.player_1.deck.iter().take(card_1).copied().collect();
            let deck_2 = self.player_2.deck.iter().take(card_2).copied().collect();

            let mut sub_game = RecursiveGame {
                cache: HashSet::new(),
                player_1: Player { deck: deck_1 },
                player_2: Player { deck: deck_2 },
            };

            sub_game.run()
        } else if card_1 > card_2 {
            Winner::Player1
        } else {
            Winner::Player2
        };

        match winner {
            Winner::Player1 => {
                self.player_1.deck.push_back(card_1);
                self.player_1.deck.push_back(card_2);
            }
            Winner::Player2 => {
                self.player_2.deck.push_back(card_2);
                self.player_2.deck.push_back(card_1);
            }
        }
    }

    fn run(&mut self) -> Winner {
        loop {
            let key = self.cache_key();
            if self.cache.contains(&key) {
                return Winner::Player1;
            } else {
                self.cache.insert(key);
            }

            self.round();

            if self.player_1.deck.is_empty() {
                return Winner::Player2;
            } else if self.player_2.deck.is_empty() {
                return Winner::Player1;
            }
        }
    }
}

impl<'a> From<&'a str> for RecursiveGame {
    fn from(input: &'a str) -> Self {
        let mut players = input.split("\n\n");
        let (player_1, player_2) = (
            players.next().unwrap().into(),
            players.next().unwrap().into(),
        );

        RecursiveGame {
            cache: HashSet::new(),
            player_1,
            player_2,
        }
    }
}

#[aoc(day22, part2)]
fn solve_part2(input: &str) -> usize {
    let mut game: RecursiveGame = input.into();

    let winning_deck = match game.run() {
        Winner::Player1 => game.player_1.deck,
        Winner::Player2 => game.player_2.deck,
    };

    winning_deck
        .into_iter()
        .rev()
        .enumerate()
        .fold(0, |sum, (index, value)| sum + ((index + 1) * value))
}
