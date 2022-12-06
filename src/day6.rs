use std::collections::BTreeSet;

use nom::InputIter;

mod parser  {
    use nom::{IResult, character::complete::*, combinator::*};

    pub fn parse(input: &str) -> IResult<&str, &str> {
        let (input, data) = alpha1(input)?;
        let (input, _) = all_consuming(multispace0)(input)?;
        Ok((input, data))
    }
}

fn find_marker(data: &str, size: usize) -> Option<usize> {
    for i in size..=data.len() {
        let set = BTreeSet::from_iter((&data[i-size..i]).iter_elements());
        if set.len() == size {
            return Some (i)
        }
    }
    None
}

pub fn solve(input: &str) -> Option<(usize,usize)> {
    let (_,data) = parser::parse(input).unwrap();
    Some ((find_marker(data,4)?,find_marker(data,14)?))
}

#[test]
fn test6_1() {
    let solution = solve(&include_str!("../inputs/day6.1"));
    assert_eq!(solution, Some ((7,19)));
}

#[test]
fn test6_2() {
    let solution = solve(&include_str!("../inputs/day6.2"));
    assert_eq!(solution, Some ((5,23)));
}

#[test]
fn test6_3() {
    let solution = solve(&include_str!("../inputs/day6.3"));
    assert_eq!(solution, Some ((6,23)));
}

#[test]
fn test6_4() {
    let solution = solve(&include_str!("../inputs/day6.4"));
    assert_eq!(solution, Some ((10,29)));
}

#[test]
fn test6_5() {
    let solution = solve(&include_str!("../inputs/day6.5"));
    assert_eq!(solution, Some ((11,26)));
}

#[test]
fn test6_6() {
    let solution = solve(&include_str!("../inputs/day6.6"));
    assert_eq!(solution, Some ((1987,3059)));
}
