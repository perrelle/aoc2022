use nom::InputIter;

mod parser {
    use nom::{
        IResult,
        character::complete::*,
        combinator::*,
        sequence::*,
        multi::*
    };

    pub fn parse(input: &str) -> IResult<&str, Vec<Vec<char>>> {
        let digit = one_of("012=-");
        let data = separated_list1(line_ending, many1(digit));
        all_consuming(terminated(data, multispace0))(input)
    }
}

pub fn snafu_to_int(x: &str) -> i64 {
    fn convert_digit(d: char) -> i64 {
         match d {
            '0' ..= '2' => (d as u8 - b'0') as i64,
            '=' => -2,
            '-' => -1,
            _ => panic!()
         }
    }

    let mut y = 0;
    for digit in x.iter_elements() {
        y = y * 5 + convert_digit(digit);
    }
    y
}


pub fn int_to_snafu(mut x: i64) -> String {
    fn convert_digit(d: i64) -> (char, i64) {
        match d {
            0 ..= 2 => ((d as u8 + b'0') as char, 0),
            3 => ('=', 1),
            4 => ('-', 1),
            _ => panic!()
        }
    }

    let mut s = String::new();
    while x != 0 {
        let (digit, remainder) = convert_digit(x % 5);
        s.push(digit);
        x = x / 5 + remainder;
    }
    s.chars().rev().collect()
}


pub fn solve(input: &str) -> (String,u32) {
    let (_,data) = parser::parse(input).unwrap();

    let mut sum : i64 = 0;
    for number in &data {
        let number: String = number.iter().collect();
        sum += snafu_to_int(&number);
    }

    let solution1 = int_to_snafu(sum);

    (solution1,0)
}

#[test]
fn test25_1() {
    let solution = solve(&include_str!("../inputs/day25.1"));
    assert_eq!(solution, (String::from("2=-1=0"), 0));
}

#[test]
fn test25_2() {
    let solution = solve(&include_str!("../inputs/day25.2"));
    assert_eq!(solution, (String::from("2--1=0=-210-1=00=-=1"), 0));
}
