#[aoc(day1, part1)]
pub fn to_vec(inp: &str) -> i32 {
    inp.lines().map(|n| n.parse::<i32>().unwrap()).sum()
}

#[aoc(day1, part2)]
pub fn do_part_2(inp: &str) -> i32 {
    use std::collections::HashSet;
    let mut freqs = HashSet::new();

    inp.lines()
        .cycle()
        .map(|n| n.parse::<i32>().unwrap())
        .scan(0, |sum, i| {
            *sum += i;
            Some(*sum)
        })
        .find(|freq| !freqs.insert(*freq))
        .unwrap()
}
