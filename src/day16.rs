use std::{collections::{HashMap, BTreeSet, BinaryHeap}, cmp::Ordering};

#[derive (Debug)]
pub struct Valve {
    _name: String,
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

fn _dfs(valves: &[Valve], pos: usize, time: u32, closed: &VertexSet) -> u32 {
    let mut closed = closed.clone();
    closed.remove(&pos);

    let mut best = 0;

    for &n in &closed {
        let d = *valves[pos].distance.get(&n).unwrap();
    
        if time > d {
            let r = time - d - 1;
            best = best.max(
                (r * valves[n].flow_rate) +
                _dfs(valves, n, r, &closed));
        }
    }

    best
}

fn dfs2(valves: &[Valve], pos1: usize, time1: u32, pos2: usize, time2: u32, closed: &VertexSet) -> u32 {
    let mut best = 0;
   
    for &n in closed {
        let d1 = *valves[pos1].distance.get(&n).unwrap();
        let d2 = *valves[pos2].distance.get(&n).unwrap();
        if time1 as i32 - d1 as i32 > time2 as i32 - d2 as i32 {
            if time1 > d1 {
                let pos1 = n;
                let time1 = time1 - d1 - 1;
                let mut closed = closed.clone();
                closed.remove(&n);
                best = best.max(
                    (time1 * valves[n].flow_rate) +
                    dfs2(valves, pos1, time1, pos2, time2, &closed));
            }
        }
        else {
            if time2 > d2 {
                let pos2 = n;
                let time2 = time2 - d2 - 1;
                let mut closed = closed.clone();
                closed.remove(&n);
                best = best.max(
                    (time2 * valves[n].flow_rate) +
                    dfs2(valves, pos1, time1, pos2, time2, &closed));
            }
        }
    }

    best
}


#[derive(Eq,PartialEq,Debug)]
struct State {
    position: usize,
    time: u32,
    score: u32,
    estimate: u32,
    closed: VertexSet,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn estimate(valves: &[Valve], closed: &VertexSet, time: u32) -> u32 {
    closed.iter().fold(0, |acc, v| acc + valves.get(*v).unwrap().flow_rate * time)
}

fn bfs(valves: &[Valve], start: usize, time: u32) -> u32 {
    let mut heap = BinaryHeap::new();
    let all_valves = VertexSet::from_iter(0..valves.len());
    let initial_state = State {
        position: start,
        time: time,
        score: 0,
        estimate: estimate(valves, &all_valves, time),
        closed: all_valves
    };
    heap.push(initial_state);

    while let Some(state) = heap.pop() {
        println!("{state:?}");
        let i = state.position;
        for &j in &state.closed {
            let d = *valves[i].distance.get(&j).unwrap();
            if state.time > d && state.closed.contains(&j) {
                let time = state.time - d - 1;
                let score = state.score + time * valves[j].flow_rate;
                let mut closed = state.closed.clone();
                closed.remove(&j);

                if closed.is_empty() {
                    return score;
                }

                let estimate = score + estimate(valves, &closed, time);
                let state = State { position: j, time, score, closed, estimate };
                heap.push(state);
            }
        }
    }
    
    panic!()
}

pub fn solve(input: &str) -> (u32,u32) {
    let (_,data) = parser::parse(input).unwrap();

    let mut valves = Vec::new();
    let mut names = HashMap::new();

    for (name, flow_rate, _) in &data {
        names.insert(name, valves.len());
        valves.push(Valve {
            _name: String::from(*name),
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
 
    let all_valves = VertexSet::from_iter((0..valves.len()).filter(|v| valves[*v].flow_rate > 0));
    let solution1 = _dfs(&valves, start, 30, &all_valves);
    let solution2 = dfs2(&valves, start, 26, start, 26, &all_valves);
 
    (solution1,solution2)
}

#[test]
fn test16_1() {
    let solution = solve(&include_str!("../inputs/day16.1"));
    assert_eq!(solution, (1651,1707));
}

#[test]
fn test16_2() {
    let solution = solve(&include_str!("../inputs/day16.2"));
    assert_eq!(solution, (1828,2292));
}
