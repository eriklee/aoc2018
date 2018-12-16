use regex::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

#[macro_export]
macro_rules! try_opt {
    ($expr:expr) => {
        match $expr {
            Option::Some(val) => val,
            Option::None => return Option::None,
        }
    };
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Op {
    AddR,
    AddI,
    MulR,
    MulI,
    BanR,
    BanI,
    BorR,
    BorI,
    SetR,
    SetI,
    GtIR,
    GtRI,
    GtRR,
    EqIR,
    EqRI,
    EqRR,
}

type Register = u8;
type Machine = [MachineWord; 4];
type MachineWord = u16;

#[derive(Debug, Copy, Clone)]
struct Inst {
    opcode: Op,
    input1: u8,
    input2: u8,
    output: Register,
}

struct TestCase {
    precondition: Machine,
    step: (u8, u8, u8, u8),
    postcondition: Machine,
}

fn reg(state: Machine, reg: u8) -> MachineWord {
    assert!(reg < 4);
    state[reg as usize]
}

fn imm(imm: u8) -> MachineWord {
    MachineWord::from(imm)
}

fn step(state: Machine, inst: Inst) -> Machine {
    use self::Op::*;
    let output_reg = inst.output as usize;
    let res = match inst.opcode {
        AddR => reg(state, inst.input1) + reg(state, inst.input2),
        AddI => reg(state, inst.input1) + imm(inst.input2),
        MulR => reg(state, inst.input1) * reg(state, inst.input2),
        MulI => reg(state, inst.input1) * imm(inst.input2),
        BanR => reg(state, inst.input1) & reg(state, inst.input2),
        BanI => reg(state, inst.input1) & imm(inst.input2),
        BorR => reg(state, inst.input1) | reg(state, inst.input2),
        BorI => reg(state, inst.input1) | imm(inst.input2),
        SetR => reg(state, inst.input1),
        SetI => imm(inst.input1),
        GtIR => (imm(inst.input1) > reg(state, inst.input2)) as MachineWord,
        GtRI => (reg(state, inst.input1) > imm(inst.input2)) as MachineWord,
        GtRR => (reg(state, inst.input1) > reg(state, inst.input2)) as MachineWord,
        EqIR => (imm(inst.input1) == reg(state, inst.input2)) as MachineWord,
        EqRI => (reg(state, inst.input1) == imm(inst.input2)) as MachineWord,
        EqRR => (reg(state, inst.input1) == reg(state, inst.input2)) as MachineWord,
    };

    let mut next = state;
    next[output_reg] = res;
    next
}

type RawInst = (u8, u8, u8, u8);

#[aoc_generator(day16, part1)]
fn parse_test_cases(inp: &str) -> Vec<TestCase> {
    inp.lines()
        .collect::<Vec<&str>>()
        .chunks_exact(4)
        .filter_map(|ls| maybe_parse_test_cases(ls))
        .collect()
}

#[aoc_generator(day16, part2)]
fn parse_input(inp: &str) -> Box<(Vec<TestCase>, Vec<RawInst>)> {
    let test_cases: Vec<TestCase> = inp
        .lines()
        .collect::<Vec<&str>>()
        .chunks_exact(4)
        .filter_map(|ls| maybe_parse_test_cases(ls))
        .collect();

    let test_program = inp
        .lines()
        .skip(test_cases.len() * 4)
        .filter_map(maybe_parse_raw_inst)
        .collect();
    Box::new((test_cases, test_program))
}

fn maybe_parse_raw_inst(inp: &str) -> Option<RawInst> {
    lazy_static! {
        static ref INST_RE: Regex = Regex::new(r"(\d+) (\d+) (\d+) (\d+)").unwrap();
    }
    INST_RE.captures(inp).and_then(|caps| {
        Some((
            caps[1].parse().unwrap(),
            caps[2].parse().unwrap(),
            caps[3].parse().unwrap(),
            caps[4].parse().unwrap(),
        ))
    })
}
fn maybe_parse_test_cases(ls: &[&str]) -> Option<TestCase> {
    lazy_static! {
        static ref BEFORE_RE: Regex =
            Regex::new(r"Before: \[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
        static ref AFTER_RE: Regex = Regex::new(r"After:  \[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
    }
    let mut lines = ls.iter();
    let before: Machine =
        try_opt!(lines
            .next()
            .and_then(|l1| BEFORE_RE.captures(l1))
            .and_then(|caps| Some([
                caps[1].parse().unwrap(),
                caps[2].parse().unwrap(),
                caps[3].parse().unwrap(),
                caps[4].parse().unwrap(),
            ])));
    let inst: (u8, u8, u8, u8) = try_opt!(lines.next().and_then(|l| maybe_parse_raw_inst(l)));
    let after: Machine =
        try_opt!(lines
            .next()
            .and_then(|l3| AFTER_RE.captures(l3))
            .and_then(|caps| Some([
                caps[1].parse().unwrap(),
                caps[2].parse().unwrap(),
                caps[3].parse().unwrap(),
                caps[4].parse().unwrap(),
            ])));

    Some(TestCase {
        precondition: before,
        step: inst,
        postcondition: after,
    })
}

#[aoc(day16, part1)]
fn part1(tests: &[TestCase]) -> usize {
    tests
        .iter()
        .map(count_valid_codes)
        .filter(|n| *n >= 3)
        .count()
}

#[aoc(day16, part2)]
fn part2((tests, test_program): &(Vec<TestCase>, Vec<RawInst>)) -> MachineWord {
    let opcode_mapping = find_opcode_mapping(tests);
    let program = build_instns(&test_program, opcode_mapping);

    let result_state = run_instns(&program);
    result_state[0]
}

fn count_valid_codes(test: &TestCase) -> usize {
    get_valid_codes(test).len()
}

fn get_valid_codes(test: &TestCase) -> HashSet<Op> {
    use self::Op::*;
    static OPCODES: [Op; 16] = [
        AddR, AddI, MulR, MulI, BanR, BanI, BorR, BorI, SetR, SetI, GtIR, GtRI, GtRR, EqIR, EqRI,
        EqRR,
    ];

    OPCODES
        .iter()
        .filter(|&opcode| {
            let inst = Inst {
                opcode: *opcode,
                input1: test.step.1,
                input2: test.step.2,
                output: test.step.3,
            };
            check_valid(test.precondition, test.postcondition, inst)
        })
        .cloned()
        .collect()
}

fn find_opcode_mapping(tests: &[TestCase]) -> [Op; 16] {
    use self::Op::*;

    let mut hashmap: HashMap<u8, HashSet<Op>> = HashMap::new();
    for n in 0..16 {
        hashmap.insert(
            n,
            HashSet::from_iter(
                [
                    AddR, AddI, MulR, MulI, BanR, BanI, BorR, BorI, SetR, SetI, GtIR, GtRI, GtRR,
                    EqIR, EqRI, EqRR,
                ]
                .iter()
                .cloned(),
            ),
        );
    }

    for tc in tests {
        let number = tc.step.0;
        let prev_valids = &hashmap[&number];
        let valids = get_valid_codes(&tc);

        let new_valids = prev_valids.intersection(&valids).cloned().collect();
        hashmap.insert(number, new_valids);
    }

    // The resulting hashmap still doesn't have a unique mapping from byte to opcode
    let hashmap = clean_mapping(&mut hashmap);

    let mut mapping = [AddI; 16];
    for n in 0..16 {
        mapping[n] = hashmap[&(n as u8)];
    }
    mapping
}

fn build_instns(test_program: &[RawInst], mapping: [Op; 16]) -> Vec<Inst> {
    test_program
        .iter()
        .map(|&(op, input1, input2, output)| {
            let opcode = mapping[op as usize];
            Inst {
                opcode,
                input1,
                input2,
                output,
            }
        })
        .collect()
}

fn check_valid(start: Machine, end: Machine, inst: Inst) -> bool {
    let step_result = step(start, inst);
    step_result == end
}

// Run the given machine instructions beginning from the initial state,
// returning the final machine state
fn run_instns(instns: &[Inst]) -> Machine {
    let initial_state = [0; 4];
    instns.iter().fold(initial_state, |s, &i| step(s, i))
}

// Iteratively finds a 1-1 mapping and removes the value from every
// set in the collection. Eventually we find a unique mapping for each
// byte
fn clean_mapping(inp: &mut HashMap<u8, HashSet<Op>>) -> HashMap<u8, Op> {
    let mut res = HashMap::new();

    loop {
        let next_key = match inp.keys().find(|k| inp[k].len() == 1) {
            None => break,
            Some(next_key) => *next_key,
        };
        let op = *inp[&next_key].iter().nth(0).unwrap();
        for (_k, v) in inp.iter_mut() {
            v.remove(&op);
        }
        res.insert(next_key, op);
    }

    res
}
