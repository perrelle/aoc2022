use array2d::Array2D;
use std::collections::VecDeque;

mod parser {
    use nom::{
        IResult,
        character::complete::*,
        combinator::*,
        sequence::*,
        multi::*
    };

    pub fn parse(input: &str) -> IResult<&str, Vec<Vec<char>>> {
        let cell = satisfy(|c| ('a'..='z').contains(&c) || c == 'S' || c == 'E');
        let grid = separated_list1(line_ending, many1(cell));
        all_consuming(terminated(grid, multispace0))(input)
    }
}

type Point = (usize,usize);

const DIRECTIONS: [(i32,i32); 4] = [
    (1,0), (0,1), (-1, 0), (0, -1)
];

fn move_point(p: &Point, d: &(i32,i32)) -> Option<Point> {
    Some((
        usize::try_from(p.0 as i32 + d.0).ok()?,
        usize::try_from(p.1 as i32 + d.1).ok()?))
}

fn shortest_path(heightmap: &Array2D<u8>, starts: &[Point], end: &Point)
        -> Option<u32> {
    let mut queue: VecDeque<(u32,(usize,usize))>  = VecDeque::new();

    for p in starts {
        queue.push_back((0,*p));
    }
    
    let mut visits =
        Array2D::filled_with(
            false, 
            heightmap.num_rows(),
            heightmap.num_columns());

    while let Some ((distance,p)) = queue.pop_front() {
        if p == *end {
            return Some(distance);
        }

        let ph = *heightmap.get(p.0, p.1).unwrap();

        for d in &DIRECTIONS {
            if let Some(q) = move_point(&p, d) {
                if let Some(false) = visits.get(q.0, q.1) {
                    let qh = *heightmap.get(q.0, q.1).unwrap();
                    if qh <= ph + 1 {
                        visits.set(q.0, q.1, true).unwrap();
                        queue.push_back((distance+1,q));
                    }
                }
            }
        }
    }

    None
}

pub fn solve(input: &str) -> Option<(u32,u32)> {
    let (_,data) = parser::parse(input).unwrap();
    let data_array = Array2D::from_rows(&data).unwrap();
    let heightmap =
        Array2D::from_iter_row_major(
            data_array.elements_row_major_iter().map(|&c| match c {
                'a'..='z' => c as u8 - b'a',
                'S' => 0,
                'E' => 25,
                _ => panic!()
            }),
            data_array.num_rows(),
            data_array.num_columns()).unwrap();

    let (start,_) =
        data_array.enumerate_row_major().find(|(_,c)| **c == 'S')?;
    let (end,_) =
        data_array.enumerate_row_major().find(|(_,c)| **c == 'E')?;
    let lowest_points: Vec<Point> =
        heightmap.enumerate_row_major()
            .filter(|(_,c)| **c == 0)
            .map(|(p,_)| p)
            .collect();

    let solution1 = shortest_path(&heightmap, &[start], &end)?;
    let solution2 = shortest_path(&heightmap, &lowest_points, &end)?;


    Some((solution1, solution2))
}

#[test]
fn test12_1() {
    let solution = solve(&include_str!("../inputs/day12.1"));
    assert_eq!(solution, Some((31,29)));
}

#[test]
fn test12_2() {
    let solution = solve(&include_str!("../inputs/day12.2"));
    assert_eq!(solution, Some((420,414)));
}
