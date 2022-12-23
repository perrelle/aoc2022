use std::{fmt::Display, ops::{Index, IndexMut}};

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


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction { North, South, West, East }

impl Direction {
    fn move_forward(&self, (x,y): (usize, usize)) -> (usize, usize) {
        match self {
            Direction::North => (x,y-1),
            Direction::South => (x,y+1),
            Direction::West => (x-1,y),
            Direction::East => (x+1,y)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Choice {
    Stay,
    Move(Direction)
}

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Empty,
    Dwarf,
    Targeted(u32)
}

impl Cell {
    fn is_empty(&self) -> bool {
        match self {
            Cell::Empty | Cell::Targeted(_) => true,
            Cell::Dwarf => false
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Dwarf {
    position: (usize, usize),
    choice: Choice
}

#[derive(Debug)]
struct Grid ([[Cell; SIZE]; SIZE]);

impl Grid {
    fn is_empty(&self, (x,y) : (usize, usize)) -> bool {
        self[(x,y)].is_empty()
    }

    fn is_isolated(&self, (x,y) : (usize, usize)) -> bool {
        self.is_empty((x-1, y-1)) &&
        self.is_empty((x, y-1)) &&
        self.is_empty((x+1, y-1)) &&
        self.is_empty((x-1, y)) &&
        self.is_empty((x+1, y)) &&
        self.is_empty((x-1, y+1)) &&
        self.is_empty((x, y+1)) &&
        self.is_empty((x+1, y+1))
    }

    fn can_move(&self, (x,y): (usize, usize), d: Direction) -> bool {
        match d {
            Direction::North =>
                self.is_empty((x-1, y-1)) &&
                self.is_empty((x, y-1)) &&
                self.is_empty((x+1, y-1)),
            Direction::South =>
                self.is_empty((x-1, y+1)) &&
                self.is_empty((x, y+1)) &&
                self.is_empty((x+1, y+1)),
            Direction::West => 
                self.is_empty((x-1, y-1)) &&
                self.is_empty((x-1, y)) &&
                self.is_empty((x-1, y+1)),
            Direction::East =>
                self.is_empty((x+1, y-1)) &&
                self.is_empty((x+1, y)) &&
                self.is_empty((x+1, y+1)),
        }
    }

    fn target_cell(&mut self, (x,y): (usize, usize)) {
        self[(x,y)] = match self[(x,y)] {
            Cell::Empty => Cell::Targeted(1),
            Cell::Targeted(i) => Cell::Targeted(i+1),
            Cell::Dwarf => panic!()
        };
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = Cell;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.0[y][x]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.0[y][x]
    }
}

const SIZE: usize = 200;

#[derive(Debug)]
pub struct Board {
    grid: Grid,
    dwarves: Vec<Dwarf>
}

impl Board {
    pub fn from_rows(rows: &[Vec<bool>]) -> Self {
        let grid = Grid([[Cell::Empty ; SIZE] ; SIZE]);
        let offset = (SIZE - rows.len()) / 2;
        let mut board = Board{grid, dwarves: Vec::new()};

        for (y, row) in rows.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell {
                    board.add_dwarf(x + offset,y + offset);
                }
            }
        }

        board
    }

    fn hull(&self) -> (usize, usize, usize, usize) {
        let mut area = (usize::MAX, usize::MAX, 0, 0);
        for Dwarf{position:(x,y), ..} in self.dwarves.iter() {
            area.0 = area.0.min(*x);
            area.1 = area.1.min(*y);
            area.2 = area.2.max(*x);
            area.3 = area.3.max(*y);
        }
        area
    }

    fn iter_area(&self, area: (usize, usize, usize, usize)) -> impl Iterator<Item = Cell> + '_ {
        (area.1..=area.3).flat_map(move |y| {
            (area.0..=area.2).map(move |x| {
                self.grid[(x,y)]
            })
        })
    }

    fn add_dwarf(&mut self, x: usize, y: usize) {
        self.dwarves.push(Dwarf{position: (x,y), choice:Choice::Stay});
        self[(x,y)] = Cell::Dwarf;
    }

    pub fn step(&mut self, directions: &[Direction]) -> bool {
        // Phase 1
        for dwarf in self.dwarves.iter_mut() {
            dwarf.choice = Choice::Stay;
            if !self.grid.is_isolated(dwarf.position) {
                for d in directions {
                    if self.grid.can_move(dwarf.position, *d) {
                        dwarf.choice = Choice::Move(*d);
                        self.grid.target_cell(d.move_forward(dwarf.position));
                        break;
                    }
                }
            }
        }

        // Phase 2
        let mut did_elves_move = false;
        for dwarf in self.dwarves.iter_mut() {
            if let Choice::Move(d) = dwarf.choice {
                let destination = d.move_forward(dwarf.position);
                if let Cell::Targeted(i) = self.grid[destination] {
                    if i <= 1 {
                        self.grid[dwarf.position] = Cell::Empty;
                        dwarf.position = destination;
                        self.grid[dwarf.position] = Cell::Dwarf;
                        did_elves_move = true;
                    }
                    else {
                        self.grid[destination] = Cell::Empty;
                    }
                }
            }
        }

        did_elves_move
    }

}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let area = self.hull();
        for y in area.1..=area.3 {
            for x in area.0..=area.2 {
                let c = match self[(x,y)] {
                    Cell::Empty => '.',
                    Cell::Targeted(_) => 'o',
                    Cell::Dwarf => {
                        let dwarf = self.dwarves.iter().find(|dwarf| {
                            dwarf.position == (x,y)
                        }).unwrap();
                        match dwarf.choice {
                            Choice::Stay => '#',
                            Choice::Move(Direction::North) => '↑',
                            Choice::Move(Direction::South) => '↓',
                            Choice::Move(Direction::West) => '→',
                            Choice::Move(Direction::East) => '←',
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
        self.grid.index((x,y))
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        self.grid.index_mut((x,y))
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
            let area = board.hull();
            solution1 =
                board.iter_area(area).filter(|c| c.is_empty()).count();
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
    assert_eq!(solution, (3877, 982));
}
