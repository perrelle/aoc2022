use std::fmt;

#[cfg(test)]
use assert_str::assert_str_trim_eq;

#[derive (Clone)]
pub enum Instruction { Noop, AddX(i32) }


mod parser {
    use nom::{
        IResult,
        character::complete::*,
        combinator::*,
        sequence::*,
        branch::*,
        bytes::complete::tag,
        multi::*};

    use super::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
        let noop = value(Instruction::Noop, tag("noop"));
        let addx = map(
            tuple((tag("addx"), space1, i32)),
            |(_,_,i)| Instruction::AddX(i));
        let instruction =  alt((noop, addx));
        let instructions = separated_list1(line_ending, instruction);
        all_consuming(terminated(instructions, multispace0))(input)
    }
}

struct Machine {
    cycle: u64,
    x: i32,
    screen: [[bool; 40]; 6],
    solution1: i64
}

impl Machine {
    fn new() -> Machine {
        let mut m = Machine {
            cycle: 1,
            x: 1,
            screen: [[false; 40]; 6], solution1: 0 
        };
        m.crt_draw();
        m
    }

    fn crt_draw(&mut self) {
        let x = ((self.cycle - 1) % 40) as usize;
        let y = ((self.cycle - 1) / 40) as usize;
        if y < 6 {
            self.screen[y][x] = (self.x - x as i32).abs() <= 1;
        }
    }

    fn tick(&mut self) {
        self.cycle += 1;

        if self.cycle % 40 == 20 {
            let signal_strength = self.cycle as i64 * self.x as i64;
            self.solution1 += signal_strength;
        }

        self.crt_draw();
    }

    fn noop(&mut self) {
        self.tick()
    }

    fn add_x(&mut self, c: i32) {
        self.tick();
        self.x += c;
        self.tick();
    }

    fn execute(&mut self, i: Instruction) {
        match i {
            Instruction::Noop => self.noop(),
            Instruction::AddX(c) => self.add_x(c)
        }
    }
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.screen {
            for pixel in line {
                write!(f, "{}", if pixel {'#'} else {'.'})?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn solve(input: &str) -> (i64,String) {
    let (_,instructions) = parser::parse(input).unwrap();

    let mut m = Machine::new();

    for i in instructions {
        m.execute(i);
    }

    let screen = m.to_string();
    println!("{screen}");

    (m.solution1,screen)
}

#[test]
fn test10_1() {
    let (solution1,solution2) = solve(&include_str!("../inputs/day10.1"));
    let oracle2 = include_str!("../inputs/day10.1.oracle");
    assert_eq!(solution1, 13140);
    assert_str_trim_eq!(solution2, String::from(oracle2));
}

#[test]
fn test10_2() {
    let (solution1,solution2) = solve(&include_str!("../inputs/day10.2"));
    let oracle2 = include_str!("../inputs/day10.2.oracle");
    assert_eq!(solution1, 13520);
    assert_str_trim_eq!(solution2, String::from(oracle2));
}
