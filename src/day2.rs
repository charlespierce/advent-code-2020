use aoc_runner_derive::{aoc, aoc_generator};

pub struct Password {
    lower: u32,
    upper: u32,
    letter: char,
    pass: String,
}

#[aoc_generator(day2)]
pub fn parser(input: &str) -> Vec<Password> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split(&['-', ':', ' '][..]);
            let lower = parts.next().unwrap().parse().unwrap();
            let upper = parts.next().unwrap().parse().unwrap();
            let letter = parts.next().unwrap().chars().next().unwrap();
            parts.next();
            let pass = parts.next().unwrap().into();

            Password {
                lower,
                upper,
                letter,
                pass,
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Password]) -> u32 {
    let mut valid_count = 0;
    for password in input {
        let mut chr_count = 0;
        for chr in password.pass.chars() {
            if chr == password.letter {
                chr_count += 1;
            }
        }
        if password.lower <= chr_count && chr_count <= password.upper {
            valid_count += 1;
        }
    }

    valid_count
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Password]) -> u32 {
    let mut valid_count = 0;
    for password in input {
        let first_chr = password
            .pass
            .chars()
            .nth(password.lower as usize - 1)
            .unwrap();
        let second_chr = password
            .pass
            .chars()
            .nth(password.upper as usize - 1)
            .unwrap();
        if (first_chr == password.letter && second_chr != password.letter)
            || (first_chr != password.letter && second_chr == password.letter)
        {
            valid_count += 1;
        }
    }

    valid_count
}
