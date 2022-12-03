
#[derive(Debug, PartialEq, Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Into<u32> for Shape {
    fn into(self) -> u32 {
        self as u32
    }
}

impl From<char> for Shape {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            _ => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6
}

impl From<char> for Outcome {
    fn from(c: char) -> Self {
        match c {
            'X' => Outcome::Loss,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => todo!(),
        }
    }
}

impl Into<u32> for Outcome {
    fn into(self) -> u32 {
        self as u32
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let input_str = std::fs::read_to_string("day2/input.txt").unwrap();

    let res: u32 = input_str.lines()
                       .map(|line| calculate_score(line))
                       .sum();

    println!("Part1 result is {}", res);
}

fn part2() {
    let input_str = std::fs::read_to_string("day2/input.txt").unwrap();

    let res: u32 = input_str.lines()
                        .map(|line| calculate_score2(line))
                        .sum();

    println!("Part2 result is {}", res);
}


fn calculate_score(tip: &str) -> u32 {
    let elves_shape = Shape::from(tip.chars().nth(0).unwrap());
    let my_shape = Shape::from(tip.chars().nth(2).unwrap());
    

    let outcome = does_first_shape_win(&my_shape, &elves_shape);

    let result: u32 = <Outcome as Into<u32>>::into(outcome) + <Shape as Into<u32>>::into(my_shape);

    result
}

fn calculate_score2(tip: &str) -> u32 {
    let elves_shape = Shape::from(tip.chars().nth(0).unwrap());
    let target_outcome = Outcome::from(tip.chars().nth(2).unwrap());

    let all_shapes = vec![Shape::Rock, Shape::Scissors, Shape::Paper];

    let my_shape = all_shapes.iter()
                    .find(|shape| {does_first_shape_win(&shape, &elves_shape)==target_outcome})
                    .unwrap();

    let result: u32 = <Outcome as Into<u32>>::into(target_outcome) + <Shape as Into<u32>>::into(*my_shape);

    result
}

fn does_first_shape_win(first_shape: &Shape, scnd_shape: &Shape) -> Outcome {
    if first_shape == scnd_shape {
        Outcome::Draw
    } else {
        let winning = match first_shape {
            Shape::Rock => {scnd_shape == &Shape::Scissors},
            Shape::Paper => {scnd_shape == &Shape::Rock},
            Shape::Scissors => {scnd_shape == &Shape::Paper},
        };

        if winning {
            Outcome::Win
        } else {
            Outcome::Loss
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn shape_from_char_works() {
        let result = Shape::from('A');
        assert_eq!(result, Shape::Rock);

        let result = Shape::from('B');
        assert_eq!(result, Shape::Paper);
    }

    #[test]
    fn outcome_from_char_works() {
        assert_eq!(Outcome::from('X'), Outcome::Loss);
    }

    #[test]
    fn does_first_shape_win_work() {
        assert_eq!(does_first_shape_win(&Shape::Rock, &Shape::Paper), Outcome::Loss);
        assert_eq!(does_first_shape_win(&Shape::Scissors, &Shape::Paper), Outcome::Win);
        assert_eq!(does_first_shape_win(&Shape::Rock, &Shape::Rock), Outcome::Draw);

    }

    #[test]
    fn shape_into_works() {
        let result: u32 = Shape::Rock.into();
        assert_eq!(result, 1);
    }

    #[test]
    fn outcome_into_works() {
        let result: u32 = Outcome::Win.into();
        assert_eq!(result, 6);
    }

    #[test]
    fn calculate_score_works() {
        let result = calculate_score("A Y");
        assert_eq!(result, 8);
        
        let result = calculate_score("B X");
        assert_eq!(result, 1);
    }

    #[test]
    fn calculate_score2_works() {
        let result = calculate_score2("A Y");
        assert_eq!(result, 4);
        
        let result = calculate_score2("B X");
        assert_eq!(result, 1);
    }
}