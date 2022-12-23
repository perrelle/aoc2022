use std::{fmt::Display, ops::{Index, IndexMut}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction { North, South, West, East }

impl Direction {
    fn move_forward(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Direction::North => (x,y-1),
            Direction::South => (x,y+1),
            Direction::West => (x-1,y),
            Direction::East => (x+1,y)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice {
    Stay,
    Move(Direction)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Dwarf(Choice),
    Targeted(u32)
}

impl Cell {
    fn is_empty(&self) -> bool {
        match self {
            Cell::Empty | Cell::Targeted(_) => true,
            Cell::Dwarf(_) => false
        }
    }
}

mod parser {
    use nom::{
        IResult,
        character::complete::*,
        combinator::*,
        sequence::*,
        multi::*
    };

    pub fn parse(input: &str) -> IResult<&str, Vec<Vec<bool>>> {
        let cell = map(one_of(".#"), |c| match c {
            '.' => false,
            '#' => true,
            _ => panic!()
        });
        let board = separated_list1(line_ending, many1(cell));
        all_consuming(terminated(board, multispace0))(input)
    }
}

const SIZE: usize = 200;

#[derive(Debug)]
pub struct Board {
    grid: [[Cell; SIZE]; SIZE],
    area: (usize, usize, usize, usize),
}

impl Board {
    pub fn from_rows(rows: &[Vec<bool>]) -> Self {
        let grid = [[Cell::Empty ; SIZE] ; SIZE];
        let area = (SIZE/2, SIZE/2, SIZE/2, SIZE/2);
        let offset = (SIZE - rows.len()) / 2;
        let mut board = Board{grid, area};

        for (y, row) in rows.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                board[(x + offset,y + offset)] =
                    if cell {
                        Cell::Dwarf(Choice::Stay)
                    }
                    else {
                        Cell::Empty
                    };
            }
        }

        board
    }

    fn increase_area(&mut self, x: usize, y: usize) {
        self.area.0 = self.area.0.min(x);
        self.area.1 = self.area.1.min(y);
        self.area.2 = self.area.2.max(x);
        self.area.3 = self.area.3.max(y);
    }

    fn reduce_area(&mut self) {
        let mut area = (usize::MAX, usize::MAX, 0, 0);
        for (x,y) in self.iter_positions() {
            if !self[(x,y)].is_empty() {
                area.0 = area.0.min(x);
                area.1 = area.1.min(y);
                area.2 = area.2.max(x);
                area.3 = area.3.max(y);
            }
        }
        self.area = area;
    }

    fn iter_positions(&self) -> impl Iterator<Item = (usize, usize)> {
        let area = self.area.clone();
        (area.1..=area.3).flat_map(move |y| {
            (area.0..=area.2).map(move |x| {
                (x,y)
            })
        })
    }

    fn is_empty(&self, x: usize, y: usize) -> bool {
        self[(x,y)].is_empty()
    }

    fn is_isolated(&self, x: usize, y: usize) -> bool {
        self.is_empty(x-1, y-1) &&
        self.is_empty(x, y-1) &&
        self.is_empty(x+1, y-1) &&
        self.is_empty(x-1, y) &&
        self.is_empty(x+1, y) &&
        self.is_empty(x-1, y+1) &&
        self.is_empty(x, y+1) &&
        self.is_empty(x+1, y+1)
    }

    fn can_move(&self, x: usize, y: usize, d: Direction) -> bool {
        match d {
            Direction::North =>
                self.is_empty(x-1, y-1) &&
                self.is_empty(x, y-1) &&
                self.is_empty(x+1, y-1),
            Direction::South =>
                self.is_empty(x-1, y+1) &&
                self.is_empty(x, y+1) &&
                self.is_empty(x+1, y+1),
            Direction::West => 
                self.is_empty(x-1, y-1) &&
                self.is_empty(x-1, y) &&
                self.is_empty(x-1, y+1),
            Direction::East =>
                self.is_empty(x+1, y-1) &&
                self.is_empty(x+1, y) &&
                self.is_empty(x+1, y+1),
        }
    }

    pub fn step(&mut self, directions: &[Direction]) -> bool {
        // Phase 1
        for (x,y) in self.iter_positions() {
            if let Cell::Dwarf(_) = self[(x,y)] {
                let mut c = Choice::Stay;
                if !self.is_isolated(x, y) {
                    for d in directions {
                        if self.can_move(x, y, *d) {
                            c = Choice::Move(*d);
                            let destination = d.move_forward(x, y);
                            self[destination] = match self[destination] {
                                Cell::Empty => Cell::Targeted(1),
                                Cell::Targeted(i) => Cell::Targeted(i+1),
                                Cell::Dwarf(_) => panic!()
                            };
                            break;
                        }
                    }
                }
                self[(x,y)] = Cell::Dwarf(c);
            }
        }

        // Phase 2
        let mut did_elves_move = false;
        for (x,y) in self.iter_positions() {
            if let Cell::Dwarf(Choice::Move(d)) = self[(x,y)] {
                let destination = d.move_forward(x, y);
                match self[destination] {
                    Cell::Targeted(i) =>
                        if i <= 1 {
                            self[(x,y)] = Cell::Empty;
                            self[destination] = Cell::Dwarf(Choice::Stay);
                            did_elves_move = true;
                        }
                    _ => panic!("{:?}", self[destination])
                }
            }
        }

        // Cleaning
        for (x,y) in self.iter_positions() {
            if let Cell::Targeted(_) = self[(x,y)] {
                self[(x,y)] = Cell::Empty;
            }
        }

        did_elves_move
    }

}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.area.1..=self.area.3 {
            for x in self.area.0..=self.area.2 {
                let c = match self.grid[y][x] {
                    Cell::Empty => '.',
                    Cell::Dwarf(Choice::Stay) => '#',
                    Cell::Dwarf(Choice::Move(Direction::North)) => '↑',
                    Cell::Dwarf(Choice::Move(Direction::South)) => '↓',
                    Cell::Dwarf(Choice::Move(Direction::West)) => '→',
                    Cell::Dwarf(Choice::Move(Direction::East)) => '←',
                    Cell::Targeted(_) => 'o'
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
        &self.grid[y][x]
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        self.increase_area(x, y);
        &mut self.grid[y][x]
    }
}

pub fn solve(input: &str) -> (usize,i64) {
    let (_,board_rows) = parser::parse(input).unwrap();
    let mut board = Board::from_rows(&board_rows);

    let mut directions = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East
    ];

    let mut solution1 = 0;
    let mut cycle = 1;
    while board.step(&directions) {
        directions.rotate_left(1);
        if cycle == 10 {
            board.reduce_area();
            solution1 =
                board.iter_positions().filter(|p| board[*p].is_empty()).count();
        }
        cycle += 1;
    }

    let solution2 = cycle;

    (solution1, solution2)
}

#[test]
fn test23_1() {
    let solution = solve(&include_str!("../inputs/day23.1"));
    assert_eq!(solution, (110, 20));
}

#[test]
fn test23_2() {
    let solution = solve(&include_str!("../inputs/day23.2"));
    assert_eq!(solution, (0, 0));
}
