use std::{fmt::Display, ops::{Index, IndexMut}};

use array2d::Array2D;

use crate::rectangle_set;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Open,
    Wall,
    Absent
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Forward(u32),
    TurnLeft,
    TurnRight
}


mod parser {
    use nom::{
        IResult,
        character::complete::*,
        combinator::*,
        sequence::*,
        multi::*,
        branch::*
    };

    use super::*;

    pub fn parse(input: &str) -> IResult<&str, (Vec<Vec<Cell>>,Vec<Instruction>)> {
        let cell = map(one_of(" .#"), |c| match c {
            ' ' => Cell::Absent,
            '.' => Cell::Open,
            '#' => Cell::Wall,
            _ => panic!()
        });
        let board = separated_list1(line_ending, many1(cell));
        let rotation = map(one_of("LR"), |c| match c {
            'L' => Instruction::TurnLeft,
            'R' => Instruction::TurnRight,
            _ => panic!()
        });
        let instruction = alt((rotation, map(u32, Instruction::Forward)));
        let path = many1(instruction);
        let data = separated_pair(board, multispace1, path);
        all_consuming(terminated(data, multispace0))(input)
    }
}


pub struct Board (Array2D<Cell>);

impl Board {
    pub fn from_rows(rows: &[Vec<Cell>]) -> Self {
        let mut col_count = 0;

        for row in rows {
            col_count = col_count.max(row.len());
        }

        let mut board = Array2D::filled_with(Cell::Absent, rows.len(), col_count);

        for (i,row) in rows.iter().enumerate() {
            for (j,&cell) in row.iter().enumerate() {
                board[(i,j)] = cell;
            }
        }

        Board(board)
    }

    pub fn num_rows(&self) -> usize {
        self.0.num_rows()
    }

    pub fn num_columns(&self) -> usize {
        self.0.num_columns()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row_iter in self.0.rows_iter() {
            for cell in row_iter {
                let c = match cell {
                    Cell::Absent => ' ',
                    Cell::Open => '.',
                    Cell::Wall => '#'
                };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

type Point = rectangle_set::Point<usize>;

impl Index<Point> for Board {
    type Output = Cell;

    fn index(&self, index: Point) -> &Self::Output {
        self.0.index((index.y, index.x))
    }
}

impl IndexMut<Point> for Board {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        self.0.index_mut((index.y, index.x))
    }
}

pub type Path = [Instruction];

#[derive(Debug, Clone, Copy)]
enum Direction { Right, Down, Left, Up }

fn turn_left(d: Direction) -> Direction {
    match d {
        Direction::Right => Direction::Up,
        Direction::Down => Direction::Right,
        Direction::Left => Direction::Down,
        Direction::Up => Direction::Left
    }
}

fn turn_right(d: Direction) -> Direction {
    match d {
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Up => Direction::Right
    }
}

fn move_forward(board: &Board, mut p: Point, d: Direction, count: u32) -> Point {
    fn wrap_incr(x: usize, n: usize) -> usize {
        if x + 1 < n { x + 1 } else { 0}
    }

    fn wrap_decr(x: usize, n: usize) -> usize {
        if x > 0 { x - 1 } else { n - 1 }
    }

    for _ in 0..count {
        let previous = p;
        loop {
            match d {
                Direction::Right => p.x = wrap_incr(p.x, board.num_columns()),
                Direction::Down => p.y = wrap_incr(p.y, board.num_rows()),
                Direction::Left => p.x = wrap_decr(p.x, board.num_columns()),
                Direction::Up => p.y = wrap_decr(p.y, board.num_rows()),
            };
            match board[p] {
                Cell::Open => break,
                Cell::Wall => return previous,
                Cell::Absent => continue
            }
        }
    }

    p
}

fn follow_path(board: &Board, path: &Path, mut p: Point, mut d: Direction) -> (Point, Direction) {
    for instruction in path {
        match instruction {
            Instruction::Forward(n) => p = move_forward(board, p, d, *n),
            Instruction::TurnLeft => d = turn_left(d),
            Instruction::TurnRight => d = turn_right(d)
        }
    }
   (p,d)
}

pub fn solve(input: &str) -> (usize,i64) {
    let (_,(board_rows, path)) = parser::parse(input).unwrap();
    let board = Board::from_rows(&board_rows);

    let initial = move_forward(&board, Point {x: 0, y: 0}, Direction::Right, 1);
    let (p,d) = follow_path(&board, &path, initial, Direction::Right);

    let solution1 = 1000 * (p.y + 1) + 4 * (p.x + 1) + match d {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3
    };
    let solution2 = 0;

    (solution1, solution2)
}

#[test]
fn test22_1() {
    let solution = solve(&include_str!("../inputs/day22.1"));
    assert_eq!(solution, (6032, 0));
}

#[test]
fn test22_2() {
    let solution = solve(&include_str!("../inputs/day22.2"));
    assert_eq!(solution, (191010, 0));
}
