use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::iter::once;

#[aoc_generator(day10)]
pub fn parser(input: &str) -> Vec<u32> {
    let mut adapters: Vec<_> = input.lines().map(|line| line.parse().unwrap()).collect();
    adapters.push(0);
    adapters.sort_unstable();

    adapters
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    let device = *input.last().unwrap() + 3;
    let (diff_1, diff_3, _) =
        input
            .iter()
            .chain(once(&device))
            .fold((0, 0, 0), |(diff_1, diff_3, curr), value| {
                match value - curr {
                    1 => (diff_1 + 1, diff_3, *value),
                    3 => (diff_1, diff_3 + 1, *value),
                    _ => (diff_1, diff_3, *value),
                }
            });

    diff_1 * diff_3
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[u32]) -> u64 {
    let mut cache = HashMap::new();
    count_possibilities(input, 0, &mut cache)
}

fn count_possibilities(adapters: &[u32], current: usize, cache: &mut HashMap<usize, u64>) -> u64 {
    if current == adapters.len() - 1 {
        return 1;
    }

    if let Some(value) = cache.get(&current) {
        return *value;
    }

    let mut possibilities = 0;

    for index in (current + 1)..adapters.len() {
        if adapters[index] > adapters[current] + 3 {
            break;
        }
        possibilities += count_possibilities(adapters, index, cache);
    }

    cache.insert(current, possibilities);

    possibilities
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let data = "16
10
15
5
1
11
7
19
6
12
4";
        let parsed = parser(data);
        assert_eq!(solve_part2(&parsed), 8)
    }

    #[test]
    fn test_part2_longer() {
        let data = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        let parsed = parser(data);
        assert_eq!(solve_part2(&parsed), 19208);
    }
}
