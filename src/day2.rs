use std::cmp;

type Gift = (u32, u32, u32);

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<Gift> {
    input.lines().map(|l| {
        let mut gift = l.trim().split('x').map(|d| d.parse().unwrap());
        (
            gift.next().unwrap(),
            gift.next().unwrap(),
            gift.next().unwrap()
        )
    }).collect()
}

#[aoc(day2, part1)]
fn part1(gifts: &[Gift]) -> u32 {
    gifts.iter().map(|&(l, w, h)| {
        let (s1, s2) = smallest_side((h, w, l));
        2 * l * w + 2 * w * h + 2 * h * l + s1 * s2
    }).sum()
}

#[aoc(day2, part2)]
fn part2(gifts: &[Gift]) -> u32 {
    gifts.iter().map(|&(l, w, h)| {
        let (s1, s2) = smallest_side((h, w, l));
        l * w * h + 2 * s1 + 2 * s2
    }).sum()
}

fn smallest_side(g: (u32, u32, u32)) -> (u32, u32) {
    if g.0 < g.1 {
        (g.0, cmp::min(g.1, g.2))
    } else {
        (g.1, cmp::min(g.0, g.2))
    }
}