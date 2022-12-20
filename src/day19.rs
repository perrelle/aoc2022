use std::{ops::{Add, SubAssign}, cmp::Ordering, hash::Hash};

#[derive (Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resource {
    Ore, Clay, Obsidian, Geode    
}

impl Resource {
    const ALL: [Resource; 4] = [
        Resource::Ore,
        Resource::Clay,
        Resource::Obsidian,
        Resource::Geode
    ];
}

#[derive (Debug)]
pub struct Cost {
    ore: u32,
    clay: u32,
    obsidian: u32
}

impl Cost {
    const ZERO: Cost = Cost {
        ore: 0, clay: 0, obsidian: 0
    };

    fn max(&self, other: &Cost) -> Cost {
        Cost {
            ore: self.ore.max(other.ore),
            clay: self.clay.max(other.clay),
            obsidian: self.obsidian.max(other.obsidian),
        }
    }

    fn get(&self, r: Resource) -> u32 {
        match r {
            Resource::Ore => self.ore,
            Resource::Clay => self.clay,
            Resource::Obsidian => self.obsidian,
            Resource::Geode => panic!()
        }
    }
}

impl Add for Cost {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Cost {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian
        }
    }
}


#[derive (Debug)]
pub struct Blueprint {
    id: u32,
    ore_robot: Cost,
    clay_robot: Cost,
    obsidian_robot: Cost,
    geode_robot: Cost,
    max_costs: Cost
}

impl Blueprint {
    fn get_robot_cost(&self, r: Resource) -> &Cost {
        match r {
            Resource::Ore => &self.ore_robot,
            Resource::Clay => &self.clay_robot,
            Resource::Obsidian => &self.obsidian_robot,
            Resource::Geode => &self.geode_robot
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
        bytes::complete::*,
        branch::*
    };

    use super::*;

    fn one_cost(input: &str) -> IResult<&str, Cost> {
        alt((
            map(terminated(u32, tag(" ore")),
                |x| Cost { ore: x, clay: 0, obsidian: 0 }),
            map(terminated(u32, tag(" clay")),
                |x| Cost { ore: 0, clay: x, obsidian: 0 }),
            map(terminated(u32, tag(" obsidian")),
                |x| Cost { ore: 0, clay: 0, obsidian: x }),
        ))(input)
    }

    fn cost(input: &str) -> IResult<&str, Cost> {
        let (mut input,mut c) = one_cost(input)?;
        while let Ok((input_rest,_)) = tag::<&str, &str, nom::error::Error<&str>>(" and ")(input) {
            let (input_rest,c2) = one_cost(input_rest)?;
            input = input_rest;
            c = c + c2;
        }
        Ok((input,c))
    }

    fn blueprint(input: &str) -> IResult<&str, Blueprint> {
        let (input, id) = delimited(
            tag("Blueprint "), u32, char(':'))(input)?;
        let (input,_) = multispace0(input)?;
        let (input, ore_robot) = delimited(
            tag("Each ore robot costs "), cost, char('.'))(input)?;
        let (input,_) = multispace0(input)?;
        let (input, clay_robot) = delimited(
            tag("Each clay robot costs "), cost, char('.'))(input)?;
        let (input,_) = multispace0(input)?;
        let (input, obsidian_robot) = delimited(
            tag("Each obsidian robot costs "), cost, char('.'))(input)?;
        let (input,_) = multispace0(input)?;
        let (input, geode_robot) = delimited(
            tag("Each geode robot costs "), cost, char('.'))(input)?;
        let max_costs =
            [&ore_robot, &clay_robot, &obsidian_robot, &geode_robot]
                .iter().fold(Cost::ZERO, |acc, robot| acc.max(robot));
        Ok((input, Blueprint {
            id,
            ore_robot,
            clay_robot,
            obsidian_robot,
            geode_robot,
            max_costs
        }))
    }

    pub fn parse(input: &str) -> IResult<&str, Vec<Blueprint>> {
        let data = separated_list1(multispace1, blueprint);
        all_consuming(terminated(data, multispace0))(input)
    }
}

#[derive (Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32
}

impl SubAssign<&Cost> for State {
    fn sub_assign(&mut self, cost: &Cost) {
        self.ore -= cost.ore;
        self.clay -= cost.clay;
        self.obsidian -= cost.obsidian;
    }
}

impl State {
    fn new() -> Self {
        State {
            ore_robots: 0,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0
        }
    }

    fn step(&mut self, count: u32) {
        self.ore += self.ore_robots * count;
        self.clay += self.clay_robots * count;
        self.obsidian += self.obsidian_robots * count;
        self.geode += self.geode_robots * count;
    }

