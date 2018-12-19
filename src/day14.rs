type Score = u8;
type Board = Vec<Score>;
type Elves = Vec<usize>;

#[aoc(day14, part1)]
pub fn part1(inp: &str) -> String {
    let mut board = vec![3, 7];
    let mut elves = vec![0, 1];

    let max: usize = inp.trim().parse().unwrap();

    while board.len() < max + 10 {
        let cur_scores = elf_scores(&board, &elves);
        let mut new_recipes = find_new_recipes(&cur_scores);
        add_new_recipes(&mut board, &mut new_recipes);
        elves = cycle_elves(board.len(), &elves, &cur_scores);
    }
    println!("{:?}", &board[max..(max + 10)]);
    board[max..(max + 10)]
        .iter()
        .map(|n| std::char::from_digit(u32::from(*n), 10).unwrap())
        .collect()
}

#[aoc(day14, part2)]
pub fn part2(inp: &str) -> usize {
    let mut board = vec![3, 7];
    let mut elves = vec![0, 1];
    let mut up_to = 0;

    let needle: Vec<u8> = inp
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    loop {
        let cur_scores = elf_scores(&board, &elves);
        let mut new_recipes = find_new_recipes(&cur_scores);
        add_new_recipes(&mut board, &mut new_recipes);
        elves = cycle_elves(board.len(), &elves, &cur_scores);

        for (i, window) in board[up_to..].windows(needle.len()).enumerate() {
            if *window == needle[..] {
                println!("Found!: {:?}", i + up_to);
                return i + up_to;
            }
        }
        up_to = std::cmp::max(board.len() as isize - 5, 0) as usize;
    }
}

fn elf_scores(board: &Board, elves: &Elves) -> Vec<Score> {
    let mut res = Vec::new();
    for &elf in elves {
        res.push(board[elf]);
    }
    res
}

fn find_new_recipes(inp: &[Score]) -> Vec<Score> {
    let mut sum: usize = inp.iter().map(|n| *n as usize).sum();
    if sum == 0 {
        return vec![0];
    }
    let mut res = Vec::new();
    while sum > 0 {
        let rem = (sum % 10) as Score;
        let quot = sum / 10;
        res.push(rem);
        sum = quot;
    }
    res.reverse();
    res
}

fn add_new_recipes(board: &mut Board, new_recipes: &mut Vec<Score>) {
    board.append(new_recipes);
}

fn cycle_elves(board_len: usize, elves: &Elves, scores: &[Score]) -> Elves {
    elves
        .iter()
        .zip(scores.iter())
        .map(|(elf, score)| (elf + 1 + *score as usize) % board_len)
        .collect()
}
