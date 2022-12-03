use std::collections::BinaryHeap;

mod parser  {
    use nom::{IResult, multi::*, character::complete::*, combinator::*};

    pub fn parse(input: &[u8]) -> IResult<&[u8], Vec<Vec<i32>>> {
        let line= separated_list1(line_ending, i32);
        let (input, data) = separated_list1(count(line_ending, 2), line)(input)?;
        let (input, _) = all_consuming(multispace0)(input)?;
        Ok((input, data))
    }
}

pub fn solve(input: &[u8]) -> Option<(i32,i32)> {
    let (_,data) = parser::parse(input).ok()?;

    let calories = data.iter().map(|food| { food.iter().sum::<i32>() });
    let solution1 = calories.clone().max()?;

    let mut heap = BinaryHeap::from_iter(calories);
    let solution2 = heap.pop()? + heap.pop()? + heap.pop()?;

    Some ((solution1, solution2))
}

#[test]
fn test1_1() {
    let solution = solve(include_bytes!("../inputs/day1.1"));
    assert_eq!(solution, Some ((24000,45000)));
}

#[test]
fn test1_2() {
    let solution = solve(include_bytes!("../inputs/day1.2"));
    assert_eq!(solution, Some ((70720,207148)));
}
