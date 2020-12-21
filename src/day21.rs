use aoc_runner_derive::aoc;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::combinator::{map, opt};
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair};
use nom::IResult;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

fn recipe(input: &str) -> IResult<&str, (HashSet<&str>, Option<Vec<&str>>)> {
    pair(ingredient_list, opt(allergens))(input)
}

fn ingredient_list(input: &str) -> IResult<&str, HashSet<&str>> {
    map(separated_list1(tag(" "), alpha1), HashSet::from_iter)(input)
}

fn allergens(input: &str) -> IResult<&str, Vec<&str>> {
    delimited(
        tag(" (contains "),
        separated_list1(tag(", "), alpha1),
        tag(")"),
    )(input)
}

struct Recipes<'a> {
    recipes: Vec<HashSet<&'a str>>,
    ingredients: HashSet<&'a str>,
    allergens: HashMap<&'a str, HashSet<&'a str>>,
}

fn parse_recipes(input: &str) -> Recipes<'_> {
    let recipe_list = input.lines().map(|l| recipe(l).unwrap().1);
    let mut recipes = Vec::new();
    let mut ingredients = HashSet::new();
    let mut allergens: HashMap<_, HashSet<_>> = HashMap::new();

    for (ingredient_list, allergy) in recipe_list {
        if let Some(allergen_list) = allergy {
            for allergen in allergen_list {
                allergens
                    .entry(allergen)
                    .and_modify(|set| {
                        let updated = set.intersection(&ingredient_list).copied().collect();
                        *set = updated
                    })
                    .or_insert_with(|| ingredient_list.clone());
            }
        }
        recipes.push(ingredient_list.clone());
        ingredients.extend(ingredient_list);
    }

    Recipes {
        recipes,
        ingredients,
        allergens,
    }
}

#[aoc(day21, part1)]
pub fn solve_part1(input: &str) -> usize {
    let recipes = parse_recipes(input);

    let unsafe_ingredients =
        recipes
            .allergens
            .values()
            .fold(HashSet::new(), |mut set, possibilities| {
                set.extend(possibilities);
                set
            });
    let safe_ingredients = recipes.ingredients.difference(&unsafe_ingredients);

    safe_ingredients
        .map(|ingredient| {
            recipes
                .recipes
                .iter()
                .filter(|set| set.contains(ingredient))
                .count()
        })
        .sum()
}

#[aoc(day21, part2)]
fn solve_part2(input: &str) -> String {
    let recipes = parse_recipes(input);
    let mut canonical = Vec::new();
    let mut allergens = recipes.allergens;

    loop {
        let singletons: Vec<_> = allergens
            .iter()
            .filter_map(|(allergen, ingredient_list)| {
                if ingredient_list.len() == 1 {
                    Some(*allergen)
                } else {
                    None
                }
            })
            .collect();

        for allergen in singletons {
            let ingredient = allergens
                .remove(&allergen)
                .unwrap()
                .into_iter()
                .next()
                .unwrap();

            for list in allergens.values_mut() {
                list.remove(&ingredient);
            }

            canonical.push((allergen, ingredient));
        }

        if allergens.is_empty() {
            break;
        }
    }

    canonical.sort_by(|a, b| a.0.cmp(b.0));
    let canonical: Vec<_> = canonical
        .into_iter()
        .map(|(_, ingredient)| ingredient)
        .collect();

    canonical.join(",")
}
