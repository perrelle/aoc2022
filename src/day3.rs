use std::collections::HashSet;

mod parser  {
    use nom::{IResult, multi::*, character::complete::*, combinator::*};

    pub fn parse(input: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
        let (input, data) = separated_list1(line_ending, alpha1)(input)?;
        let (input, _) = all_consuming(multispace0)(input)?;
        Ok((input, data))
    }
}

pub fn value(item: u8) -> Option<u8> {
    Some (match item {
        b'a'..=b'z' => item - b'a' + 1,
        b'A'..=b'Z' => item - b'A' + 27,
        _ => return None
    })
}

pub fn solve(input: &[u8]) -> Option<(u32,u32)> {
    let (_,data) = parser::parse(input).ok()?;  

    let mut value1 = 0;

    for &line in &data {
        let (left, right) = line.split_at(line.len() / 2);
        let set_left: HashSet<u8> = left.iter().copied().collect();
        let set_right: HashSet<u8> = right.iter().copied().collect();
        let common = 
            set_left
                .intersection(&set_right).copied().collect::<Vec<u8>>();
        assert!(common.len() == 1);
        value1 += value(*common.first()?)? as u32;
    }

    let mut value2 = 0;
    for chunk in data.chunks(3) {
        match *chunk {
            [rucksack1, rucksack2, rucksack3] => {
                let set1: HashSet<u8> = rucksack1.iter().copied().collect();
                let set2: HashSet<u8> = rucksack2.iter().copied().collect();
                let set3: HashSet<u8> = rucksack3.iter().copied().collect();
                let common =
                    set1
                        .intersection(&set2).copied().collect::<HashSet<u8>>()
                        .intersection(&set3).copied().collect::<Vec<u8>>();
                assert!(common.len() == 1);
                value2 += value(*common.first()?)? as u32;
            }
            _ => return None
        }
    }
    
    Some ((value1, value2))
}

#[test]
fn test3_1() {
    let solution = solve(include_bytes!("../inputs/day3.1"));
    assert_eq!(solution, Some ((157,70)));
}

#[test]
fn test3_2() {
    let solution = solve(include_bytes!("../inputs/day3.2"));
    assert_eq!(solution, Some ((7908,2838)));
}
