use core::panic;
use std::{fmt::Display, ops::{Index, IndexMut}, collections::HashSet};
use array2d::Array2D;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up, Down, Left, Right
}

impl Direction {
    const ALL: [Direction; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right
    ];
}

mod parser {
    use nom::{
        IResult,
        character::complete::*,
        combinator::*,
        sequence::*,
        multi::*
    };

    use super::Direction;

    #[derive(Debug, Clone, Copy)]
    pub enum Cell {
        Clear, Wall, Blizzard(Direction)
    }

    pub fn parse(input: &str) -> IResult<&str, Vec<Vec<Cell>>> {
        let cell = map(one_of(".#^v<>"), |c| match c {
            '.' => Cell::Clear,
            '#' => Cell::Wall,
            '^' => Cell::Blizzard(Direction::Up),
            'v' => Cell::Blizzard(Direction::Down),
            '<' => Cell::Blizzard(Direction::Left),
            '>' => Cell::Blizzard(Direction::Right),
            _ => panic!()
        });
        let board = separated_list1(line_ending, many1(cell));
        all_consuming(terminated(board, multispace0))(input)
    }
}

impl Direction {
    fn move_forward(&self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        Some (match self {
            Direction::Up =>  if y > 0 { (x, y-1) } else { return None },
            Direction::Down => (x, y+1),
            Direction::Left => if x > 0 { (x-1, y) } else { return None },
            Direction::Right => (x+1, y)
        })
    }

    fn move_wrap_forward(
            &self,
            (x, y): (usize, usize),
            (wx, wy): (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (x, if y > 0 {y-1} else {wy-1}),
            Direction::Down => (x, if y+1 < wy {y+1} else {0}),
            Direction::Left => (if x > 0 {x-1} else {wx-1}, y),
            Direction::Right => (if x+1 < wx {x+1} else {0}, y)
        }
    }
}

#[derive(Debug, Clone)]
pub enum Cell {
    Clear,
    Wall,
    Blizzard(Vec<Direction>),
}

impl Cell {
    fn is_empty(&self) -> bool {
        matches!(self, Cell::Clear)
    }

    fn is_wall(&self) -> bool {
        matches!(self, Cell::Wall)
    }
}

#[derive(Debug,Clone)]
struct Board (Array2D<Cell>);

impl Board {
    fn from_data(data: &[Vec<parser::Cell>]) -> Self {
        let (num_rows, num_columns) = (data.len(), data[0].len());
        let it = data.iter().flatten().map(|c|
            match c {
                parser::Cell::Clear => Cell::Clear,
                parser::Cell::Wall => Cell::Wall,
                parser::Cell::Blizzard(d) => Cell::Blizzard(vec![*d])
            });
        let array = Array2D::from_iter_row_major(it, num_rows, num_columns);
        Board(array.unwrap())
    }

    fn num_rows(&self) -> usize {
        self.0.num_rows()
    }

    fn num_columns(&self) -> usize {
        self.0.num_columns()
    }

    fn step(&self) -> Self {
        let size = (self.0.num_columns(), self.0.num_rows());
        let mut new_board = Board(Array2D::filled_with(
            Cell::Clear,
            size.1,
            size.0));

        for ((y,x),c) in self.0.enumerate_row_major() {
            match c {
                Cell::Clear => (),
                Cell::Wall => new_board[(x,y)] = Cell::Wall,
                Cell::Blizzard(directions) =>
                    for direction in directions {
                        let mut p = (x,y);
                        loop {
                            p = direction.move_wrap_forward(p, size);
                            if !self[p].is_wall() {
                                break;
                            }
                        }

                        match &new_board[p] {
                            Cell::Wall => panic!(),
                            Cell::Clear =>
                                new_board[p] = Cell::Blizzard(vec![*direction]),
                            Cell::Blizzard(v) => {
                                let mut v = v.clone();
                                v.push(*direction);
                                new_board[p] = Cell::Blizzard(v);
                            }
                        }
                    }
            }
        }
        new_board
    }

    fn write(
            &self,
            f: &mut std::fmt::Formatter<'_>,
            positions: &HashSet<(usize,usize)>) -> std::fmt::Result
    {
        for (y,row_iter) in self.0.rows_iter().enumerate() {
            for (x,cell) in row_iter.enumerate() {
                let c =
                    if positions.contains(&(x,y)) {
                        'E'
                    }
                    else {
                        match cell {
                            Cell::Clear => '.',
                            Cell::Wall => '#',
                            Cell::Blizzard(v) => {
                                if v.len() > 1 {
                                    (v.len() as u8 + b'0') as char
                                }
                                else {
                                    match v.first().unwrap() {
                                        Direction::Up => '^',
                                        Direction::Down => 'v',
                                        Direction::Left => '<',
                                        Direction::Right => '>'
                                    }
                                }
                            }
                        }
                    };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

impl Index<(usize, usize)> for Board {
    type Output = Cell;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        self.0.index((y,x))
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        self.0.index_mut((y,x))
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write(f, &HashSet::new())
    }
}

#[derive(Debug)]
struct State<'a> {
    positions: &'a HashSet<(usize, usize)>,
    board: Board,
}

impl<'a> Display for State<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.board.write(f, self.positions)
    }
}

fn search_path(
        mut board: Board,
        source: (usize,usize),
        dest: (usize,usize)) -> (u32,u32)
{
    let mut queue = HashSet::new();
    let mut next_queue = HashSet::new();
    let mut count = 0;
    let mut solution1 = None;
    queue.insert((source,0));

    loop {
        board = board.step();

        for (p, mut phase) in queue {
            if p == dest {
                if phase == 0 {
                    if solution1.is_none() {
                        solution1 = Some(count);
                    }
                    phase = 1;
                }
                else if phase == 2 {
                    return (solution1.unwrap(), count);
                }
            }
            else if p == source && phase == 1 {
                phase = 2;
            }

            if board[p].is_empty() {
                next_queue.insert((p, phase)); // Do not move
            }

            for d in Direction::ALL {
                if let Some(p) = d.move_forward(p) {
                    if let Some(c) = board.0.get(p.1, p.0) {
                        if c.is_empty() {
                            next_queue.insert((p, phase));
                        }
                    }
                }
            }
        }

        count += 1;
        queue = next_queue;
        next_queue = HashSet::new();
        if queue.is_empty() {
            panic!();
        }
    }
}

pub fn solve(input: &str) -> (u32,u32) {
    let (_,data) = parser::parse(input).unwrap();
    let board = Board::from_data(&data);

    let source = (1,0);
    let destination = (board.num_columns() - 2, board.num_rows() - 1);
    search_path(board, source, destination)
}

#[test]
fn test24_1() {
    let solution = solve(&include_str!("../inputs/day24.1"));
    assert_eq!(solution, (10, 30));
}

#[test]
fn test24_2() {
    let solution = solve(&include_str!("../inputs/day24.2"));
    assert_eq!(solution, (18, 54));
}

#[test]
fn test24_3() {
    let solution = solve(&include_str!("../inputs/day24.3"));
    assert_eq!(solution, (301, 859));
}
