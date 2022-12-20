mod parser {
    use nom::{
        IResult,
        character::complete::*,
        combinator::*,
        sequence::*,
        multi::*
    };

    pub fn parse(input: &str) -> IResult<&str, Vec<i32>> {
        let data = separated_list1(multispace1, i32);
        all_consuming(terminated(data, multispace0))(input)
    }
}

pub fn solve_part(data: &[i32], key: i32, iterations: u32) -> i64 {

    let mut v: Vec<(usize, i64)> =
        data.iter().enumerate().map(|(i,x)| (i,*x as i64 * key as i64)).collect();
    let n = data.len();

    for _iteration in 1..=iterations {
        for index in 0..n {
            let i = v.iter().position(|(k,_)| *k == index).unwrap();

            let x = v[i].1;
            let d = (i as i64 + x).rem_euclid((n - 1) as i64) as usize;

            if d > i {
                for j in i..d {
                    v[j] = v[j+1];
                }
            }
            else {
                for j in ((d+1)..=i).rev() {
                    v[j] = v[j-1];
                }
            }
            v[d] = (index,x);
        }
    }

    let zero_index = v.iter().position(|(_,x)| *x == 0).unwrap();
    v[(zero_index + 1000) % n].1 +
    v[(zero_index + 2000) % n].1 +
    v[(zero_index + 3000) % n].1
}

pub fn solve(input: &str) -> (i64,i64) {
    let (_,data) = parser::parse(input).unwrap();
        
    let solution1 = solve_part(&data, 1, 1);
    let solution2 = solve_part(&data, 811589153, 10);

    (solution1, solution2)
}

#[test]
fn test20_1() {
    let solution = solve(&include_str!("../inputs/day20.1"));
    assert_eq!(solution, (3,1623178306));
}

#[test]
fn test20_2() {
    let solution = solve(&include_str!("../inputs/day20.2"));
    assert_eq!(solution, (2827,7834270093909));
}
