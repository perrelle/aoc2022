use std::{slice, cmp::Ordering, fmt};

#[derive (Debug, PartialEq, Eq, Clone)]
pub enum Packet {
    Int(i32),
    List(Vec<Packet>)
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        fn cmp_packet(p1: &Packet, p2: &Packet) -> Ordering {
            match (p1,p2) {
                (Packet::Int(i1), Packet::Int(i2)) =>
                    i1.cmp(i2),
                (Packet::Int(_), Packet::List(l2)) =>
                    cmp_slices(slice::from_ref(p1), l2),
                (Packet::List(l1), Packet::Int(_)) =>
                    cmp_slices(l1, slice::from_ref(p2)),
                (Packet::List(l1), Packet::List(l2)) =>
                    cmp_slices(l1, l2)
            }
        }

        fn cmp_slices(l1: &[Packet], l2: &[Packet]) -> Ordering {
            match (l1.split_first(), l2.split_first()) {
                (None, None) => Ordering::Equal,
                (None, Some (_)) => Ordering::Less,
                (Some (_) , None) => Ordering::Greater,
                (Some ((h1,t1)), Some ((h2,t2))) => {
                    let r = cmp_packet(h1, h2);
                    if r != Ordering::Equal {
                        r
                    }
                    else {
                        cmp_slices(t1, t2)
                    }
                }
            }
        }
        
        cmp_packet(self, other)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Packet::Int(i) => write!(f, "{i}"),
            Packet::List(l) => {
                let mut first = true;
                write!(f, "[")?;
                for p in l {
                    if first {
                        first = false
                    }
                    else {
                        write!(f, ", ")?;
                    }
                    p.fmt(f)?;
                }
                write!(f, "]")
            }
        }
    }
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

    fn packet(input: &str) -> IResult<&str, Packet> {
        alt((
            map(i32, Packet::Int),
            map(
                delimited(
                    tag("["), 
                    separated_list0(tag(","), packet),
                    tag("]")),
                Packet::List)))(input)
    }

    pub fn parse(input: &str) -> IResult<&str, Vec<Packet>> {
        let data = separated_list1(multispace1, packet);
        all_consuming(terminated(data, multispace0))(input)
    }
}

pub fn solve(input: &str) -> Option<(u32,u32)> {
    let (_,data) = parser::parse(input).unwrap();
    let mut solution1 = 0;

    for (i,chunk) in data.chunks(2).enumerate() {
        let (p1,p2) = match chunk {
            [p1,p2] => (p1,p2),
            _ => panic!()
        };
        let c = p1.partial_cmp(p2);
        if c == Some(Ordering::Less) {
            solution1 += (i as u32) + 1;
        }
        else if c == Some(Ordering::Greater) {
            // Wrong order
        }
        else {
            println!("{p1} {c:?} {p2}");
            panic!()
        }
    }

    let mut signal = data;
    let divider1 = Packet::List(vec![Packet::Int(2)]);
    let divider2 = Packet::List(vec![Packet::Int(6)]);

    signal.push(divider1.clone());
    signal.push(divider2.clone());
    signal.sort();

    let solution2 =
        ((signal.iter().position(|x| x == &divider1)? + 1) *
        (signal.iter().position(|x| x == &divider2)? + 1)) as u32;

    for p in signal {
        println!("{p}");
    }

    Some((solution1, solution2))
}

#[test]
fn test13_1() {
    let solution = solve(&include_str!("../inputs/day13.1"));
    assert_eq!(solution, Some((13,140)));
}

#[test]
fn test13_2() {
    let solution = solve(&include_str!("../inputs/day13.2"));
    assert_eq!(solution, Some((6478,21922)));
}
