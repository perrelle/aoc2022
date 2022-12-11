use std::collections::BinaryHeap;

#[derive (Debug,Clone)]
pub enum Operand {
    Old,
    Value(u32)
}

#[derive (Debug,Clone)]
pub enum Op {
    Mul(Operand, Operand),
    Add(Operand, Operand)
}

#[derive (Debug,Clone)]
pub enum Test {
    DivisibleBy(u32)
}

#[derive (Debug,Clone)]
pub struct Monkey {
    id: usize,
    items: Vec<u64>,
    operation: Op,
    test: Test,
    throw_to_if_true: usize,
    throw_to_if_false: usize,
    inspections: u32
}


mod parser {
    use nom::{
        IResult,
        character::complete::*,
        combinator::*,
        sequence::*,
        branch::*,
        bytes::complete::tag,
        multi::*,
        error::ParseError};

    use super::*;

    fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) ->
        impl FnMut(&'a str) -> IResult<&'a str, O, E>
        where F: FnMut(&'a str) -> IResult<&'a str, O, E>,
    {
        delimited(multispace0, inner, multispace0)
    }

    fn usize(input: &str) -> IResult<&str, usize> {
        map(u32, |x| x as usize)(input)
    }

    fn operand(input: &str) -> IResult<&str, Operand> {
        let old = value(Operand::Old, tag("old"));
        let val = map(u32, Operand::Value);
        alt((old, val))(input)
    }

    fn op(input: &str) -> IResult<&str, Op> {
        let (input, (_,o1,c,o2)) = tuple(
            (tag("new = "), operand, ws(one_of("+*")), operand))(input)?;
        let r = match c {
            '+' => Op::Add(o1,o2),
            '*' => Op::Mul(o1,o2),
            _ => panic!()
        };
        Ok((input, r))
    }

    fn monkey(input: &str) -> IResult<&str, Monkey> {
        let test = map(preceded(tag("divisible by "), u32), Test::DivisibleBy);

        let (input,id) = ws(delimited(
            tag("Monkey "), usize, tag(":")))(input)?;
        let (input, items) = ws(preceded(
            tag("Starting items: "), 
            separated_list1(tag(", "), u64)))(input)?;
        let (input, operation) = ws(preceded(tag("Operation: "), op))(input)?;
        let (input, test) = ws(preceded(tag("Test: "), test))(input)?;
        let (input, throw_to_if_true) = ws(preceded(
            tag("If true: throw to monkey "),
            usize))(input)?;
        let (input, throw_to_if_false) = ws(preceded(
            tag("If false: throw to monkey "),
            usize))(input)?;
    
        let m = Monkey {
            id, items, operation, test,
            throw_to_if_true, throw_to_if_false,
            inspections: 0
        };
        Ok((input, m))
    }

    pub fn parse(input: &str) -> IResult<&str, Vec<Monkey>> {
        let monkeys = many1(ws(monkey));
        all_consuming(terminated(monkeys, multispace0))(input)
    }
}

fn apply(op: &Op, old_w: u64) -> u64 {
    let eval = |x: &Operand| {
        match *x {
            Operand::Old => old_w,
            Operand::Value(x) => x as u64
        }
    };
    match op {
        Op::Mul(l, r) => eval(l) * eval(r),
        Op::Add(l, r) => eval(l) + eval(r)
    }
}

fn test(t: &Test, x: u64) -> bool {
    match t {
        &Test::DivisibleBy(y) => x % (y as u64) == 0
    }
}

pub fn solve_part(mut monkeys: Vec<Monkey>, worry_decreases: bool, rounds: u32) -> Option<u64> {
    for _i in 1..=rounds {
        for i in 0..monkeys.len() {
            let m = monkeys.get_mut(i)?;
            let mut items_t = Vec::new();
            let mut items_f = Vec::new();

            for item in &m.items {
                m.inspections += 1;
                let mut w = *item;
                w = apply(&m.operation, w);
                if worry_decreases {
                    w /= 3;
                }
                else {
                    w %= 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19 * 23;
                }
                if test(&m.test, w) {
                    items_t.push(w);
                }
                else {
                    items_f.push(w);
                }
            }

            let dest_t = m.throw_to_if_true;
            let dest_f = m.throw_to_if_false;
            m.items.clear();

            monkeys.get_mut(dest_t)?.items.append(&mut items_t);
            monkeys.get_mut(dest_f)?.items.append(&mut items_f);            
        }

        if _i % 1000 == 0 || _i == 20 || _i == 1 {
            println!("== After round {_i} ==");
            for m in &monkeys {
                println!("Monkey {} inspected  items {} times", m.id, m.inspections);
            }
        }
    }

    let mut inspections = BinaryHeap::from_iter(monkeys.iter().map(|m| m.inspections));
    Some (inspections.pop()? as u64 * inspections.pop()? as u64)
}

pub fn solve(input: &str) -> Option<(u64,u64)> {
    let (_,monkeys) = parser::parse(input).unwrap();
    let solution1 = solve_part(monkeys.clone(), true, 20)?;
    let solution2 = solve_part(monkeys, false, 10000 )?;
    Some((solution1, solution2))
}

#[test]
fn test11_1() {
    let solution = solve(&include_str!("../inputs/day11.1"));
    assert_eq!(solution, Some((10605,2713310158)));
}

#[test]
fn test11_2() {
    let solution = solve(&include_str!("../inputs/day11.2"));
    assert_eq!(solution, Some((117624,16792940265)));
}