    fn steps_required_to_build(&self, blueprint: &Blueprint, r: Resource) -> Option<u32> {
        fn steps(current: u32, target: u32, production: u32) -> Option<u32> {
            if current >= target {
                Some(0)
            }
            else if production == 0 {
                None
            }
            else {
                Some((target - current - 1) / production + 1) // rounded up
            }
        }

        let cost = blueprint.get_robot_cost(r);

        let n_ore = steps(self.ore, cost.ore, self.ore_robots)?;
        let n_clay = steps(self.clay, cost.clay, self.clay_robots)?;
        let n_obsidian = steps(self.obsidian, cost.obsidian, self.obsidian_robots)?;

        Some(n_ore.max(n_clay.max(n_obsidian)))
    }

    fn get_robot_count(&self, r: Resource) -> u32 {
        match r {
            Resource::Ore => self.ore_robots,
            Resource::Clay => self.clay_robots,
            Resource::Obsidian => self.obsidian_robots,
            Resource::Geode => self.geode_robots
        }
    }

    fn add_robot(&mut self, r: Resource) {
        match r {
            Resource::Ore => self.ore_robots += 1,
            Resource::Clay => self.clay_robots += 1,
            Resource::Obsidian => self.obsidian_robots += 1,
            Resource::Geode => self.geode_robots += 1
        }
    }

    fn build_robot(&mut self, blueprint: &Blueprint, r: Resource) {
        let cost = blueprint.get_robot_cost(r);
        *self -= cost;
        self.add_robot(r);
    }

    fn get_end_geodes(&self, remaining_time: u32) -> u32 {
        self.geode + remaining_time * self.geode_robots
    }
}


#[derive (Eq, PartialEq)]
struct Item(u32, State);

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }

}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn bfs(blueprint: &Blueprint, initial_state: State, time: u32) -> u32 {
    let mut score = 0;
    let mut queue = Vec::new();
    queue.push(Item(time, initial_state));

    while let Some(Item(time, state)) = queue.pop() {
        let state_score = state.get_end_geodes(time);
        if state_score > score {
            score = state_score;
        }
        else if time > 0 && state_score + (time * (time - 1) /  2) <= score { // Cut branch as suboptimal
            continue;
        }

        for r in Resource::ALL {
            if r != Resource::Geode &&
                state.get_robot_count(r) >= blueprint.max_costs.get(r)
            {
                continue; // Do not build more robot of this kind, it's useless
            }

            if let Some(n) = state.steps_required_to_build(blueprint, r) {
                if n < time {
                    let mut state = state.clone();
                    state.step(n + 1);
                    state.build_robot(blueprint, r);
                    queue.push(Item(time - n - 1, state));
                }
            }
        }
    }

    score
}

pub fn solve(input: &str) -> (u32,u32) {
    let (_,data) = parser::parse(input).unwrap();

    let mut initial = State::new();
    initial.add_robot(Resource::Ore);

    let mut solution1 = 0;

    for blueprint in &data {
        let score = bfs(blueprint, initial.clone(), 24);
        println!("Blueprint {}: scored {score}", blueprint.id);
        solution1 += blueprint.id * score;
    }

    let mut solution2 = 1;
    let sub_data = if data.len() > 3 { &data[0..3] } else { &data };

    for blueprint in sub_data {
        let score = bfs(blueprint, initial.clone(), 32);
        println!("Blueprint {}: scored {score}", blueprint.id);
        solution2 *= score;
    }

    (solution1, solution2)
}

#[test]
fn test19() {
    let input = &include_str!("../inputs/day19.1");
    let (_,data) = parser::parse(input).unwrap();
    let blueprint = &data[0];

    let mut state = State::new();
    state.add_robot(Resource::Ore);
    let mut time = 24;

    let choices = [
        Resource::Clay,
        Resource::Clay,
        Resource::Clay,
        Resource::Obsidian,
        Resource::Clay,
        Resource::Obsidian,
        Resource::Geode,
        Resource::Geode
    ];

    for r in choices {
        println!("{time}: {state:?}");
        let n = state.steps_required_to_build(blueprint, r).unwrap() + 1;
        state.step(n);
        state.build_robot(blueprint, r);
        time -= n;
    }

    println!("{time}: {state:?}");
    state.step(time);
    time = 0;
    println!("{time}: {state:?}");
}

#[test]
fn test19_1() {
    let solution = solve(&include_str!("../inputs/day19.1"));
    assert_eq!(solution, (33,3472));
}

#[test]
fn test19_2() {
    let solution = solve(&include_str!("../inputs/day19.2"));
    assert_eq!(solution, (1115,25056));
}
