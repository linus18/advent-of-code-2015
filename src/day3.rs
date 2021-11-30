use itertools::Itertools;

#[aoc_generator(day3)]
fn input_generator(input: &str) -> String {
    input.lines().next().expect("").to_string()
}

fn coords<I>(instructions: I) -> Vec<(i16, i16)>
    where I: Iterator<Item=char>
{
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut coords: Vec<(i16, i16)> = vec![(0, 0)];
    for c in instructions {
        match c {
            '>' => x += 1,
            '<' => x -= 1,
            '^' => y += 1,
            'v' => y -= 1,
            _ => panic!("unknown char")
        }
        coords.push((x, y));
    }
    coords
}

#[aoc(day3, part1)]
fn part1(c: &str) -> usize {
    coords(c.chars()).into_iter().unique().count()
}

#[aoc(day3, part2)]
fn part2(c: &str) -> usize {
    let mut santa = coords(c.chars().step_by(2));
    let mut robot = coords(c.chars().skip(1).step_by(2));
    santa.append(&mut robot);
    santa.into_iter().unique().count()
}

#[test]
fn test_part1() {
    assert_eq!(2, part1("^v^v^v^v^v"));
    assert_eq!(4, part1("^>v<"));
    assert_eq!(2, part1(">"));
}

#[test]
fn test_part2() {
    assert_eq!(3, part2("^v"));
    assert_eq!(3, part2("^>v<"));
    assert_eq!(11, part2("^v^v^v^v^v"));
}