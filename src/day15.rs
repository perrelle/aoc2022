use interval::{IntervalSet, ops::Range};
use gcollections::ops::{constructor::Empty, Union, Cardinality, Difference, Bounded, ProperSubset};

type Point = (i32, i32);

#[derive (Debug)]
pub struct Sensor {
    position: Point,
    neighbor: Point
}

mod parser {
    use nom::{
        IResult,
        character::complete::*,
        combinator::*,
        sequence::*,
        multi::*,
        bytes::complete::*
    };
    use super::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Sensor>> {
        let sensor = map(tuple((
            tag("Sensor at x="), i32,
            tag(", y="), i32,
            tag(": closest beacon is at x="), i32,
            tag(", y="), i32)),
            |(_,x1,_,y1,_,x2,_,y2)|
                Sensor { position: (x1,y1), neighbor: (x2,y2 ) });
        let data = separated_list1(line_ending, sensor);
        all_consuming(terminated(data, multispace0))(input)
    }
}

pub fn dist(p: &Point, q: &Point) -> i32 {
    (p.0 - q.0).abs() + (p.1 - q.1).abs()
}

fn scanned_positions(sensors: &Vec<Sensor>, row: i32) -> IntervalSet<i32> {
    let mut intervals = IntervalSet::empty();

    for sensor in sensors {
        let d = dist(&sensor.position, &sensor.neighbor);
        let h = (sensor.position.1 - row).abs();
        if h <= d  {
            let interval = IntervalSet::new(
                sensor.position.0 - (d - h),
                sensor.position.0 + (d - h));
            intervals = intervals.union(&interval);
        }
    }

    intervals
}

fn _old_part2(sensors: &Vec<Sensor>, range: (i32,i32)) -> Option<i64> {
    let interval_range = IntervalSet::new(range.0,range.1);

    for y in range.0..=range.1 {
        let scanned = scanned_positions(sensors, y);
        if !interval_range.is_proper_subset(&scanned) {
            println!("{y}, {scanned}, {interval_range}");
            let unscanned = interval_range.difference(&scanned);
            let mut iter = unscanned.iter();
            if let Some(interval) = iter.next() {
                assert_eq!(iter.next(), None);
                assert_eq!(interval.lower(), interval.upper());
                let x = interval.lower();
                println!("Found empty spot at ({x},{y})");
                return Some(x as i64 * 4000000 + y as i64);
            }
        }
    }

    None
}

fn part2(sensors: &Vec<Sensor>, range: (i32,i32)) -> Option<i64> {
    for y in range.0..=range.1 {
        let mut x = range.0;
        loop {
            let mut change = false;
            for sensor in sensors {
                let d = dist(&sensor.position, &sensor.neighbor);
                let h = (sensor.position.1 - y).abs();
                let r = d - h;
                if x >= sensor.position.0 - r && x <= sensor.position.0 + r {
                    if x > range.1 {
                        break;
                    }
                    x = sensor.position.0 + r + 1;
                    change = true;
                }
            }

            if x > range.1 {
                break;
            }
            else if !change {
                println!("Found empty spot at ({x},{y})");
                return Some(x as i64 * 4000000 + y as i64);
            }
        }
    }

    None
}

pub fn solve(input: &str, row1: i32, range: (i32,i32)) -> Option<(u32,i64)> {
    let (_,data) = parser::parse(input).unwrap();

    let mut scanned = scanned_positions(&data, row1);

    for sensor in &data {
        if sensor.neighbor.1 == row1 {
            scanned = scanned.difference(&sensor.neighbor.0);
        }
    }

    let solution1 = scanned.size();
    let solution2 = part2(&data, range);
 
    Some((solution1,solution2?))
}

#[test]
fn test15_1() {
    let solution = solve(&include_str!("../inputs/day15.1"), 10, (0,20));
    assert_eq!(solution, Some((26,56000011)));
}

#[test]
fn test15_2() {
    let solution = solve(&include_str!("../inputs/day15.2"), 2000000, (0,4000000));
    assert_eq!(solution, Some((5166077,13071206703981)));
}
