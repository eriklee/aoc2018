use std::collections::hash_map::{DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};
type State = Vec<Vec<CellTy>>;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum CellTy {
    Open,
    Trees,
    Lumberyard,
}

fn parse_cell(c: char) -> CellTy {
    use self::CellTy::*;
    match c {
        '.' => Open,
        '|' => Trees,
        '#' => Lumberyard,
        _ => unreachable!(),
    }
}
#[aoc_generator(day18)]
fn parse(inp: &str) -> State {
    //let inp = ".#.#...|#.
    //.....#|##|
    //.|..|...#.
    //..|#.....#
    //#.#|||#|#|
    //...#.||...
    //.|....|...
    //||...#|.#|
    //|.||||..|.
    //...#.|..|.";
    inp.lines()
        .map(|l| l.chars().map(parse_cell).collect())
        .collect()
}

fn clamp(n: usize) -> usize {
    use std::cmp::max;
    let x = n as isize;
    let y = max(0, x - 1);
    y as usize
}

fn neighborhood(x: usize, y: usize, state: &State) -> Vec<CellTy> {
    let mut res = Vec::new();
    for yn in clamp(y)..=(y + 1) {
        for xn in clamp(x)..=(x + 1) {
            //print!("{},{} ", xn, yn);
            if (x, y) != (xn, yn) {
                res.push(state.get(yn).and_then(|yl| yl.get(xn)));
            }
        }
    }
    //println!();
    return res.into_iter().filter_map(|x| x).map(|x| *x).collect();
}
fn step_state(prev: &State) -> State {
    let mut outer = Vec::new();
    for (y, yl) in prev.iter().enumerate() {
        let mut inner = Vec::new();
        for (x, cell) in yl.iter().enumerate() {
            let neighbors = neighborhood(x, y, &prev);
            let next_cell = step_cell(*cell, &neighbors);
            //println!("{:?} ({:?}) => {:?}", (x, y, cell), neighbors, next_cell);
            inner.push(next_cell);
        }
        outer.push(inner);
    }
    outer
}

fn step_cell(cell: CellTy, neighbors: &Vec<CellTy>) -> CellTy {
    use self::CellTy::*;
    match cell {
        Open => step_open(neighbors),
        Trees => step_trees(neighbors),
        Lumberyard => step_lumberyard(neighbors),
    }
}

fn step_open(neighbors: &Vec<CellTy>) -> CellTy {
    use self::CellTy::*;
    if neighbors.iter().filter(|c| **c == Trees).count() >= 3 {
        Trees
    } else {
        Open
    }
}

fn step_trees(neighbors: &Vec<CellTy>) -> CellTy {
    use self::CellTy::*;
    if neighbors.iter().filter(|c| **c == Lumberyard).count() >= 3 {
        Lumberyard
    } else {
        Trees
    }
}

fn step_lumberyard(neighbors: &Vec<CellTy>) -> CellTy {
    use self::CellTy::*;
    if neighbors.iter().filter(|c| **c == Trees).count() >= 1
        && neighbors.iter().filter(|c| **c == Lumberyard).count() >= 1
    {
        Lumberyard
    } else {
        Open
    }
}

#[aoc(day18, part1)]
fn part1(initial_state: &State) -> usize {
    let mut state: State = initial_state.clone();

    for _ in 0..10 {
        state = step_state(&state);
    }
    println!("Final:");
    show_state(&state);
    score_state(&state)
}

#[aoc(day18, part2)]
fn part2(initial_state: &State) -> usize {
    let mut state: State = initial_state.clone();
    let mut hm = HashMap::new();

    for i in 1..1_000_000_000 {
        state = step_state(&state);

        // Storing just the hash because that seems likely to be much smaller
        let mut s = DefaultHasher::new();
        state.hash(&mut s);
        let hash = s.finish();

        match hm.insert(hash, i) {
            None => continue,
            Some(last_i) => {
                let period = i - last_i;
                let rem_iters = (1_000_000_000 - i) % period;
                println!(
                    "Hash refound! {} -> {} (period {}), going to {}",
                    last_i, i, period, rem_iters
                );
                for _ in 0..rem_iters {
                    state = step_state(&state);
                }
                break;
            }
        }
    }
    println!("Final:");
    show_state(&state);
    score_state(&state)
}

fn score_state(state: &State) -> usize {
    use self::CellTy::*;
    let ly_count: usize = state
        .iter()
        .map(|l| l.iter().filter(|c| **c == Lumberyard).count())
        .sum();
    let tr_count: usize = state
        .iter()
        .map(|l| l.iter().filter(|c| **c == Trees).count())
        .sum();
    tr_count * ly_count
}

fn show_state(state: &State) {
    use self::CellTy::*;
    for l in state.iter() {
        for c in l.iter() {
            let show = match c {
                Open => ".",
                Trees => "|",
                Lumberyard => "#",
            };
            print!("{}", show);
        }
        println!();
    }
}
