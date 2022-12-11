use std::collections::HashSet;

pub enum Direction { Left, Right, Up, Down }

pub type Command = (Direction, u32);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}


mod parser {
    use nom::{
        IResult,
        character::complete::*,
        combinator::*,
        sequence::*,
        multi::*};

    use super::*;

    pub fn direction(input: &str) -> IResult<&str, Direction> {
        let (input,c) = one_of("LRUD")(input)?;
        let d = match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            'U' => Direction::Up,
            'D' => Direction::Down,
            _ => panic!()
        };
        Ok((input,d))
    }

    pub fn parse(input: &str) -> IResult<&str, Vec<Command>> {
        let command = separated_pair(direction, space1, u32);
        let commands = separated_list1(line_ending, command);
        all_consuming(terminated(commands, multispace0))(input)
    }
}


fn move_head(p: Point, d: &Direction) -> Point {
    match d {
        Direction::Left => Point {x: p.x-1, y: p.y},
        Direction::Right => Point {x: p.x+1, y: p.y},
        Direction::Up => Point {x: p.x, y: p.y+1},
        Direction::Down => Point {x: p.x, y: p.y-1}
    }
}

fn distance(p1: &Point, p2: &Point) -> i32 {
    (p1.x - p2.x).abs().max((p1.y - p2.y).abs())
}

fn move_tail(head: &Point, tail: Point) -> Point {
    if distance(head, &tail) <= 1 {
        tail
    }
    else {
        let x = 
            if tail.x < head.x {
                tail.x + 1
            }
            else if tail.x > head.x {
                tail.x - 1
            }
            else {
                tail.x
            };
        let y =
            if tail.y < head.y {
                tail.y + 1
            }
            else if tail.y > head.y {
                tail.y - 1
            }
            else {
                tail.y
            };
        Point {x, y}
    }
}

pub fn simulate(commands: &Vec<Command>, length: usize) -> usize {
    let mut rope = vec![Point {x:0, y:0}; length];
    let mut visited_cells = HashSet::<Point>::new();

    for (d,n) in commands {
        for _i in 0..*n {
            let mut prev = None;

            for k in rope.iter_mut() {
                if let Some(prev_k) = prev {
                    *k = move_tail(prev_k, *k);
                }
                else {
                    *k = move_head(*k, d);
                }
                prev = Some(k)
            }
            visited_cells.insert(*rope.last().unwrap());
        }
    }

    visited_cells.len()
}


pub fn solve(input: &str) -> Option<(usize,usize)> {
    let (_,commands) = parser::parse(input).unwrap();
    Some ((simulate(&commands,2),simulate(&commands,10)))
}

#[test]
fn test9_1() {
    let solution = solve(&include_str!("../inputs/day9.1"));
    assert_eq!(solution, Some ((13,1)));
}

#[test]
fn test9_2() {
    let solution = solve(&include_str!("../inputs/day9.2"));
    assert_eq!(solution, Some ((88,36)));
}

#[test]
fn test9_3() {
    let solution = solve(&include_str!("../inputs/day9.3"));
    assert_eq!(solution, Some ((6181, 2386)));
}
