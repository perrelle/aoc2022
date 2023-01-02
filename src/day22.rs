use std::{fmt::Display, ops::{Index, IndexMut, Neg}, collections::HashMap, hash::Hash};
use array2d::Array2D;
use crate::rectangle_set;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction { Right, Down, Left, Up }

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


#[derive(Debug, Clone)]
struct Face {
    frame: Frame<i32>,
    mapping: Point
}

#[derive(Debug, Clone)]
pub struct Board {
    grid: Array2D<Cell>,
    path: Array2D<Option<Direction>>,
    side: usize,
    faces: HashMap<Vector3<i32>, Face>
}

impl Board {
    pub fn from_rows(rows: &[Vec<Cell>], side: usize) -> Self {
        let mut col_count = 0;

        for row in rows {
            col_count = col_count.max(row.len());
        }

        let mut grid = Array2D::filled_with(Cell::Absent, rows.len(), col_count);

        for (i,row) in rows.iter().enumerate() {
            for (j,&cell) in row.iter().enumerate() {
                grid[(i,j)] = cell;
            }
        }

        fn enumerate_faces(
            faces: &mut HashMap<Vector3<i32>, Face>,
            grid: &Array2D<Cell>,
            side: usize,
            frame: Frame<i32>,
            x: usize,
            y: usize)
        {
            match grid.get(y,x) {
                None | Some(Cell::Absent) => return,
                Some(Cell::Open | Cell::Wall) => ()
            }

            if faces.contains_key(&frame.elevation()) {
                return;
            }

            faces.insert(frame.elevation().clone(), Face{
                frame: frame.clone(),
                mapping: Point{x,y}});

            enumerate_faces(faces, grid, side, frame.clone().rotate_right(), x + side, y);
            if x > side {
                enumerate_faces(faces, grid, side, frame.clone().rotate_left(), x - side, y);
            }
            enumerate_faces(faces, grid, side, frame.clone().rotate_down(), x, y + side);
            if y > side {
                enumerate_faces(faces, grid, side, frame.clone().rotate_up(), x, y - side);
            }
        }

        let mut faces = HashMap::new ();
        let ((y,x),_) =
            grid.enumerate_row_major().find(|(_,c)|
                **c != Cell::Absent).unwrap();
        enumerate_faces(&mut faces, &grid, side, Frame::standard(), x, y);

        let path = Array2D::filled_with(
            None,
            grid.num_rows(),
            grid.num_columns());

        Board{grid, path, side, faces}
    }



    pub fn num_rows(&self) -> usize {
        self.grid.num_rows()
    }

