use std::{collections::{HashMap, BTreeSet}};

#[derive (Debug)]
pub struct Valve {
    name: String,
    flow_rate: u32,
    neighbors: Vec<usize>,
    distance: HashMap<usize, u32>
}

mod parser {
    use nom::{
        IResult,
        character::complete::*,
        combinator::*,
        sequence::*,
        multi::*,
        bytes::complete::*,
        branch::*
    };

    pub fn parse(input: &str) -> IResult<&str, Vec<(&str, u32, Vec<&str>)>> {

        let valve = map(tuple((
            tag("Valve "), alpha1,
            tag(" has flow rate="), u32,
            alt((tag("; tunnels lead to valves "), tag("; tunnel leads to valve "))),
            separated_list1(tag(", "), alpha1))),
            |(_,name,_,flow_rate,_,neighbors)| 
                (name, flow_rate, neighbors));
        let data = separated_list1(line_ending, valve);
        all_consuming(terminated(data, multispace0))(input)
    }
}

type VertexSet = BTreeSet<usize>;

static mut COUNT: i32 = 2000000000;

fn dfs(valves: &[Valve], current: usize, remaining: u32, closed: &VertexSet) -> u32 {
    let mut closed = closed.clone();
    closed.insert(current);

    let mut best = 0;

    unsafe {
        COUNT -= 1;
        if COUNT < 0 {
            panic!()
        }
    }

    for (n,d) in &valves[current].distance {
        if remaining > *d && !closed.contains(&n) {
            let r = remaining - *d - 1;
            // println!("{} -> {}, for {} minutes, {} remaining, (rest: {:?})", current, n, d, r, closed);
            let new_best =
                (r * valves[*n].flow_rate) +
                dfs(valves, *n, r, &closed);
            if new_best > best {
                best = new_best;
            }
        }
    }

    best
}

pub fn solve(input: &str) -> (u32,i64) {
    let (_,data) = parser::parse(input).unwrap();

    let mut valves = Vec::new();
    let mut names = HashMap::new();

    for (name, flow_rate, _) in &data {
        names.insert(name, valves.len());
        valves.push(Valve {
            name: String::from(*name),
            flow_rate: *flow_rate,
            neighbors: Vec::new(),
            distance: HashMap::new()
         });
    }

    for (name, _, neigbors) in &data {
        let i = *names.get(name).unwrap();
        for n in neigbors {
            let j = *names.get(n).unwrap();
            valves[i].neighbors.push(j);
            valves[i].distance.insert(j, 1);
        }
    }

    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                let current_dist = valves[i].distance.get(&j);
                let shortcut_dist =
                    valves[i].distance.get(&k).unwrap_or(&1000) +
                    valves[k].distance.get(&j).unwrap_or(&1000);

                if let Some(&d) = current_dist {
                    if d <= shortcut_dist {
                        continue;
                    }
                }

                valves[i].distance.insert(j, shortcut_dist);
            }
        }
    }

    let start = *names.get(&"AA").unwrap();

    let solution1 = dfs(&valves, start, 30, &VertexSet::new());
 
    (solution1,0)
}

#[test]
fn test16_1() {
    let solution = solve(&include_str!("../inputs/day16.1"));
    assert_eq!(solution, (1651,0));
}

#[test]
fn test16_2() {
    let solution = solve(&include_str!("../inputs/day16.2"));
    assert_eq!(solution, (0,0));
}
