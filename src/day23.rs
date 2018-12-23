use lazy_static::*;
use regex::*;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Bot {
    x: i64,
    y: i64,
    z: i64,
    rad: u64,
}

#[aoc_generator(day23)]
fn parse_nanobots(inp: &str) -> Vec<Bot> {
    lazy_static! {
        static ref INP_RE: Regex = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
    }
    INP_RE
        .captures_iter(inp)
        .map(|cap| Bot {
            x: cap[1].parse().unwrap(),
            y: cap[2].parse().unwrap(),
            z: cap[3].parse().unwrap(),
            rad: cap[4].parse().unwrap(),
        })
        .collect()
}

#[aoc(day23, part1)]
fn part1(inp: &[Bot]) -> usize {
    let strongest = inp.iter().max_by_key(|b| b.rad).unwrap();
    println!(
        "Strongest: ({},{},{}) {}",
        strongest.x, strongest.y, strongest.z, strongest.rad
    );

    inp.iter()
        .filter(|b| distance(&strongest, b) <= strongest.rad)
        .count()
}

fn distance(a: &Bot, b: &Bot) -> u64 {
    ((a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs()) as u64
}

#[aoc(day23, part2)]
fn part2(inp: &[Bot]) -> u64 {
    let (x, y, z) = run_z3(inp);

    println!("Solved!: {}, {}, {}", x, y, z);

    let origin_bot = Bot {
        x: 0,
        y: 0,
        z: 0,
        rad: 0,
    };
    let fake_bot = Bot { x, y, z, rad: 0 };
    let count = inp
        .iter()
        .filter(|b| distance(&fake_bot, b) <= b.rad)
        .count();
    println!("{} bots in range", count);
    distance(&origin_bot, &fake_bot)
}

fn run_z3(bots: &[Bot]) -> (i64, i64, i64) {
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let opt = z3::Optimize::new(&ctx);

    let zero = ctx.from_i64(0);
    let one = ctx.from_i64(1);

    let x = ctx.named_int_const("x");
    let y = ctx.named_int_const("y");
    let z = ctx.named_int_const("z");

    fn abs<'a>(zero: &z3::Ast<'a>, n: &z3::Ast<'a>) -> z3::Ast<'a> {
        n.le(zero).ite(&zero.sub(&[n]), n)
    }

    let mut dist_asts: HashMap<Bot, z3::Ast> = HashMap::new();
    for bot in bots {
        let ast = ctx.fresh_int_const("bot");
        let x_dist = ctx.fresh_int_const("x_dist");
        let x_abs = abs(&zero, &x_dist);
        opt.assert(&x_dist._eq(&x.sub(&[&ctx.from_i64(bot.x)])));
        opt.assert(&x_abs._eq(&abs(&zero, &x_dist)));
        let y_dist = ctx.fresh_int_const("y_dist");
        let y_abs = ctx.fresh_int_const("y_abs");
        opt.assert(&y_dist._eq(&y.sub(&[&ctx.from_i64(bot.y)])));
        opt.assert(&y_abs._eq(&abs(&zero, &y_dist)));
        let z_dist = ctx.fresh_int_const("z_dist");
        let z_abs = ctx.fresh_int_const("z_abs");
        opt.assert(&z_dist._eq(&z.sub(&[&ctx.from_i64(bot.z)])));
        opt.assert(&z_abs._eq(&abs(&zero, &z_dist)));
        let fin = ast._eq(
            &zero
                .add(&[&x_abs, &y_abs, &z_abs])
                .le(&ctx.from_i64(bot.rad as i64))
                .ite(&one, &zero),
        );
        opt.assert(&fin);
        dist_asts.insert(*bot, ast);
    }
    // 138697281
    let count_ast = &ctx
        .from_i64(0)
        .add(&dist_asts.values().collect::<Vec<&z3::Ast>>());
    opt.maximize(&count_ast);
    opt.minimize(&zero.add(&[&abs(&zero, &x), &abs(&zero, &y), &abs(&zero, &z)]));

    assert!(opt.check());
    let model = opt.get_model();

    let xv = model.eval(&x).unwrap().as_i64().unwrap();
    let yv = model.eval(&y).unwrap().as_i64().unwrap();
    let zv = model.eval(&z).unwrap().as_i64().unwrap();

    (xv, yv, zv)
}
