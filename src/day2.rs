mod parser  {
    use nom::{IResult, multi::*, sequence::*, character::complete::*, combinator::*};

    pub fn parse(input: &[u8]) -> IResult<&[u8], Vec<(char,char)>> {
        let line = separated_pair(one_of("ABC"), space1, one_of("XYZ"));
        let (input, data) = separated_list1(line_ending, line)(input)?;
        let (input, _) = all_consuming(multispace0)(input)?;
        Ok((input, data))
    }
}

#[derive(Clone, Copy)]
enum Shape { Rock, Paper, Scissors }

#[derive(PartialEq)]
enum Result {Loss, Draw, Win }

fn round(opponent: Shape, player: Shape) -> Result {
    match opponent {
        Shape::Rock => match player {
            Shape::Rock => Result::Draw,
            Shape::Paper => Result::Win,
            Shape::Scissors => Result::Loss
         },
         Shape::Paper => match player {
            Shape::Rock => Result::Loss,
            Shape::Paper => Result::Draw,
            Shape::Scissors => Result::Win
        },
        Shape::Scissors => match player {
            Shape::Rock => Result::Win,
            Shape::Paper => Result::Loss,
            Shape::Scissors => Result::Draw,
        }
    }
}

fn score(opponent: Shape, player: Shape) -> i32 {
    let choice_score = match player {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3
    };
    let result_score = match round(opponent, player) {
        Result::Loss => 0,
        Result::Draw => 3,
        Result::Win => 6
    };
    choice_score + result_score
}

pub fn solve(input: &[u8]) -> Option<(i32,i32)> {
    let (_,data) = parser::parse(input).ok()?;  

    let mut score1 = 0;
    let mut score2 = 0;

    for (opponent, player) in data {
        let opponent_play = match opponent {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,
            _ => return None
        };
        let player_play1 = match player {
            'X' => Shape::Rock,
            'Y' => Shape::Paper,
            'Z' => Shape::Scissors,
            _ => return None
        };
        let result2 = match player {
            'X' => Result::Loss,
            'Y' => Result::Draw,
            'Z' => Result::Win,
            _ => return None
        };
        let choices = [Shape::Rock, Shape::Paper, Shape::Scissors];
        let &player_play2 =
            choices.iter().find(|&&p| round(opponent_play, p) == result2)?;

        score1 += score(opponent_play, player_play1);
        score2 += score(opponent_play, player_play2);
    }
    Some ((score1, score2))
}

#[test]
fn test2_1() {
    let solution = solve(include_bytes!("../inputs/day2.1"));
    assert_eq!(solution, Some ((15,12)));
}

#[test]
fn test2_2() {
    let solution = solve(include_bytes!("../inputs/day2.2"));
    assert_eq!(solution, Some ((15632,14416)));
}
