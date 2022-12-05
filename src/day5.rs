pub struct Command {
    count: usize,
    src: usize,
    dst: usize
}

pub type Stacks = Vec<Vec<char>>;

mod parser  {
    use nom::{
        IResult, branch::*, multi::*,
        combinator::*, sequence::*,
        character::complete::*,
        bytes::complete::tag
    };
    use crate::day5::Command;

    fn usize(input: &str) -> IResult<&str, usize> {
        let (input, i) = u32(input)?;
        Ok((input, i as usize))
    }

    fn space(input: &str)  -> IResult<&str, char> {
        char(' ')(input)
    }

    fn one_crate(input: &str) -> IResult<&str, char> {
        alt((
            delimited(char('['), satisfy(|c| c.is_uppercase()), char(']')),
            delimited(space, space, space)
        ))(input)
    }

    fn crate_array(input: &str) -> IResult<&str, Vec<Vec<char>>> {
        let (input, array) =
            separated_list1(line_ending, separated_list1(space, one_crate))
                (input)?;
        // Legend
        let (input, _) = line_ending(input)?;
        let (input, _) =
            separated_list1(space, delimited(space, u32, space))
                (input)?;
        Ok ((input, array))
    }

    fn command(input: &str) -> IResult<&str, Command> {
        let (input,(_,count,_,src,_,dst)) = tuple((
            tag("move "), usize, tag(" from "), usize, tag(" to "), usize
        ))(input)?;
        Ok((input,Command { count, src, dst }))
    }

    pub fn parse(input: &str) -> IResult<&str, (Vec<Vec<char>>,Vec<Command>)> {
        let (input, crates) = crate_array(input)?;
        let (input, _) = multispace1(input)?;
        let (input, commands) = separated_list1(line_ending, command)(input)?;
        let (input, _) = all_consuming(multispace0)(input)?;
        Ok((input, (crates,commands)))
    }
}


fn execute_9000(command: &Command, stacks: &mut Stacks) {
    for _i in 0..command.count {
        let c = stacks[command.src-1].pop().unwrap();
        stacks[command.dst-1].push(c);
    }
}

fn execute_9001(command: &Command, stacks: &mut Stacks) {
    let n = command.count;
    let src = &mut stacks[command.src-1];
    let moved_crates = src.drain((src.len()-n)..).collect::<Vec<char>>();
    let dst = &mut stacks[command.dst-1];
    dst.extend(moved_crates);
}

fn build_stacks(array: Vec<Vec<char>>) -> Stacks {
    let mut stacks: Stacks = Vec::new ();

    for line in array {
        for (i,&c) in line.iter().enumerate() {
            if c != ' ' {
                if stacks.len() <= i {
                    stacks.resize(i + 1, Vec::new ());
                }
                stacks[i].push(c);
            }
        }
    }

    stacks.iter_mut().for_each(|v| v.reverse());
    stacks
}

fn stacks_top(stacks: Stacks) -> String {
    stacks.iter().map(|v| v.last().unwrap()).collect::<String>()
}

pub fn solve(input: &str) -> Option<(String,String)> {
    let (_,(array, commands)) = parser::parse(input).unwrap();

    let stacks = build_stacks(array);  
    let mut stacks1 = stacks.clone();

    for command in &commands {
        execute_9000(command, &mut stacks1);
    }

    let mut stacks2 = stacks;
    for command in &commands {
        execute_9001(command, &mut stacks2);
    }

    Some ((stacks_top(stacks1), stacks_top(stacks2)))
}

#[test]
fn test5_1() {
    let solution = solve(&include_str!("../inputs/day5.1"));
    assert_eq!(solution, Some ((String::from("CMZ"),String::from("MCD"))));
}

#[test]
fn test5_2() {
    let solution = solve(&include_str!("../inputs/day5.2"));
    assert_eq!(solution, Some ((String::from("BZLVHBWQF"),String::from("TDGJQTZSL"))));
}