    pub fn num_columns(&self) -> usize {
        self.grid.num_columns()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y,row_iter) in self.grid.rows_iter().enumerate() {
            for (x,cell) in row_iter.enumerate() {
                let c = match cell {
                    Cell::Absent => ' ',
                    Cell::Wall => '#',
                    Cell::Open => {
                        match self.path[(y,x)] {
                            None => '.',
                            Some(Direction::Right) => '→',
                            Some(Direction::Down) => '↓',
                            Some(Direction::Left) => '←',
                            Some(Direction::Up) => '↑'
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

type Point = rectangle_set::Point<usize>;

impl Index<Point> for Board {
    type Output = Cell;

    fn index(&self, index: Point) -> &Self::Output {
        self.grid.index((index.y, index.x))
    }
}

impl IndexMut<Point> for Board {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        self.grid.index_mut((index.y, index.x))
    }
}

pub type Path = [Instruction];

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector3<T>(T,T,T);

impl<T: Neg<Output = T>> Neg for Vector3<T> {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        self.0 = -self.0;
        self.1 = -self.1;
        self.2 = -self.2;
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Frame<T>(Vector3<T>, Vector3<T>, Vector3<T>);

impl Frame<i32> {
    pub fn standard() -> Self {
        Frame(Vector3(1,0,0),Vector3(0,1,0),Vector3(0,0,1))
    }
}

impl<T: Clone> Frame<T> {
    pub fn easting(&self) -> Vector3<T> {
        self.0.clone()
    }

    pub fn northing(&self) -> Vector3<T> {
        self.1.clone()
    }

    pub fn elevation(&self) -> Vector3<T> {
        self.2.clone()
    }
}

impl<T: Neg<Output = T> + Copy>  Frame<T> {
    pub fn rotate_right(&self) -> Frame<T> {
        Frame(self.2,self.1,-self.0)
    }

    pub fn rotate_left(&self) -> Frame<T> {
        Frame(-self.2,self.1,self.0)
    }

    pub fn rotate_up(&self) -> Frame<T> {
        Frame(self.0,self.2,-self.1)
    }

    pub fn rotate_down(&self) -> Frame<T> {
        Frame(self.0,-self.2,self.1)
    }
}

struct Position {
    point: Point,
    frame: Option<Frame<i32>>,
}

impl Board {  
    fn step_forward(&mut self, p: &Position, d: Direction) -> Option<Position> {
        fn wrap_incr(x: usize, n: usize) -> usize {
            if x + 1 < n { x + 1 } else { 0 }
        }

        fn wrap_decr(x: usize, n: usize) -> usize {
            if x > 0 { x - 1 } else { n - 1 }
        }
    
        let Position{point: Point{mut x, mut y}, mut frame} = p;
        self.path[(y,x)] = Some(d);

        match frame {
            None =>
                loop {
                    match d {
                        Direction::Right => x = wrap_incr(x, self.num_columns()),
                        Direction::Down => y = wrap_incr(y, self.num_rows()),
                        Direction::Left => x = wrap_decr(x, self.num_columns()),
                        Direction::Up => y = wrap_decr(y, self.num_rows()),
                    };
                    if self[p.point] != Cell::Absent {
                        break;
                    }
                }
            Some(mut f) => {
                match d {
                    Direction::Right =>
                        if x % self.side == self.side - 1 {
                            x = 0;
                            f = f.rotate_right();
                        },
                    Direction::Down =>
                        if y % self.side == self.side - 1 {
                            y = 0;
                            f = f.rotate_down();
                        },               
                    Direction::Left =>
                        if x % self.side == 0 {
                            x = self.side - 1;
                            f = f.rotate_left();
                        },
                    Direction::Up =>
                        if y % self.side == 0 {
                            y = self.side -1;
                            f = f.rotate_up();
                        }
                }
                frame = Some(f)
            }
        }

        if self[Point{x,y}] == Cell::Wall {
            None
        }
        else {
            Some(Position{point: Point{x, y}, frame})
        }
    }

    fn follow_path(&mut self, path: &Path, mut p: Position, mut d: Direction) -> (Point, Direction) {
        for instruction in path {
            match instruction {
                Instruction::TurnLeft => d = turn_left(d),
                Instruction::TurnRight => d = turn_right(d),
                Instruction::Forward(n) =>
                    for _ in 0..*n {
                        if let Some(q) = self.step_forward(&p, d) {
                            p = q;
                        }
                    }
            }
        }
        (p.point, d)
    }
}

pub fn solve(input: &str, side: usize) -> (usize,usize) {
    let (_,(board_rows, path)) = parser::parse(input).unwrap();
    let mut board = Board::from_rows(&board_rows, side);

    let position = Position{point: Point {x: 0, y: 0}, frame: None};
    let initial1 = board.step_forward(&position, Direction::Right).unwrap();
    let initial2 = Position{point: initial1.point.clone(), frame: Some(Frame::standard())};
    let solution1 = board.clone().follow_path(&path, initial1, Direction::Right);
    let solution2 = board.follow_path(&path, initial2, Direction::Right);

    println!("{board}");

    fn convert_solution((p,d): (Point, Direction)) -> usize {
        1000 * (p.y + 1) + 4 * (p.x + 1) + match d {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3
        }
    }

    (convert_solution(solution1), convert_solution(solution2))
}

#[test]
fn test22_1() {
    let grid = include_str!("../inputs/day22.1");
    /*
    let horizontal_transitions = [
        [None, None, Some((Rotation::UTurn, 0, 1)), None],
        [Some((Rotation::UTurn, 2, 0)), Some((Rotation::ClockWise, 2, 0)), None, None],
        [Some((Rotation::UTurn, 2, 2)), Some((Rotation::Counterclockwise, 2, 0)), None, Some((Rotation::Counterclockwise, 2,1))],
        [None, None, Some((Rotation::UTurn, 0,1)), Some((Rotation::Counterclockwise, 0, 1))],
        [None, None, None, None]];
    let vertical_transitions = [
        [None, None, Some((Rotation::Counterclockwise, 1, 1)), Some((Rotation::UTurn, 3, 2)), None],
        [Some((Rotation::ClockWise, 3, 2)), None, None, Some((Rotation::ClockWise, 3, 2)), None],
        [None, None, Some((Rotation::ClockWise, 1,1)), None, Some((Rotation::UTurn, 2, 0))],
        [None, None, None, None, None]
    ];
     */
    let solution = solve(&grid, 4);
    assert_eq!(solution, (6032, 0));
}

#[test]
fn test22_2() {
    let solution = solve(&include_str!("../inputs/day22.2"), 50);
    assert_eq!(solution, (191010, 0));
}
