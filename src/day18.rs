use std::{fmt, fmt::Debug};

#[derive (Debug)]
pub struct Point (i32,i32,i32);

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

    pub fn parse(input: &str) -> IResult<&str, Vec<Point>> {
        let point = map(
            tuple((i32, tag(","), i32, tag(","), i32)),
            |(x,_,y,_,z)| Point(x,y,z));
        let data = separated_list1(multispace1, point);
        all_consuming(terminated(data, multispace0))(input)
    }
}

struct Grid<T, const SIZE: usize>
    ([[[T; SIZE]; SIZE]; SIZE]);

impl<T: std::marker::Copy, const SIZE: usize> Grid<T, SIZE> {
    fn create(x: T) -> Self {
        Grid ([[[x; SIZE]; SIZE]; SIZE])
    }

    fn get(&self, x: i32, y: i32, z: i32, default: T) -> T {
        if x >= 0 && y >= 0 && z >= 0 {
            let (x,y,z) = (x as usize, y as usize, z as usize);
            if x < SIZE && y < SIZE && z < SIZE {
                return self.0[z][y][x]
            }
        }
        default
    }

    fn set(&mut self, x: i32, y: i32, z: i32, value: T) {
        let (x,y,z) = (x as usize, y as usize, z as usize);
        assert!(x < SIZE && y < SIZE && z < SIZE);
        self.0[z][y][x] = value;
    }
}

impl<const SIZE: usize> fmt::Display for Grid<bool, SIZE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for z in 0..SIZE {
            for y in 0..SIZE {
                for x in 0..SIZE {
                    write!(f, "{}", if self.0[z][y][x] {'#'} else {'.'})?
                }
                writeln!(f)?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

const SIZE: usize = 22;

fn solve_part1(grid: &Grid<bool, SIZE>) -> u32 {
    let mut count = 0;

    for z in 0..=SIZE as i32 {
        for y in 0..=SIZE as i32 {
            for x in 0..=SIZE as i32 {
                let c = grid.get(x, y, z, false);
                for (dx,dy,dz) in [(1,0,0),(0,1,0),(0,0,1)] {
                    if c != grid.get(x-dx, y-dy, z-dz, false) {
                        count += 1
                    }
                }
            }
        }
    }

    count
}

fn solve_part2(grid: &Grid<bool, SIZE>) -> u32 {
    const MARKS_SIZE: usize = SIZE + 2;
    let mut marks: Grid<bool, MARKS_SIZE> = Grid::create(false);
    let mut stack: Vec<(i32,i32,i32)> = Vec::new();
    stack.push((0,0,0));
    let mut count = 0;

    while let Some((x,y,z)) = stack.pop() {
        if marks.get(x+1, y+1, z+1, false) {
            continue;
        }

        marks.set(x+1, y+1, z+1, true);
        
        for (dx,dy,dz) in [(1,0,0),(0,1,0),(0,0,1),(-1,0,0),(0,-1,0),(0,0,-1)] {
            let (nx,ny,nz) = (x+dx, y+dy, z+dz);
            if grid.get(nx, ny, nz, false) {
                count += 1;
            }
            else if nx >= -1 && ny >= -1 && nz >= -1 &&
                    nx < SIZE as i32 + 1 &&
                    ny < SIZE as i32 + 1 &&
                    nz < SIZE as i32 + 1
            {
                stack.push((nx,ny,nz));
            }
        }
    }

    count
}

pub fn solve(input: &str) -> (u32,u32) {
    let (_,data) = parser::parse(input).unwrap();

    let mut grid: Grid<bool, SIZE> = Grid::create(false);

    for Point(x,y,z) in &data {
        grid.set(*x, *y, *z, true);
    }

    let solution1 = solve_part1(&grid);
    let solution2 = solve_part2(&grid);

    (solution1, solution2)
}

#[test]
fn test18_1() {
    let solution = solve(&include_str!("../inputs/day18.1"));
    assert_eq!(solution, (64,58));
}

#[test]
fn test18_2() {
    let solution = solve(&include_str!("../inputs/day18.2"));
    assert_eq!(solution, (4244,2460));
}
