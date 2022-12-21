use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum Operator { Add, Sub, Mul, Div }

#[derive(Debug)]
pub enum Exp {
    Number(i32),
    Unknown,
    Op(Operator, Box<Exp>, Box<Exp>),
}

mod parser {
    use nom::{
        IResult,
        character::complete::*,
        combinator::*,
        sequence::*,
        multi::*,
        branch::*,
        bytes::complete::*
    };

    use super::*;

    #[derive(Debug, Clone)]
    pub enum Job<'a> { Number(i32), Op(Operator, &'a str, &'a str) }
    pub type Monkey<'a> = (&'a str, Job<'a>);

    pub fn parse(input: &str) -> IResult<&str, Vec<Monkey>> {
        let operator = map(one_of("+-*/"), |c| match c {
            '+' => Operator::Add,
            '-' => Operator::Sub,
            '*' => Operator::Mul,
            '/' => Operator::Div,
            _ => panic!()
        });
        let operator = delimited(space0, operator, space0);
        let number = map(i32, Job::Number);
        let op = map(tuple((alpha1, operator, alpha1)), |(e1, o, e2)|
            Job::Op(o, e1, e2));
        let job = alt((number, op)); 
        let monkey = separated_pair(alpha1, tag(": "), job);
        let data = separated_list1(multispace1, monkey);
        all_consuming(terminated(data, multispace0))(input)
    }
}

fn build_exp(monkeys: &[parser::Monkey], root: &str, unknown: Option<&str>)
        -> Exp
{
    type Dictionary<'a> = HashMap<&'a str, parser::Job<'a>>;
    let dictionary: Dictionary = monkeys.iter().cloned().collect();

    fn build(dictionary: &Dictionary, root: &str, unknown: Option<&str>) -> Exp {
        if Some(root) == unknown {
            Exp::Unknown
        }
        else {
            match dictionary.get(root).unwrap() {
                parser::Job::Number(i) => Exp::Number(*i),
                parser::Job::Op(op, m1, m2) => Exp::Op(
                    *op,
                    Box::new(build(dictionary, m1, unknown)),
                    Box::new(build(dictionary, m2, unknown))) 
            }
        }
    }

    build(&dictionary, root, unknown)
}

fn eval(exp: &Exp) -> Option<i64> {
    Some(match exp {
        Exp::Number(i) => *i as i64,
        Exp::Unknown => return None,
        Exp::Op(Operator::Add, e1, e2) => eval(e1)? + eval(e2)?,
        Exp::Op(Operator::Sub, e1, e2) => eval(e1)? - eval(e2)?,
        Exp::Op(Operator::Mul, e1, e2) => eval(e1)? * eval(e2)?,
        Exp::Op(Operator::Div, e1, e2) => eval(e1)? / eval(e2)?,
    })
}

fn solve_equality(exp: &Exp, i: i64) -> i64 {
    match exp {
        Exp::Number(_) => panic!(),
        Exp::Unknown => i,
        Exp::Op(op, e1, e2) => {
            match (eval(e1), eval(e2)) {
                (None, None) | (Some(_), Some(_)) => panic!(),
                (Some(j), None) => {     
                    match op {
                        Operator::Add => solve_equality(e2, i - j),
                        Operator::Sub => solve_equality(e2, j - i),
                        Operator::Mul => solve_equality(e2, i / j),
                        Operator::Div => solve_equality(e2, j / i),
                    }
                }
                (None, Some(j)) => {
                    match op {
                        Operator::Add => solve_equality(e1, i - j),
                        Operator::Sub => solve_equality(e1, i + j),
                        Operator::Mul => solve_equality(e1, i / j),
                        Operator::Div => solve_equality(e1, i * j),
                    }
                }
            }
        }    
    }
}

fn solve_equation(e1: &Exp, e2: &Exp) -> i64 {
    match (eval(e1), eval(e2)) {
        (None, None) | (Some(_), Some(_)) => panic!(),
        (Some(i), None) => solve_equality(e2, i),
        (None, Some(i)) => solve_equality(e1, i)
    }
}

pub fn solve(input: &str) -> (i64,i64) {
    let (_,data) = parser::parse(input).unwrap();

    let exp1 = build_exp(&data, "root", None);
    let solution1 = eval(&exp1).unwrap();

    let exp2 = build_exp(&data, "root", Some("humn"));
    let solution2 =
        if let Exp::Op(_,e1,e2) = exp2 {
            solve_equation(&e1, &e2)
        }
        else {
            panic!()
        };

    (solution1, solution2)
}

#[test]
fn test21_1() {
    let solution = solve(&include_str!("../inputs/day21.1"));
    assert_eq!(solution, (152, 301));
}

#[test]
fn test21_2() {
    let solution = solve(&include_str!("../inputs/day21.2"));
    assert_eq!(solution, (54703080378102, 3952673930912));
}
