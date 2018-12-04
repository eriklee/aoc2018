use std::collections::HashMap;
type GuardId = u16;
type Time = u8;

#[derive(Debug)]
pub struct Span {
    guard: GuardId,
    asleep: Time,
    awake: Time,
}

#[aoc_generator(day4)]
pub fn to_spans(inp: &str) -> Vec<Span> {
    let mut sorted: Vec<&str> = inp.lines().collect();
    // Sorting is fine here, because Guard < falls < wakes and times are sensible
    sorted.sort_unstable();

    let mut guard = 0;
    let mut asleep = 0;
    let mut res = Vec::new();

    // Filthy "parsing"
    // [1518-09-25 00:28] falls asleep
    // [1518-10-14 00:00] Guard #2927 begins shift
    // [1518-07-20 00:53] wakes up
    for line in sorted {
        match &line[19..20] {
            "G" => {
                guard = line.split_whitespace().nth(3).unwrap()[1..]
                    .parse()
                    .unwrap()
            }
            "f" => asleep = line[15..17].parse().unwrap(),
            "w" => res.push(Span {
                guard,
                asleep,
                awake: line[15..17].parse().unwrap(),
            }),
            _ => unreachable!(),
        }
    }

    res
}

fn group_guard_spans(spans: &Vec<Span>) -> HashMap<GuardId, Vec<(Time, Time)>> {
    let mut guard_spans = HashMap::new();

    for span in spans {
        let g = guard_spans.entry(span.guard).or_insert_with(|| Vec::new());
        g.push((span.asleep, span.awake));
    }
    guard_spans
}

#[aoc(day4, part1)]
pub fn part1(spans: &Vec<Span>) -> usize {
    let guard_spans = group_guard_spans(spans);

    let (_time, sleepiest_guard): (u16, &u16) = guard_spans
        .iter()
        .map(|(guard, spans)| {
            (((*spans).iter().map(|(s, w)| (w - s) as u16).sum()), guard) //flip them to use max
        })
        .max()
        .unwrap();

    let mut minutes: [u8; 60] = [0; 60];

    for (s, w) in guard_spans.get(sleepiest_guard).unwrap().iter() {
        for x in *s..*w {
            minutes[x as usize] += 1
        }
    }

    let mut sleepiest_minute = 0;
    let mut max = 0;
    for (m, t) in minutes.iter().enumerate() {
        if *t > max {
            sleepiest_minute = m;
            max = *t
        }
    }
    (*sleepiest_guard) as usize * sleepiest_minute
}

#[aoc(day4, part2)]
pub fn part2(spans: &Vec<Span>) -> usize {
    let guard_spans = group_guard_spans(spans);
    let mut sleepiest_minute = 0;
    let mut sleepiest_guard = 0;
    let mut max = 0;

    for (&g, spans) in guard_spans.iter() {
        let mut minutes: [u8; 60] = [0; 60];

        for (s, w) in spans.iter() {
            for x in *s..*w {
                minutes[x as usize] += 1
            }
        }

        for (m, &t) in minutes.iter().enumerate() {
            if t > max {
                sleepiest_minute = m;
                sleepiest_guard = g;
                max = t;
            }
        }
    }

    sleepiest_guard as usize * sleepiest_minute
}
