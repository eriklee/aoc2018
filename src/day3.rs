use regex::*;

#[derive(Debug, PartialEq)]
pub struct Claim {
    id: u32,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

#[aoc(day3, part1, a_warmup)]
pub fn warmup(inp: &str) -> usize {
    inp.lines().count()
}

#[aoc_generator(day3, part1, w_vec)]
pub fn to_claims(inp: &str) -> Vec<Claim> {
    let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    inp.lines()
        .into_iter()
        .map(move |i| {
            let caps = re.captures(i).unwrap();
            Claim {
                id: caps[1].parse().unwrap(),
                x: caps[2].parse().unwrap(),
                y: caps[3].parse().unwrap(),
                w: caps[4].parse().unwrap(),
                h: caps[5].parse().unwrap(),
            }
        })
        .collect()
}

#[aoc(day3, part1, w_vec)]
pub fn count_overlaps(claims: &Vec<Claim>) -> u32 {
    let mut squares: [u8; 1_048_576] = [0; 1_048_576];
    let mut count = 0;

    for claim in claims {
        for i in claim.x..(claim.x + claim.w) {
            for j in claim.y..(claim.y + claim.h) {
                squares[(j * 1024 + i) as usize] += 1;
            }
        }
    }

    for s in squares.iter() {
        if *s > 1 {
            count += 1
        }
    }
    return count;
}

#[aoc(day3, part1, more_iterator)]
pub fn count_overlaps_iter(inp: &str) -> usize {
    let mut squares: [[u8; 1024]; 1024] = [[0; 1024]; 1024];
    let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    let claims = inp.lines().into_iter().map(|i| {
        let caps = re.captures(i).unwrap();
        Claim {
            id: caps[1].parse().unwrap(),
            x: caps[2].parse().unwrap(),
            y: caps[3].parse().unwrap(),
            w: caps[4].parse().unwrap(),
            h: caps[5].parse().unwrap(),
        }
    });

    for claim in claims {
        for i in claim.x..(claim.x + claim.w) {
            for j in claim.y..(claim.y + claim.h) {
                squares[j as usize][i as usize] += 1;
            }
        }
    }

    squares
        .iter()
        .flat_map(|s| s.iter())
        .filter(|&x| *x > 1)
        .count()
}

#[aoc(day3, part1, no_vec)]
pub fn count_overlaps_all(inp: &str) -> u32 {
    let mut squares: [u8; 1_000_000] = [0; 1_000_000];
    let mut count = 0;

    let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    let claims = inp.lines().into_iter().map(|i| {
        let caps = re.captures(i).unwrap();
        Claim {
            id: caps[1].parse().unwrap(),
            x: caps[2].parse().unwrap(),
            y: caps[3].parse().unwrap(),
            w: caps[4].parse().unwrap(),
            h: caps[5].parse().unwrap(),
        }
    });

    for claim in claims {
        for i in claim.x..(claim.x + claim.w) {
            for j in claim.y..(claim.y + claim.h) {
                squares[(j * 1000 + i) as usize] += 1;
            }
        }
    }

    for s in squares.iter() {
        if *s > 1 {
            count += 1
        }
    }
    return count;
}

#[aoc_generator(day3, part2)]
pub fn to_claims2(inp: &str) -> Vec<Claim> {
    let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    inp.lines()
        .into_iter()
        .map(move |i| {
            let caps = re.captures(i).unwrap();
            Claim {
                id: caps[1].parse().unwrap(),
                x: caps[2].parse().unwrap(),
                y: caps[3].parse().unwrap(),
                w: caps[4].parse().unwrap(),
                h: caps[5].parse().unwrap(),
            }
        })
        .collect()
}

#[aoc(day3, part2)]
pub fn find_unused(claims: &Vec<Claim>) -> u32 {
    let mut squares: [u8; 1_000_000] = [0; 1_000_000];

    for claim in claims {
        for i in claim.x..(claim.x + claim.w) {
            for j in claim.y..(claim.y + claim.h) {
                squares[(j * 1000 + i) as usize] += 1;
            }
        }
    }

    'outer: for claim in claims {
        for i in claim.x..(claim.x + claim.w) {
            for j in claim.y..(claim.y + claim.h) {
                if squares[(j * 1000 + i) as usize] > 1 {
                    continue 'outer;
                };
            }
        }
        return claim.id;
    }
    unreachable!();
}
