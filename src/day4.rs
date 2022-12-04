type Interval = (u32,u32);

mod parser  {
    use nom::{IResult, multi::*, character::complete::*, combinator::*, sequence::separated_pair};
    use crate::day4::Interval;

    pub fn parse(input: &str) -> IResult<&str, Vec<(Interval,Interval)>> {
        fn pair(input: &str) -> IResult<&str, Interval> {
            separated_pair(u32, char('-'), u32)(input)
        }
        let entry = separated_pair(pair, char(','), pair);
        let (input, data) = separated_list1(line_ending, entry)(input)?;
        let (input, _) = all_consuming(multispace0)(input)?;
        Ok((input, data))
    }
}

pub fn includes(&(l1,u1): &Interval, &(l2,u2): &Interval) -> bool {
    l2 >= l1 && u2 <= u1
}

pub fn either_includes(x1: &Interval, x2: &Interval) -> bool {
    includes(x1, x2) || includes(x2, x1)
}

pub fn overlaps(&(l1,u1): &Interval, &(l2,u2): &Interval) -> bool {
    u1 >= l2 && l1 <= u2
}

pub fn solve(input: &str) -> Option<(u32,u32)> {
    let (_,data) = parser::parse(input).unwrap();

    let count1 = data.iter().filter(|(x1,x2)| either_includes(x1,x2)).count();
    let count2 = data.iter().filter(|(x1,x2)| overlaps(x1,x2)).count();

    Some ((count1 as u32, count2 as u32))
}

#[test]
fn test4_1() {
    let solution = solve(&include_str!("../inputs/day4.1"));
    assert_eq!(solution, Some ((2,4)));
}

#[test]
fn test4_2() {
    let solution = solve(&include_str!("../inputs/day4.2"));
    assert_eq!(solution, Some ((500,815)));
}
