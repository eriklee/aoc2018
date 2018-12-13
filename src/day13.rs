use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, Clone, Copy)]
enum IntersectionChoice {
    ICLeft,
    ICStraight,
    ICRight,
}
#[derive(Debug, Clone, Copy)]
enum PointType {
    Vert,
    Horiz,
    ForSlash,
    BackSlash,
    Intersection,
}

#[derive(Debug)]
struct Cart {
    location: Point,
    direction: Direction,
    next_intersection: IntersectionChoice,
}
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Point {
    y: usize,
    x: usize,
}
#[derive(Debug)]
struct GridPoint {
    gtype: PointType,
    next_points: [Option<Point>; 4],
}
#[derive(Debug)]
struct Grid {
    points: HashMap<Point, GridPoint>,
}
#[derive(Debug)]
struct Carts {
    carts: HashMap<Point, Cart>,
}

#[aoc_generator(day13)]
fn parse_input(inp: &[u8]) -> Box<(Grid, Carts)> {
    use self::Direction::*;
    use self::IntersectionChoice::*;
    use self::PointType::*;
    use std::io::BufRead;

    let mut points = HashMap::new();
    let mut carts = HashMap::new();

    for (y, line) in inp.lines().enumerate() {
        let mut prev_byte = ' ';
        for (x, char) in line.unwrap().chars().enumerate() {
            let loc = Point { x, y };
            let (gpm, cartm) = match char {
                '^' => {
                    let cart = Cart {
                        location: loc,
                        direction: Up,
                        next_intersection: ICLeft,
                    };
                    let grid_point = build_grid_point(Vert, &loc, prev_byte);
                    (Some(grid_point), Some(cart))
                }
                'v' => {
                    let cart = Cart {
                        location: loc,
                        direction: Down,
                        next_intersection: ICLeft,
                    };
                    let grid_point = build_grid_point(Vert, &loc, prev_byte);
                    (Some(grid_point), Some(cart))
                }
                '<' => {
                    let cart = Cart {
                        location: loc,
                        direction: Left,
                        next_intersection: ICLeft,
                    };
                    let grid_point = build_grid_point(Horiz, &loc, prev_byte);
                    (Some(grid_point), Some(cart))
                }
                '>' => {
                    let cart = Cart {
                        location: loc,
                        direction: Right,
                        next_intersection: ICLeft,
                    };
                    let grid_point = build_grid_point(Horiz, &loc, prev_byte);
                    (Some(grid_point), Some(cart))
                }
                '|' => {
                    let grid_point = build_grid_point(Vert, &loc, prev_byte);
                    (Some(grid_point), None)
                }
                '-' => {
                    let grid_point = build_grid_point(Horiz, &loc, prev_byte);
                    (Some(grid_point), None)
                }
                '/' => {
                    let grid_point = build_grid_point(ForSlash, &loc, prev_byte);
                    (Some(grid_point), None)
                }
                '\\' => {
                    let grid_point = build_grid_point(BackSlash, &loc, prev_byte);
                    (Some(grid_point), None)
                }
                '+' => {
                    let grid_point = build_grid_point(Intersection, &loc, prev_byte);
                    (Some(grid_point), None)
                }
                ' ' => (None, None),
                _ => unreachable!(),
            };
            if let Some(gp) = gpm {
                points.insert(loc, gp);
            }
            if let Some(cart) = cartm {
                carts.insert(loc, cart);
            }
            prev_byte = char;
        }
    }

    Box::new((Grid { points }, Carts { carts }))
}

fn next_point(dir: Direction, loc: &Point) -> Point {
    use self::Direction::*;
    match dir {
        Up => Point {
            x: loc.x,
            y: loc.y - 1,
        },
        Down => Point {
            x: loc.x,
            y: loc.y + 1,
        },
        Left => Point {
            x: loc.x - 1,
            y: loc.y,
        },
        Right => Point {
            x: loc.x + 1,
            y: loc.y,
        },
    }
}

fn build_grid_point(pt: PointType, loc: &Point, prev_byte: char) -> GridPoint {
    use self::Direction::*;
    use self::PointType::*;
    let next_points = match pt {
        Vert => [
            Some(next_point(Up, &loc)),
            Some(next_point(Down, &loc)),
            None,
            None,
        ],
        Horiz => [
            None,
            None,
            Some(next_point(Left, &loc)),
            Some(next_point(Right, &loc)),
        ],
        ForSlash => pick_slash_type(pt, loc, prev_byte),
        BackSlash => pick_slash_type(pt, loc, prev_byte),
        Intersection => [
            Some(next_point(Up, &loc)),
            Some(next_point(Down, &loc)),
            Some(next_point(Left, &loc)),
            Some(next_point(Right, &loc)),
        ],
    };
    GridPoint {
        gtype: pt,
        next_points,
    }
}

fn pick_slash_type(pt: PointType, loc: &Point, prev_byte: char) -> [Option<Point>; 4] {
    use self::Direction::*;
    use self::PointType::*;
    match pt {
        ForSlash => {
            if prev_byte == '-' {
                // Up, Left
                [
                    Some(next_point(Up, &loc)),
                    None,
                    Some(next_point(Left, &loc)),
                    None,
                ]
            } else {
                // Down, Right
                [
                    None,
                    Some(next_point(Down, &loc)),
                    None,
                    Some(next_point(Right, &loc)),
                ]
            }
        }
        BackSlash => {
            if prev_byte == '-' {
                // Down, Left
                [
                    None,
                    Some(next_point(Down, &loc)),
                    Some(next_point(Left, &loc)),
                    None,
                ]
            } else {
                // Up, Right
                [
                    Some(next_point(Up, &loc)),
                    None,
                    None,
                    Some(next_point(Right, &loc)),
                ]
            }
        }
        _ => unreachable!(),
    }
}

