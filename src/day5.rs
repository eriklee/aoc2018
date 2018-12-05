// All ASCII all the time!
#[aoc(day5, part1)]
fn part1(nums: &[u8]) -> usize {
    dedup(nums, 0).len()
}

#[aoc(day5, part2)]
fn part2(inp: &[u8]) -> usize {
    (('A' as u8)..('Z' as u8))
        .map(|bad| dedup(inp, bad).len())
        .min()
        .unwrap_or(0)
}

fn dedup(nums: &[u8], bad: u8) -> Vec<u8> {
    let mut res = Vec::new();
    for &y in nums.into_iter().filter(|&x| x & (255 - 32) != bad) {
        if let Some(&x) = res.last() {
            // the difference between lowercase and capital ascii
            // characters is 1 bit. It's almost as though someone
            // thought it through and made good decisions
            if (x ^ y) == 32 {
                res.pop();
            } else {
                res.push(y);
            }
        } else {
            res.push(y)
        }
    }
    res
}

#[aoc(day5, part2, maybe_cleaner)]
fn part2_2(inp: &[u8]) -> usize {
    (('A' as u8)..('Z' as u8))
        .map(|bad| dedup2(inp, bad))
        .min()
        .unwrap_or(0)
}

fn dedup2(nums: &[u8], bad: u8) -> usize {
    let mut res = Vec::new();
    for &y in nums.into_iter().filter(|&x| x & (255 - 32) != bad) {
        if res.last().map(|&l| l ^ y == 32).unwrap_or(false) {
            res.pop();
        } else {
            res.push(y);
        }
    }
    res.len()
}
