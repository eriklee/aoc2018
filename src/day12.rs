use regex::*;

#[derive(Debug, PartialEq, Copy, Clone)]
struct Rule {
    input: [bool; 5],
    output: bool,
}

#[derive(Debug, PartialEq, Clone)]
struct State {
    vec: Vec<bool>,
    first_index: i64,
}

#[derive(Debug, PartialEq, Clone)]
struct InitialState {
    state: State,
    rules: Vec<Rule>,
}

#[aoc_generator(day12)]
fn parse_input(inp: &str) -> Box<InitialState> {
    //initial state: ##.......#.######.##..#...#.#.#..#...#..####..#.##...#....#...##..#..#.##.##.###.##.#.......###....#
    let initial_state_re = Regex::new(r"^initial state: ([#\.]+)$").unwrap();

    let mut lines = inp.lines();
    let state_match = initial_state_re.captures(lines.next().unwrap()).unwrap();
    let state = State {
        vec: inp_to_vec(&state_match[1]),
        first_index: -5,
    };
    let rules = parse_rules(&lines.skip(1).collect::<Vec<&str>>());

    Box::new(InitialState { state, rules })
}

fn inp_to_vec(inp: &str) -> Vec<bool> {
    let mut res: Vec<bool> = inp.chars().map(|c| c == '#').collect();
    res.insert(0, false);
    res.insert(0, false);
    res.insert(0, false);
    res.insert(0, false);
    res.insert(0, false);
    res.push(false);
    res.push(false);
    res.push(false);
    res.push(false);
    res.push(false);
    res
}

fn parse_rules(lines: &[&str]) -> Vec<Rule> {
    //.#### => .
    let rule_re = Regex::new(r"([#\.]+) => ([#\.])").unwrap();
    lines
        .iter()
        .map(move |rl| {
            let caps = rule_re.captures(rl).unwrap();
            let input = inp_to_arr(&caps[1]);
            let output = caps[2] == *"#";
            Rule { input, output }
        })
        .collect()
}

fn inp_to_arr(inp: &str) -> [bool; 5] {
    assert!(inp.len() == 5);
    let mut result = [false; 5];
    for (i, c) in inp.chars().take(5).enumerate() {
        result[i] = c == '#';
    }
    result
}

#[aoc(day12, part1)]
fn part1(inp: &InitialState) -> i64 {
    let rules = &inp.rules;
    let mut state = inp.state.clone();

    for _ in 0..20 {
        state = iterate_state(&rules, &state);
    }

    show_state(&state);
    score_state(&state)
}

#[aoc(day12, part2)]
fn part2(inp: &InitialState) -> i64 {
    let rules = &inp.rules;
    let mut state = inp.state.clone();
    let mut score = 0;
    let mut diff = 0;

    let max: i64 = 500_000_000_000;
    for g in 1..=max {
        state = iterate_state(&rules, &state);
        let new_score = score_state(&state);
        if diff == new_score - score {
            println!("G {} score: {} diff: {}", g, new_score, diff);
            return (max - g) * diff + new_score;
        }
        diff = new_score - score;
        score = new_score;
    }

    show_state(&state);
    score_state(&state)
}

fn iterate_state(rules: &[Rule], prev: &State) -> State {
    let mut next = vec![false, false, false];
    for i in 2..prev.vec.len() - 2 {
        let w = &prev.vec[i - 2..=i + 2];
        for r in rules {
            if *w == r.input {
                next.push(r.output);
                break;
            }
        }
    }

    next.push(false);
    next.push(false);
    next.push(false);
    next.push(false);
    next.push(false);
    let st = State {
        vec: next,
        first_index: prev.first_index - 1,
    };
    trim_state(st)
}

fn trim_state(mut st: State) -> State {
    for w in st.vec.clone().windows(5) {
        if w.iter().filter(|x| **x).count() > 0 {
            break;
        } else {
            st.vec.remove(0);
            st.first_index += 1
        }
    }

    for w in st.vec.clone().windows(5).rev() {
        if w.iter().filter(|x| **x).count() > 0 {
            break;
        } else {
            st.vec.pop();
        }
    }

    st
}

fn show_state(state: &State) {
    println!("from: {}", state.first_index);
    println!(
        "{:?}",
        state
            .vec
            .iter()
            .map(|b| if *b { '#' } else { '.' })
            .collect::<String>()
    );
}
fn score_state(state: &State) -> i64 {
    let first_index = state.first_index;
    state
        .vec
        .iter()
        .enumerate()
        .flat_map(|(i, e)| {
            if *e {
                Some(i as i64 + first_index)
            } else {
                None
            }
        })
        .sum()
}