#[aoc(day13, part1)]
fn part1((grid, carts): &(Grid, Carts)) -> String {
    let mut this_tick = tick(&grid, &carts);

    loop {
        match this_tick {
            Ok(carts) => this_tick = tick(&grid, &carts),
            Err(crash_loc) => {
                println!("Crash! {:?}", crash_loc);
                return format!("{},{}", crash_loc.x, crash_loc.y);
            }
        }
    }
}

#[aoc(day13, part2)]
fn part2((grid, carts): &(Grid, Carts)) -> String {
    let mut this_tick2 = tick2(&grid, &carts);
    loop {
        this_tick2 = tick2(&grid, &this_tick2);
        if this_tick2.carts.len() == 1 {
            let point = this_tick2.carts.keys().next().unwrap();
            return format!("{},{}", point.x, point.y);
        }
    }
}

#[allow(unused)]
fn print_grid(grid: &Grid) {
    let mut gps: Vec<Point> = grid.points.keys().map(|p| *p).collect();
    gps.sort();
    for p in gps {
        println!("{:?} - {:?}", p, grid.points[&p])
    }
}

fn step_cart(grid: &Grid, cart: &Cart) -> Cart {
    let next_point = next_point(cart.direction, &cart.location);
    let next_gp = &grid.points[&next_point];
    let (direction, next_intersection) =
        next_direction(next_gp.gtype, cart.direction, cart.next_intersection);

    Cart {
        location: next_point,
        direction,
        next_intersection,
    }
}

fn tick(grid: &Grid, carts: &Carts) -> Result<Carts, Point> {
    let mut ticked_carts = HashMap::with_capacity(carts.carts.len());
    let mut cart_keys = carts.carts.keys().map(|p| *p).collect::<Vec<Point>>();
    cart_keys.sort();

    for cart_id in cart_keys.iter() {
        let cart = &carts.carts[&cart_id];
        let next_cart = step_cart(&grid, cart);
        let next_loc = next_cart.location;
        // There's a crash if the cart is now where a cart was last turn
        // that hasn't been moved yet
        if next_loc > *cart_id && cart_keys.contains(&next_loc) {
            return Err(next_loc);
        }
        // There's a crash if we try to put two carts in the same place
        if ticked_carts.insert(next_loc, next_cart).is_some() {
            return Err(next_loc);
        }
    }
    return Ok(Carts {
        carts: ticked_carts,
    });
}

fn tick2(grid: &Grid, carts: &Carts) -> Carts {
    let mut ticked_carts = HashMap::with_capacity(carts.carts.len());
    let mut cart_keys = carts.carts.keys().map(|p| *p).collect::<Vec<Point>>();
    cart_keys.sort();

    let mut skip_carts = HashSet::new();
    for cart_id in cart_keys.iter() {
        if skip_carts.contains(cart_id) {
            continue;
        }
        let cart = &carts.carts[&cart_id];
        let next_cart = step_cart(&grid, cart);
        let next_loc = next_cart.location;
        // There's a crash if the cart is now where a cart was last turn
        // that hasn't been moved yet
        if next_loc > *cart_id && cart_keys.contains(&next_loc) && !skip_carts.contains(&next_loc) {
            skip_carts.insert(next_loc);
            continue;
        }
        // There's a crash if we try to put two carts in the same place
        if ticked_carts.insert(next_loc, next_cart).is_some() {
            ticked_carts.remove(&next_loc);
            continue;
        }
    }
    assert!(ticked_carts.len() % 2 == 1);
    return Carts {
        carts: ticked_carts,
    };
}

fn next_direction(
    pt: PointType,
    dir: Direction,
    ic: IntersectionChoice,
) -> (Direction, IntersectionChoice) {
    use self::Direction::*;
    use self::PointType::*;
    match (pt, dir) {
        (Vert, Up) => (Up, ic),
        (Vert, Down) => (Down, ic),
        (Horiz, Left) => (Left, ic),
        (Horiz, Right) => (Right, ic),

        (ForSlash, Up) => (Right, ic),
        (ForSlash, Down) => (Left, ic),
        (ForSlash, Left) => (Down, ic),
        (ForSlash, Right) => (Up, ic),

        (BackSlash, Up) => (Left, ic),
        (BackSlash, Down) => (Right, ic),
        (BackSlash, Left) => (Up, ic),
        (BackSlash, Right) => (Down, ic),

        (Intersection, dir) => pick_intersection(dir, ic),
        (_, _) => unreachable!(),
    }
}

fn pick_intersection(dir: Direction, ic: IntersectionChoice) -> (Direction, IntersectionChoice) {
    use self::IntersectionChoice::*;
    let new_dir = match ic {
        ICLeft => turn_left(dir),
        ICStraight => dir,
        ICRight => turn_right(dir),
    };
    (new_dir, cycle_ic(ic))
}

fn turn_left(dir: Direction) -> Direction {
    use self::Direction::*;
    match dir {
        Up => Left,
        Down => Right,
        Left => Down,
        Right => Up,
    }
}

fn turn_right(dir: Direction) -> Direction {
    use self::Direction::*;
    match dir {
        Up => Right,
        Down => Left,
        Left => Up,
        Right => Down,
    }
}

fn cycle_ic(ic: IntersectionChoice) -> IntersectionChoice {
    use self::IntersectionChoice::*;
    match ic {
        ICLeft => ICStraight,
        ICStraight => ICRight,
        ICRight => ICLeft,
    }
}
