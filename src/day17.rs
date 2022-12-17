use std::fmt;

#[derive (Debug, Clone, Copy)]
pub enum Direction { Left, Right }

mod parser {
    use nom::{
        IResult,
        character::complete::*,
        combinator::*,
        sequence::*,
        multi::*
    };

    use super::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Direction>> {
        let direction = map(one_of("<>"), |c| match c { 
            '>' => Direction::Right,
            '<' => Direction::Left,
            _ => panic!()
        });
        let data = many1(direction);
        all_consuming(terminated(data, multispace0))(input)
    }
}

type Shape<'a> = [&'a[bool]];

const SHAPES: [&Shape; 5] = [
    &[
        &[true, true, true, true]],
    &[
        &[false, true, false],
        &[true, true, true],
        &[false, true, false]],
    &[
        &[false, false, true],
        &[false, false, true],
        &[true, true, true]],
    &[
        &[true],
        &[true],
        &[true],
        &[true]],
    &[
        &[true, true],
        &[true, true]]];

struct Grid (Vec<[bool; 7]>);

impl Grid {
    fn empty() -> Self {
        Grid(Vec::new())
    }

    fn new_line(&mut self) {
        self.0.push([false; 7]);
    }

    fn get(&self, x: usize, y: usize) -> bool {
        assert!(x < 7);
        if y >= self.0.len() {
            return false;
        }
        self.0[y][x]
    }

    fn set(&mut self, x: usize, y: usize) {
        assert!(x < 7);
        while y >= self.0.len() {
            self.new_line();
        }
        self.0[y][x] = true;
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn shape_fits(&self, shape: &Shape, x: usize, y: usize) -> bool {
        for (ys, row) in shape.iter().enumerate() {
            for (xs, cell) in row.iter().enumerate() {
                if *cell && self.get(x + xs, y - ys) {
                    return false;
                }
            }
        }
        true
    }

    fn place_shape(&mut self, shape: &Shape, x: usize, y: usize) {
        for (ys, row) in shape.iter().enumerate() {
            for (xs, cell) in row.iter().enumerate() {
                if *cell {
                    self.set(x + xs, y - ys);
                }
            }
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.0.iter().rev() {
            for &cell in row {
                write!(f, "{}", if cell {'#'} else {'.'})?
            }
            writeln!(f)?
        }
        Ok(())
    }
}


pub fn solve(input: &str) -> (usize,u32) {
    let (_,data) = parser::parse(input).unwrap();

    let mut grid = Grid::empty();
    let mut shape_iterator = SHAPES.iter().cycle();
    let mut direction_iterator = data.iter().cycle();

    for _count in 1..=2022 {
        let shape = shape_iterator.next().unwrap();
        let (w, h) = (shape[0].len(), shape.len());
        let mut x = 2;
        let mut y = grid.height() + shape.len() + 2;

        loop {
            match *direction_iterator.next().unwrap() {
                Direction::Left => {
                    if x > 0 && grid.shape_fits(&shape, x-1, y) {
                        x -= 1;
                    }
                },
                Direction::Right => {
                    if x + w < 7 && grid.shape_fits(&shape, x+1, y) {
                        x += 1;
                    }
                }
            }

            if y >= h && grid.shape_fits(&shape, x, y-1) {
                y -= 1;
            }
            else {
                grid.place_shape(&shape, x, y);
                break;
            }
        }
    }

    let solution1 = grid.height();

    (solution1,0)
}

#[test]
fn test17_1() {
    let solution = solve(&include_str!("../inputs/day17.1"));
    assert_eq!(solution, (3068,0));
}

#[test]
fn test16_2() {
    let solution = solve(&include_str!("../inputs/day17.2"));
    assert_eq!(solution, (0,0));
}
