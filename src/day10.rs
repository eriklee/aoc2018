#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    v_x: i32,
    v_y: i32,
}

#[aoc_generator(day10)]
fn parse_points(inp: &str) -> Vec<Point> {
    use regex::*;
    // position=<-3,  6> velocity=< 2, -1>
    let re = Regex::new(r"^position=<\s*([\d-]+),\s*([\d-]+)> velocity=<\s*([\d-]+),\s*([\d-]+)>$")
        .expect("invalid regex");
    inp.lines()
        .map(move |i| {
            let caps = re.captures(i).expect("No captures in line");
            Point {
                x: caps[1].parse().expect("Couldn't parse x"),
                y: caps[2].parse().unwrap(),
                v_x: caps[3].parse().unwrap(),
                v_y: caps[4].parse().unwrap(),
            }
        })
        .collect()
}

#[aoc(day10, part1)]
fn part1(points: &[Point]) -> String {
    let mut score: usize = std::usize::MAX;
    let mut points = points.to_owned();
    let mut step_count = 0;

    loop {
        iterate_points(&mut points);
        step_count += 1;
        let new_score = calc_score(&points);
        if score < new_score {
            step_count -= 1; // We've gone a step too far, and are now stepping back
            println!("steps: {}, score: {}", step_count, score);
            uniterate_points(&mut points);
            show_points(&points);
            break;
        } else {
            score = new_score;
        }
    }
    "reclrnze".to_owned()
}

fn iterate_points(points: &mut [Point]) {
    for p in points.iter_mut() {
        p.x += p.v_x;
        p.y += p.v_y;
    }
}

fn uniterate_points(points: &mut [Point]) {
    for p in points.iter_mut() {
        p.x -= p.v_x;
        p.y -= p.v_y;
    }
}

fn calc_score(points: &[Point]) -> usize {
    use std::cmp::{max, min};
    let (mut min_y, mut max_y) = (std::i32::MAX, std::i32::MIN);

    for p in points {
        min_y = min(min_y, p.y);
        max_y = max(max_y, p.y);
    }
    (max_y - min_y) as usize
}

fn show_points(points: &[Point]) {
    let mut sorted: Vec<(i32, i32)> = points.iter().map(|p| (p.y, p.x)).collect();
    sorted.sort();
    sorted.dedup();
    let min_x = points.iter().map(|p| p.x).min().unwrap_or(0);
    let mut last_y = 0;
    let mut last_x = min_x;
    for (y, x) in sorted.iter() {
        if *y != last_y {
            last_y = *y;
            last_x = min_x;
            println!();
        }
        for _ in last_x + 1..*x {
            print!(" ")
        }
        print!("#");
        last_x = *x;
    }
    println!();
    println!();
}
