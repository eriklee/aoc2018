use counter::Counter;
use std::collections::HashSet;

type Id = usize;
type Coord = (i32, i32);

#[aoc_generator(day6)]
pub fn part1_g(inp: &str) -> Vec<(Id, Coord)> {
    let mut res = Vec::new();
    for (id, l) in inp.lines().enumerate() {
        let mut coord_strs = l.split(", ").map(str::parse);
        let x = coord_strs.next().unwrap().unwrap();
        let y = coord_strs.next().unwrap().unwrap();
        res.push((id, (x, y)));
    }
    res
}

#[aoc(day6, part1)]
pub fn part1(points: &[(Id, Coord)]) -> usize {
    let (x_max, y_max) = get_bounds(points);
    let mut grid = build_grid(x_max, y_max);

    for (id, p) in points {
        for x in 0..x_max {
            for y in 0..y_max {
                let dist = manhattan_distance((x as i32, y as i32), *p);
                let (owner, cur_dist) = grid.get_mut((y * x_max) + x).unwrap();
                match dist {
                    _ if dist == *cur_dist => *owner = None,
                    _ if dist < *cur_dist => {
                        *owner = Some(*id);
                        *cur_dist = dist
                    }
                    _ => {}
                }
            }
        }
    }
    let edge_lords = get_edges(&grid, (x_max as i32, y_max as i32));
    let areas = get_areas(&grid);
    areas
        .most_common_ordered()
        .iter()
        .filter_map(|(id, count)| {
            if edge_lords.contains(id) {
                None
            } else {
                Some(*count)
            }
        })
        .next()
        .unwrap_or(0)
}

// Had a vague intuition this might be more natural.
// It also happens to be quite a lot faster and I'm not exactly sure why...
#[aoc(day6, part1, inside_out)]
pub fn part1_io(points: &[(Id, Coord)]) -> usize {
    let (x_max, y_max) = get_bounds(points);
    let mut grid = Vec::with_capacity(x_max * y_max as usize);

    for y in 0..y_max {
        for x in 0..x_max {
            let mut min = std::usize::MAX;
            let mut owner = None;
            for (id, p) in points {
                let dist = manhattan_distance((x as i32, y as i32), *p);
                if dist == min {
                    owner = None
                } else if dist < min {
                    owner = Some(*id);
                    min = dist
                }
            }
            grid.push((owner, min));
        }
    }
    let edge_lords = get_edges(&grid, (x_max as i32, y_max as i32));
    let areas = get_areas(&grid);
    areas
        .most_common_ordered()
        .iter()
        .filter_map(|(id, count)| {
            if edge_lords.contains(id) {
                None
            } else {
                Some(*count)
            }
        })
        .next()
        .unwrap_or(0)
}

#[aoc(day6, part2)]
pub fn part2(points: &[(Id, Coord)]) -> usize {
    let (x_max, y_max) = get_bounds_fold(points);
    let mut res = 0;

    for x in 0..x_max {
        for y in 0..y_max {
            res += (points
                .iter()
                .map(|(_, p)| manhattan_distance((x as i32, y as i32), *p))
                .sum::<usize>()
                < 10_000) as usize;
        }
    }
    res
}

fn get_areas(grid: &[(Option<Id>, usize)]) -> Counter<Id> {
    grid.iter().map(|x| x.0).filter_map(|x| x).collect()
}

// Anything on an edge will have an infinite area
fn get_edges(grid: &[(Option<Id>, usize)], (x_max, y_max): Coord) -> HashSet<Id> {
    let mut res = HashSet::new();
    let edges = (0..x_max) // Top
        .chain((x_max * (y_max - 1))..(x_max * y_max)) // Bottom
        .chain((0..(x_max * (y_max - 1))).step_by(x_max as usize)) // Left Edge
        .chain(((x_max - 1)..(x_max * y_max)).step_by(x_max as usize)); // Right Edge
    for p in edges {
        if let Some(id) = grid[p as usize].0 {
            res.insert(id);
        }
    }
    res
}

fn manhattan_distance((x1, y1): Coord, (x2, y2): Coord) -> usize {
    ((x1 - x2).abs() + (y1 - y2).abs()) as usize
}

fn build_grid(x_max: usize, y_max: usize) -> Vec<(Option<Id>, usize)> {
    let mut grid = Vec::with_capacity(x_max * y_max as usize);

    for _ in 0..(x_max * y_max) {
        grid.push((None, std::usize::MAX))
    }
    grid
}

fn get_bounds(inp: &[(Id, Coord)]) -> (usize, usize) {
    let mut x_max = 0;
    let mut y_max = 0;
    for &(_, (x, y)) in inp {
        if x > x_max {
            x_max = x
        }
        if y > y_max {
            y_max = y
        }
    }
    (x_max as usize, y_max as usize)
}

fn get_bounds_fold(inp: &[(Id, Coord)]) -> (usize, usize) {
    let (x_max, y_max) = inp.iter().fold((0, 0), |(x_max, y_max), (_, (x, y))| {
        (std::cmp::max(*x, x_max), std::cmp::max(*y, y_max))
    });
    (x_max as usize, y_max as usize)
}
