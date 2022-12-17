use std::{fmt, collections::{VecDeque, HashMap}};

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

#[derive (PartialEq, Eq, Clone)]
struct Grid {
    cells: VecDeque<[bool; 7]>,
    offset: usize
}

const GRID_SIZE: usize = 38;

impl Grid {
    fn empty() -> Self {
        Grid { cells: VecDeque::new(), offset: 0 }
    }

    fn new_line(&mut self) {
        if self.cells.len() >= GRID_SIZE {
            self.cells.pop_front();
            self.offset += 1;
        }
        self.cells.push_back([false; 7]);
    }

    fn get(&self, x: usize, y: usize) -> bool {
        assert!(x < 7);
        assert!(y >= self.offset);
        if y - self.offset >= self.cells.len() {
            return false;
        }
        self.cells[y - self.offset][x]
    }

    fn set(&mut self, x: usize, y: usize) {
        assert!(x < 7);
        assert!(y >= self.offset);
        while y - self.offset >= self.cells.len() {
            self.new_line();
        }
        self.cells[y - self.offset][x] = true;
    }

    fn height(&self) -> usize {
        self.cells.len() + self.offset
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

    fn same_top(&self, other: &Self) -> bool {
        self.cells == other.cells
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.cells.iter().rev() {
            for &cell in row {
                write!(f, "{}", if cell {'#'} else {'.'})?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

pub fn solve_part(wind: &[Direction], limit: u64) -> usize {
    type State = (usize, usize);
    type Cache = HashMap<State, (u64,Grid)>;

    let mut grid = Grid::empty();
    let mut shape_iterator = SHAPES.iter().enumerate().cycle();
    let mut direction_iterator = wind.iter().enumerate().cycle();
    let mut cache = Cache::new();
    let mut cycle = 1;
    let mut did_fast_forward = false;

    while cycle <= limit {
        let (i,shape) = shape_iterator.next().unwrap();
        let (w, h) = (shape[0].len(), shape.len());
        let mut x = 2;
        let mut y = grid.height() + shape.len() + 2;

        loop {
            let (j,&direction) = direction_iterator.next().unwrap();
            match direction {
                Direction::Left => {
                    if x > 0 && grid.shape_fits(shape, x-1, y) {
                        x -= 1;
                    }
                },
                Direction::Right => {
                    if x + w < 7 && grid.shape_fits(shape, x+1, y) {
                        x += 1;
                    }
                }
            }

            if y >= h && grid.shape_fits(shape, x, y-1) {
                y -= 1;
            }
            else {
                grid.place_shape(shape, x, y);
                if !did_fast_forward {
                    if let Some(state) = cache.get(&(i,j)) {
                        let (old_cycle,old_grid) = state;
                        if grid.same_top(old_grid) {
                            let period = cycle - old_cycle;
                            let height = grid.height() - old_grid.height();
                            let epochs = (limit - cycle) / period;
                            cycle += epochs * period;
                            grid.offset += epochs as usize * height;
                            did_fast_forward = true
                        }
                    }
                    cache.insert((i,j), (cycle, grid.clone()));
                }
                break;
            }
        }

        cycle += 1;
    }

    grid.height()
}


pub fn solve(input: &str) -> (usize,usize) {
    let (_,data) = parser::parse(input).unwrap();

    (solve_part(&data, 2022), solve_part(&data, 1000000000000))
}


#[test]
fn test17_1() {
    let solution = solve(&include_str!("../inputs/day17.1"));
    assert_eq!(solution, (3068,1514285714288));
}

#[test]
fn test16_2() {
    let solution = solve(&include_str!("../inputs/day17.2"));
    assert_eq!(solution, (3100,1540634005751));
}
