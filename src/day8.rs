use array2d::Array2D;

pub struct Grid<T: std::clone::Clone>(Array2D<T>);

impl<T: std::clone::Clone> std::ops::Deref for Grid<T> {
    type Target = Array2D<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: std::clone::Clone> std::ops::DerefMut for Grid<T> {
    fn deref_mut(&mut self) -> &mut Array2D<T> {
        &mut self.0
    }
}

impl<T: std::clone::Clone + std::fmt::Display> std::fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row_iter in self.rows_iter() {
            for element in row_iter {
                write!(f, "{element} ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}


mod parser  {
    use nom::{
        IResult,
        character::complete::*,
        combinator::*,
        sequence::*,
        multi::*};

    use array2d::Array2D;

    pub fn parse(input: &str) -> IResult<&str, Array2D<u8>> {
        let digit = map(
            satisfy(|c| c.is_ascii_digit()),
            |c| c.to_digit(10).unwrap() as u8);
        let array = map(
            separated_list1(line_ending, many1(digit)),
            |v| Array2D::from_rows(&v));
        all_consuming(terminated(array, multispace0))(input)
    }
}

pub fn scenic_score(grid: &Grid<u8>, i: usize, j: usize) -> Option<usize> {
    let h = *grid.get(i,j)?;
    let mut s1 = i;
    for (s,il) in (0..i).rev().enumerate() {
        let g = *grid.get(il, j)?;
        if g >= h {
            s1 = s+1;
            break;
        }
    }
    let mut s2 = grid.row_len() - i - 1;
    for (s,il) in (i+1..grid.row_len()).enumerate() {
        let g = *grid.get(il, j)?;
        if g >= h {
            s2 = s+1;
            break;
        }
    }
    let mut s3 = j;
    for (s,jl) in (0..j).rev().enumerate() {
        let g = *grid.get(i, jl)?;
        if g >= h {
            s3 = s+1;
            break;
        }
    }
    let mut s4 = grid.column_len() - j - 1;
    for (s,jl) in (j+1..grid.column_len()).enumerate() {
        let g = *grid.get(i, jl)?;
        if g >= h {
            s4 = s+1;
            break;
        }
    }
    Some(s1*s2*s3*s4)
}


pub fn solve(input: &str) -> Option<(usize,usize)> {
    let (_,data) = parser::parse(input).unwrap();

    let grid = Grid(data);
    let mut markers = Grid(
        Array2D::filled_with(false, grid.row_len(), grid.column_len())
    );

    for i in 0..grid.row_len() {
        let mut highest = -1;
        for j in 0..grid.column_len() {
            let x = *grid.get(i,j)? as i32;
            if x > highest {
                highest = x;
                markers.set(i, j, true).unwrap();
            }
        }
        let mut highest = -1;
        for j in (0..grid.column_len()).rev() {
            let x = *grid.get(i,j)? as i32;
            if x > highest {
                highest = x;
                markers.set(i, j, true).unwrap();
            }
        }
    }

    for j in 0..grid.column_len() {
        let mut highest = -1;
        for i in 0..grid.row_len() {
            let x = *grid.get(i,j)? as i32;
            if x > highest {
                highest = x;
                markers.set(i, j, true).unwrap();
            }
        }
        let mut highest = -1;
        for i in (0..grid.row_len()).rev() {
            let x = *grid.get(i,j)? as i32;
            if x > highest {
                highest = x;
                markers.set(i, j, true).unwrap();
            }
        }
    }

    let solution1 = markers.elements_row_major_iter().filter(|&&x| x).count();
    let mut solution2 = 0;

    for i in 0..grid.row_len() {
        for j in 0..grid.column_len() {
            let x = scenic_score(&grid, i, j)?;
            if x > solution2 {
                solution2 = x;
            }
        }
    }

    

    Some ((solution1,solution2))
}

#[test]
fn test8_1() {
    let solution = solve(&include_str!("../inputs/day8.1"));
    assert_eq!(solution, Some ((21,8)));
}

#[test]
fn test8_2() {
    let solution = solve(&include_str!("../inputs/day8.2"));
    assert_eq!(solution, Some ((1792,334880)));
}

