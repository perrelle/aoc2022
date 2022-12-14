#[derive (Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cell { Empty, Rock, Sand }

type Grid = array2d::Array2D<Cell>;

type Point = (usize, usize);

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

    pub fn parse(input: &str) -> IResult<&str, Vec<Vec<Point>>> {
        let usize = |input| map(u32, |x| x as usize)(input);
        let point = separated_pair(usize, tag(","), usize);
        let path = separated_list1(tag(" -> "), point);
        let data = separated_list1(line_ending, path);
        all_consuming(terminated(data, multispace0))(input)
    }
}

fn draw_line(grid: &mut Grid, u: &Point, v: &Point) {
    if u.0 == v.0 {
        if u.1 < v.1 {
            for y in u.1..=v.1 {
                grid.set(y, u.0, Cell::Rock).unwrap();
            }
        }
        else {
            for y in v.1..=u.1 {
                grid.set(y, u.0, Cell::Rock).unwrap();
            }
        }
    }
    else if u.1 == v.1 {
        if u.0 < v.0 {
            for x in u.0..=v.0 {
                grid.set(u.1, x, Cell::Rock).unwrap();
            }
        }
        else {
            for x in v.0..=u.0 {
                grid.set(u.1, x, Cell::Rock).unwrap();
            }
        }
    }
    else {
        panic!()
    }
}


fn draw_path(grid: &mut Grid, path: &[Point])  {
    if let Some((mut u, tail)) = path.split_first() {
        for v in tail {
            draw_line(grid, u, v);
            u = v;
        }
    }
}

fn drop_sand(grid: &mut Grid, source: &Point) -> bool {
    let mut p = *source;

    loop {
        match  grid.get(p.1+1, p.0) {
            None => { return false; },
            Some (Cell::Empty) => {
                p.1 += 1;
                continue;
            },
            Some (_) => ()
        }

        if let Some (Cell::Empty) = grid.get(p.1+1, p.0-1) {
            p.1 += 1;
            p.0 -= 1;
            continue;
        }
        else if let Some (Cell::Empty) = grid.get(p.1+1, p.0+1) {
            p.1 += 1;
            p.0 += 1;
            continue;
        }
        else {
            assert_eq!(grid.get(p.1, p.0), Some(&Cell::Empty));
            grid.set(p.1, p.0, Cell::Sand).unwrap();
            return true;
        }
    }
}

pub fn print_grid(grid: &Grid) {
    for y in 0..12 {
        for x in 490..=510 {
            let c = match grid.get(y, x).unwrap() {
                Cell::Empty => ' ',
                Cell::Rock => '#',
                Cell::Sand => '~'
            };
            print!("{c}");
        }
        println!();
    }
}

fn simulate(grid: &mut Grid) -> u32 {
    let mut count: u32 = 0;
    let source = (500,0);

    while drop_sand(grid, &source) {
        count += 1;
        if grid.get(source.1,source.0) != Some(&Cell::Empty) {
            break;
        }
    }
 
    count
}

pub fn solve(input: &str) -> Option<(u32,u32)> {
    let (_,data) = parser::parse(input).unwrap();

    let mut grid = Grid::filled_with(Cell::Empty, 1000, 1000);
    data.iter().for_each(|path| draw_path(&mut grid, path));

    let solution1 = simulate(&mut grid.clone());

    let highest_y = data.iter().flatten().fold(0, |y,p| y.max(p.1));
    draw_line(&mut grid, &(0, highest_y + 2), &(999, highest_y + 2));
    let solution2 = simulate(&mut grid);

    Some((solution1, solution2))
}

#[test]
fn test14_1() {
    let solution = solve(&include_str!("../inputs/day14.1"));
    assert_eq!(solution, Some((24,93)));
}

#[test]
fn test14_2() {
    let solution = solve(&include_str!("../inputs/day14.2"));
    assert_eq!(solution, Some((817,23416)));
}
