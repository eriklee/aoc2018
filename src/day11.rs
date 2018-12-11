#[aoc_generator(day11)]
fn build_grid(inp: &str) -> Vec<Vec<i8>> {
    let serial_number: i32 = inp.trim().parse().unwrap();
    let mut rows: Vec<Vec<i8>> = Vec::new();

    for y in 1..=300 {
        let mut row = Vec::new();
        for x in 1..=300 {
            let rack_id = x + 10;
            let mut power_level = rack_id * y;
            power_level += serial_number;
            power_level *= rack_id;
            power_level = (power_level % 1000) / 100;
            power_level -= 5;
            row.push(power_level as i8);
        }
        rows.push(row);
    }
    rows
}

#[aoc(day11, part1)]
pub fn part1(rows: &[Vec<i8>]) -> String {
    let mut best = std::i32::MIN;
    let mut best_coords = (0, 0);

    let size = 3;
    for y in 0..(300 - size) {
        for x in 0..(300 - size) {
            let score = rows[y..y + size]
                .iter()
                .map(|r| r[x..x + size].iter().map(|n| i32::from(*n)).sum::<i32>())
                .sum();
            if score > best {
                best = score;
                best_coords = (x + 1, y + 1);
            }
        }
    }
    println!("Best Score: {} at {:?}", best, best_coords);
    format!("{:?}", best_coords)
}

#[aoc(day11, part2, brute_force)]
pub fn part2(rows: &[Vec<i8>]) -> String {
    let mut best = std::i32::MIN;
    let mut best_size = 0;
    let mut best_coords = (0, 0);

    for y in 0..300 {
        for x in 0..300 {
            let max_size = 300 - std::cmp::max(x, y);
            for size in 2..=max_size {
                let score = rows[y..y + size]
                    .iter()
                    .map(|r| r[x..x + size].iter().map(|n| i32::from(*n)).sum::<i32>())
                    .sum();
                if score > best {
                    best = score;
                    best_coords = (x + 1, y + 1);
                    best_size = size;
                }
            }
        }
    }
    println!(
        "Best Score: {} at {:?} sized {}",
        best, best_coords, best_size
    );
    format!("{},{},{}", best_coords.0, best_coords.1, best_size)
}

#[aoc(day11, part2, use_the_L)]
pub fn part2_more_clever(rows: &[Vec<i8>]) -> String {
    let mut best = std::i32::MIN;
    let mut best_size = 0;
    let mut best_coords = (0, 0);

    for y in 0..300 {
        for x in 0..300 {
            let max_size = 300 - std::cmp::max(x, y);
            let mut score = i32::from(rows[y][x]);
            for size in 2..=max_size {
                let row: i32 = rows[y + size - 1][x..x + size]
                    .iter()
                    .map(|n| i32::from(*n))
                    .sum();
                // The col is slightly shorter since we don't
                // want to double count the bottom right corner
                let col: i32 = rows[y..y + size - 1]
                    .iter()
                    .map(|r| i32::from(r[x + size - 1]))
                    .sum();
                score += row + col;
                if score > best {
                    best = score;
                    best_coords = (x + 1, y + 1);
                    best_size = size;
                }
            }
        }
    }
    println!(
        "Best Score: {} at {:?} sized {}",
        best, best_coords, best_size
    );
    format!("{},{},{}", best_coords.0, best_coords.1, best_size)
}

#[aoc(day11, part2, use_the_transposed_L)]
pub fn part2_more_clever_transpose(rows: &[Vec<i8>]) -> String {
    let cols = transpose(&rows);
    let mut best = std::i32::MIN;
    let mut best_size = 0;
    let mut best_coords = (0, 0);

    for y in 0..300 {
        for x in 0..300 {
            let max_size = 300 - std::cmp::max(x, y);
            let mut score = i32::from(rows[y][x]);
            assert!(score == i32::from(cols[x][y]));
            for size in 2..=max_size {
                let row: i32 = rows[y + size - 1][x..x + size]
                    .iter()
                    .map(|n| i32::from(*n))
                    .sum();
                // The col is slightly shorter since we don't
                // want to double count the bottom right corner
                let col: i32 = cols[x + size - 1][y..y + size - 1]
                    .iter()
                    .map(|n| i32::from(*n))
                    .sum();
                score += row + col;
                if score > best {
                    best = score;
                    best_coords = (x + 1, y + 1);
                    best_size = size;
                }
            }
        }
    }
    println!(
        "Best Score: {} at {:?} sized {}",
        best, best_coords, best_size
    );
    format!("{},{},{}", best_coords.0, best_coords.1, best_size)
}

fn transpose(rows: &[Vec<i8>]) -> Vec<Vec<i8>> {
    let mut cols = Vec::new();
    for x in 0..rows.len() {
        let mut col = Vec::new();
        for y in 0..rows[x].len() {
            col.push(rows[y][x]);
        }
        cols.push(col);
    }
    cols
}

#[aoc(day11, part2, sat)]
fn part2_sat(rows: &[Vec<i8>]) -> String {
    let sat = build_sat(&rows);
    let mut best = std::i64::MIN;
    let mut best_size = 0;
    let mut best_coords = (0, 0);

    for y in 1..=300 {
        for x in 1..=300 {
            let max_size = 301 - std::cmp::max(x, y);
            for size in 1..=max_size {
                let score = get_sat_score(x, y, size, &sat);

                if score > best {
                    best = score;
                    best_coords = (x, y);
                    best_size = size;
                }
            }
        }
    }
    println!(
        "Best Score: {} at {:?} sized {}",
        best, best_coords, best_size
    );
    format!("{},{},{}", best_coords.0, best_coords.1, best_size)
}

fn get_sat_score(x: usize, y: usize, size: usize, sat: &[Vec<i64>]) -> i64 {
    let tl = sat[y - 1][x - 1];
    let tr = sat[y - 1][x + size - 1];
    let bl = sat[y + size - 1][x - 1];
    let br = sat[y + size - 1][x + size - 1];

    br + tl - tr - bl
}

fn build_sat(rows: &[Vec<i8>]) -> Vec<Vec<i64>> {
    let mut tab = Vec::new();

    // Want to put in an empty row/col so the get_sat_score fn is simpler
    let mut row = Vec::new();
    for _ in 0..rows[0].len() + 1 {
        row.push(0);
    }
    tab.push(row);

    for y in 0..rows.len() {
        let mut row = vec![0];
        let mut row_sum = 0;
        for x in 0..rows[y].len() {
            row_sum += i64::from(rows[y][x]);
            // The prev rows indexing is a bit funky due to the shift
            let prev_row_sum = tab[y][x + 1];
            row.push(row_sum + prev_row_sum);
        }
        tab.push(row);
    }
    tab
}
