use counter::Counter;

#[aoc(day2, part1)]
pub fn day2_1(inp: &str) -> i32 {
    let (count2, count3) = inp
        .lines()
        .map(|box_id| {
            box_id
                .chars()
                .collect::<Counter<_>>()
                .values()
                .fold((false, false), |(has2, has3), &count| {
                    (has2 || (count == 2), has3 || (count == 3))
                })
        })
        .map(|(has2, has3)| (has2 as i32, has3 as i32))
        .fold((0, 0), |(acc2, acc3), (h2, h3)| (acc2 + h2, acc3 + h3));
    count2 * count3
}

#[aoc(day2, part2)]
pub fn day2(inp: &str) -> String {
    let (id1, id2) = find_id_pair(inp);
    id1.chars()
        .zip(id2.chars())
        .filter(|(a, b)| a == b)
        .map(|(a, _b)| a)
        .collect()
}

fn distance(in1: &str, in2: &str) -> usize {
    in1.chars().zip(in2.chars()).filter(|(a, b)| a != b).count()
}

fn find_id_pair(inp: &str) -> (String, String) {
    let box_ids = inp.lines();

    for id in box_ids {
        for candidate in inp.lines() {
            if distance(id, candidate) == 1 {
                return (id.to_owned(), candidate.to_owned());
            }
        }
    }
    unreachable!()
}
