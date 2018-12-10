use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Default)]
struct Node {
    dep_count: u32,
    dependents: Vec<char>,
}

#[aoc(day7, part1)]
pub fn part1(inp: &str) -> String {
    let dep_pairs = parse_pairs(inp);
    let graph = build_graph(dep_pairs);
    println!("{:?}", graph);
    let res = compute_path(graph);

    res.into_iter().collect()
}

#[aoc(day7, part2)]
pub fn part2(inp: &str) -> usize {
    let dep_pairs = parse_pairs(inp);
    let graph = build_graph(dep_pairs);
    compute_timing(graph)
}

// Vec of pairs where the first element is a dependency for the second
fn parse_pairs(inp: &str) -> Vec<(char, char)> {
    // Step Q must be finished before step I can begin.
    inp.lines()
        .map(|l| {
            (
                l.bytes().nth(5).unwrap() as char,
                l.bytes().nth(36).unwrap() as char,
            )
        })
        .collect()
}

fn build_graph(pairs: Vec<(char, char)>) -> [Node; 26] {
    let mut res: [Node; 26] = Default::default();

    for (src, dst) in pairs {
        let src_node = res.get_mut((src as u8 - b'A') as usize).unwrap();
        src_node.dependents.push(dst);

        let dst_node = res.get_mut((dst as u8 - b'A') as usize).unwrap();
        dst_node.dep_count += 1;
    }
    res
}

// The binary heap is a max heap and we need a min heap
// The correct way to do this is to make a new type and
// define a bunch of traits, but I'm in a bit of a hurry...
fn flip_index(letter: u8) -> u8 {
    25 - letter
}

fn compute_path(mut inp: [Node; 26]) -> Vec<char> {
    let mut res = Vec::new();
    let mut queue = BinaryHeap::new();

    for (i, e) in inp.iter().enumerate() {
        if (*e).dep_count == 0 {
            queue.push(flip_index(i as u8))
        }
    }

    while let Some(inv_id) = queue.pop() {
        let id = flip_index(inv_id);
        res.push((id + b'A') as char);
        let deps = {
            let src = inp.get(id as usize).unwrap();
            assert!(src.dep_count == 0);
            src.dependents.clone()
        };
        for dep in deps {
            let index = (dep as u8 - b'A') as usize;
            let mut n = inp.get_mut(index).unwrap();
            n.dep_count -= 1;
            if n.dep_count == 0 {
                queue.push(flip_index(index as u8))
            }
        }
    }

    res
}

// "Letters" are 0 indexed
fn job_length(letter: u8) -> usize {
    letter as usize + 60 + 1
}

#[derive(Debug, Ord, PartialEq, Eq, PartialOrd)]
struct Worker {
    next_free: usize,
    job: u8,
}

fn compute_timing(mut inp: [Node; 26]) -> usize {
    let mut time = 0;
    let mut free_workers = 5;
    let mut workers = Vec::new();
    let mut queue = BinaryHeap::new();

    for (i, e) in inp.iter().enumerate() {
        if (*e).dep_count == 0 {
            queue.push(flip_index(i as u8))
        }
    }

    // Assign initial jobs to workers
    while free_workers > 0 {
        if let Some(inv_id) = queue.pop() {
            let job = flip_index(inv_id);
            let next_free = job_length(job) + time;
            println!(
                "Time: {} worker starting {} until {}",
                time,
                show_job_id(job),
                next_free
            );
            workers.push(Worker { next_free, job });
            free_workers -= 1;
        } else {
            break;
        }
    }

    println!("Begin time");
    while free_workers < 5 {
        let worker = {
            workers.sort();
            workers.remove(0)
        };
        let id = worker.job;
        time = worker.next_free;
        free_workers += 1;
        println!(
            "Job {} finished at time: {}, available workers: {}",
            show_job_id(id),
            time,
            free_workers
        );

        // Should probably check to make sure the next worker isn't
        // also finished now...
        // update available jobs
        let deps = {
            let src = inp.get(id as usize).unwrap();
            assert!(src.dep_count == 0);
            src.dependents.clone()
        };
        for dep in deps {
            let index = letter_to_index(dep);
            let mut n = inp.get_mut(index).unwrap();
            n.dep_count -= 1;
            if n.dep_count == 0 {
                println!("{} is now available for work", show_job_id(index as u8));
                queue.push(flip_index(index as u8))
            }
        }
        // Assign any workers to spare jobs
        while free_workers > 0 {
            if let Some(inv_id) = queue.pop() {
                let job = flip_index(inv_id);
                let next_free = job_length(job) + time;
                println!(
                    "Time: {} worker starting {} until {}",
                    time,
                    show_job_id(job),
                    next_free
                );
                workers.push(Worker { next_free, job });
                free_workers -= 1;
            } else {
                break;
            }
        }
    }

    time
}

fn show_job_id(id: u8) -> char {
    (id + (b'A')) as char
}

fn letter_to_index(letter: char) -> usize {
    (letter as u8 - b'A') as usize
}
