use lazy_static::*;
use regex::*;

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

type MachineWord = u64;
type Register = u8;
type Registers = [MachineWord; 6];

struct Machine {
    registers: Registers,
    program: Vec<Inst>,
    ip: usize,
    ip_reg: usize,
}

#[derive(Debug, Copy, Clone)]
struct Inst {
    opcode: Op,
    input1: u8,
    input2: u8,
    output: Register,
}

fn reg(state: Registers, reg: u8) -> MachineWord {
    assert!(reg < 6);
    state[reg as usize]
}

fn imm(imm: u8) -> MachineWord {
    MachineWord::from(imm)
}

impl Machine {
    fn step(self: &mut Self) -> bool {
        use self::Op::*;

        let inst = match self.program.get(self.ip) {
            None => return true,
            Some(i) => i,
        };

        self.registers[self.ip_reg] = self.ip as MachineWord;

        let state = self.registers;
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

        self.registers[output_reg] = res;
        self.ip = self.registers[self.ip_reg] as usize + 1;

        false
    }
}

#[aoc_generator(day19)]
fn parse_input(inp: &str) -> Box<(usize, Vec<Inst>)> {
    let ip_reg = inp.lines().next().unwrap().chars().nth(4).unwrap() as u8 - b'0';

    let test_program = inp.lines().skip(1).filter_map(maybe_parse_inst).collect();

    Box::new((ip_reg as usize, test_program))
}

fn parse_op(inp: &str) -> Op {
    use self::Op::*;

    match inp {
        "addr" => AddR,
        "addi" => AddI,
        "mulr" => MulR,
        "muli" => MulI,
        "banr" => BanR,
        "bani" => BanI,
        "borr" => BorR,
        "bori" => BorI,
        "setr" => SetR,
        "seti" => SetI,
        "gtir" => GtIR,
        "gtri" => GtRI,
        "gtrr" => GtRR,
        "eqir" => EqIR,
        "eqri" => EqRI,
        "eqrr" => EqRR,
        _ => unreachable!(),
    }
}

fn maybe_parse_inst(inp: &str) -> Option<Inst> {
    lazy_static! {
        static ref INST_RE: Regex = Regex::new(r"([[:alpha:]]+) (\d+) (\d+) (\d+)").unwrap();
    }

    INST_RE.captures(inp).and_then(|caps| {
        Some(Inst {
            opcode: parse_op(&caps[1]),
            input1: caps[2].parse().unwrap(),
            input2: caps[3].parse().unwrap(),
            output: caps[4].parse().unwrap(),
        })
    })
}

#[aoc(day19, part1)]
fn part1((ip, instns): &(usize, Vec<Inst>)) -> MachineWord {
    run_instns(*ip, instns).registers[0]
}

#[aoc(day19, part2)]
fn part2((ip, instns): &(usize, Vec<Inst>)) -> MachineWord {
    run_instns_p2(*ip, instns);

    13_083_798
}

// Run the given machine instructions beginning from the initial state,
// returning the final machine state
fn run_instns(ip_reg: usize, instns: &[Inst]) -> Machine {
    let initial_state = Machine {
        ip_reg,
        ip: 0,
        registers: [0; 6],
        program: instns.to_vec(),
    };

    let mut state = initial_state;
    while !state.step() {}

    state
}

// Run the given machine instructions beginning from the initial state,
// returning the final machine state
// After looking at the code we were looping for, it was clear it was performing the following
// operation (pseudo code)
// for B in 0..D {
//    for C in 1..D {
//      if B * C = D { A += B }
//    }
// }
// I.e. finding the sum of all factors of D (10551425 in my case).
// So I just did it in haskell with an O(n) filter instead of an N^2 double loop
fn run_instns_p2(ip_reg: usize, instns: &[Inst]) -> Machine {
    let mut registers = [0; 6];
    registers[0] = 1;
    // 13083798
    let initial_state = Machine {
        ip_reg,
        ip: 0,
        registers,
        program: instns.to_vec(),
    };

    let mut state = initial_state;
    for _ in 0..1000 {
        state.step();
    }

    state
}
