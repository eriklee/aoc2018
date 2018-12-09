#[derive(Debug, Clone, Copy)]
pub struct Input {
    player_count: usize,
    last_marble: usize,
}
#[aoc_generator(day9)]
fn parse_inp(inp: &str) -> Box<Input> {
    // "431 players; last marble is worth 70950 points"
    use regex::*;
    let re = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
    let caps = re.captures(inp).unwrap();
    Box::new(Input {
        player_count: caps[1].parse().unwrap(),
        last_marble: caps[2].parse().unwrap(),
    })
}

#[aoc(day9, part1, slow)]
pub fn part1(inp: &Input) -> usize {
    let player_count = inp.player_count;
    let last_marble = inp.last_marble;

    solve(player_count, last_marble)
}
#[aoc(day9, part1, faster)]
pub fn part1_f(inp: &Input) -> usize {
    let player_count = inp.player_count;
    let last_marble = inp.last_marble;

    solve_better(player_count, last_marble)
}

#[aoc(day9, part2)]
fn part2(inp: &Input) -> usize {
    let player_count = inp.player_count;
    let last_marble = inp.last_marble * 100;

    solve_better(player_count, last_marble)
}

fn solve(player_count: usize, last_marble: usize) -> usize {
    let mut players = Vec::with_capacity(player_count);
    for _ in 0..player_count {
        (players.push(0));
    }

    let mut marbles: Vec<u32> = Vec::with_capacity(last_marble);
    marbles.push(0);
    marbles.push(2);
    marbles.push(1);

    let mut index = 1;
    for marble in 3..=last_marble {
        let player = marble % player_count;
        if marble % 23 == 0 {
            index = (index + marbles.len() - 7) % marbles.len();
            let removed = marbles.remove(index);
            players[player] += marble + removed as usize;
        } else {
            index = (((index + 1) % marbles.len()) + 1) % (marbles.len() + 1);
            marbles.insert(index, marble as u32);
        }
        //println!("idx: {} : {:?}", marbles[index], marbles);
    }
    *players.iter().max().unwrap()
}

fn solve_better(player_count: usize, last_marble: usize) -> usize {
    let mut players = Vec::with_capacity(player_count);
    for _ in 0..player_count {
        (players.push(0));
    }

    let mut marbles = CircBuffer::new();

    for marble in 1..=last_marble {
        let player = marble % player_count;
        if marble % 23 == 0 {
            for _ in 0..7 {
                marbles.counter_clockwise();
            }
            let removed = marbles.pop();
            players[player] += marble + removed as usize;
        } else {
            marbles.clockwise();
            marbles.clockwise();
            marbles.push(marble as u32);
        }
        //marbles.show();
    }

    *players.iter().max().unwrap()
}

// A Zipper inspired circular buffer with a focus
// the cw vec is 'backwards' to enable efficiently
// pushing and popping of the elements
//
// Doing this avoids an enormous amount of shuffling vec
// elements back and forth as items are inserted and removed
struct CircBuffer {
    ccw: Vec<u32>,
    // This should be an Option<T>, but that isn't really necessary for this...
    focus: u32,
    cw: Vec<u32>,
}

impl CircBuffer {
    fn new() -> Self {
        CircBuffer {
            ccw: Vec::new(),
            focus: 0,
            cw: Vec::new(),
        }
    }

    // Remove the focus and slide the next item cw around in its place
    // It is an error to remove the last element
    fn pop(self: &mut Self) -> u32 {
        let res = self.focus;
        match self.cw.pop() {
            Some(new_focus) => self.focus = new_focus,
            None => {
                std::mem::swap(&mut self.cw, &mut self.ccw);
                self.cw.reverse();
                self.focus = self.cw.pop().expect("Tried to pop from a 1 element buffer");
            }
        };
        res
    }

    // Rotates the focus one item clockwise
    fn clockwise(self: &mut Self) {
        self.ccw.push(self.focus);
        match self.cw.pop() {
            Some(new_focus) => self.focus = new_focus,
            None => {
                // It's probably much better to reuse our potentially big vecs
                std::mem::swap(&mut self.cw, &mut self.ccw);
                self.cw.reverse();
                self.focus = self.cw.pop().expect("Tried to pop from a 1 element buffer");
            }
        };
    }

    // Rotates the focus one item counter_clockwise
    fn counter_clockwise(self: &mut Self) {
        self.cw.push(self.focus);
        match self.ccw.pop() {
            Some(new_focus) => self.focus = new_focus,
            None => {
                std::mem::swap(&mut self.cw, &mut self.ccw);
                self.ccw.reverse();
                self.focus = self
                    .ccw
                    .pop()
                    .expect("Tried to pop from a 1 element buffer");
            }
        };
    }

    // Insert the new element in the focus, pushing the current focus cw
    fn push(self: &mut Self, elem: u32) {
        self.cw.push(self.focus);
        self.focus = elem;
    }

    #[allow(dead_code)]
    fn show(self: &Self) -> () {
        println!(
            "{:?} ={:?}= {:?}",
            self.ccw,
            self.focus,
            self.cw.iter().rev()
        )
    }
}
