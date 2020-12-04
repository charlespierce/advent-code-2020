use aoc_runner_derive::{aoc, aoc_generator};

pub struct Passport {
    birth_year: Option<u16>,
    issue_year: Option<u16>,
    expiration_year: Option<u16>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    fn new() -> Self {
        Passport {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            passport_id: None,
            country_id: None,
        }
    }

    fn is_valid_primitive(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }

    fn is_valid_advanced(&self) -> bool {
        self.valid_birth_year()
            && self.valid_issue_year()
            && self.valid_expiration_year()
            && self.valid_height()
            && self.valid_hair_color()
            && self.valid_eye_color()
            && self.valid_passport_id()
    }

    fn birth_year(&mut self, year: u16) {
        self.birth_year = Some(year);
    }

    fn valid_birth_year(&self) -> bool {
        match self.birth_year {
            Some(year) => 1920 <= year && year <= 2002,
            None => false,
        }
    }

    fn issue_year(&mut self, year: u16) {
        self.issue_year = Some(year);
    }

    fn valid_issue_year(&self) -> bool {
        match self.issue_year {
            Some(year) => 2010 <= year && year <= 2020,
            None => false,
        }
    }

    fn expiration_year(&mut self, year: u16) {
        self.expiration_year = Some(year);
    }

    fn valid_expiration_year(&self) -> bool {
        match self.expiration_year {
            Some(year) => 2020 <= year && year <= 2030,
            None => false,
        }
    }

    fn height(&mut self, height: String) {
        self.height = Some(height);
    }

    fn valid_height(&self) -> bool {
        match self.height.as_ref() {
            Some(height) => {
                if let Some(h) = height.strip_suffix("cm").and_then(|v| v.parse::<u8>().ok()) {
                    return 150 <= h && h <= 193;
                }

                if let Some(h) = height.strip_suffix("in").and_then(|v| v.parse::<u8>().ok()) {
                    return 59 <= h && h <= 76;
                }

                false
            }
            None => false,
        }
    }

    fn hair_color(&mut self, color: String) {
        self.hair_color = Some(color);
    }

    fn valid_hair_color(&self) -> bool {
        self.hair_color
            .as_ref()
            .and_then(|color| color.strip_prefix('#'))
            .map(|color| color.chars().all(|c| c.is_digit(16)))
            .unwrap_or(false)
    }

    fn eye_color(&mut self, color: String) {
        self.eye_color = Some(color);
    }

    fn valid_eye_color(&self) -> bool {
        matches!(
            self.eye_color.as_deref(),
            Some("amb")
                | Some("blu")
                | Some("brn")
                | Some("gry")
                | Some("grn")
                | Some("hzl")
                | Some("oth")
        )
    }

    fn passport_id(&mut self, id: String) {
        self.passport_id = Some(id);
    }

    fn valid_passport_id(&self) -> bool {
        self.passport_id
            .as_ref()
            .map(|pid| pid.chars().all(|c| c.is_digit(10)) && pid.len() == 9)
            .unwrap_or(false)
    }

    fn country_id(&mut self, id: String) {
        self.country_id = Some(id);
    }
}

#[aoc_generator(day4)]
pub fn parser(input: &str) -> Vec<Passport> {
    let data = input.lines().flat_map(|l| l.split(' '));
    let mut passport_list = Vec::new();
    let mut passport = Passport::new();

    for item in data {
        if item.is_empty() {
            let mut next = Passport::new();
            std::mem::swap(&mut passport, &mut next);
            passport_list.push(next);
        } else {
            let key = &item[0..3];
            let value = &item[4..];
            match key {
                "byr" => passport.birth_year(value.parse().unwrap()),
                "iyr" => passport.issue_year(value.parse().unwrap()),
                "eyr" => passport.expiration_year(value.parse().unwrap()),
                "hgt" => passport.height(value.into()),
                "hcl" => passport.hair_color(value.into()),
                "ecl" => passport.eye_color(value.into()),
                "pid" => passport.passport_id(value.into()),
                "cid" => passport.country_id(value.into()),
                e => panic!("Unknown passport data: {}", e),
            }
        }
    }

    passport_list
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Passport]) -> usize {
    input.iter().filter(|p| p.is_valid_primitive()).count()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[Passport]) -> usize {
    input.iter().filter(|p| p.is_valid_advanced()).count()
}
