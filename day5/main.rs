use regex::Regex;

fn main() {
    do_part1();
    do_part2();
}

fn do_part1() {
    const INPUT_FILE: &str = "day5/input.txt";

    let (mut stacks, moves) = parse_input(INPUT_FILE);

    for amove in moves {
        amove.apply(&mut stacks);
    }

    let res: String = stacks.iter().map(|stack| stack.last().copied().unwrap()).collect();

    println!("{}", res);
}

fn do_part2() {
    const INPUT_FILE: &str = "day5/input.txt";

    let (mut stacks, moves) = parse_input(INPUT_FILE);

    for amove in moves {
        amove.apply_mover9001(&mut stacks);
    }

    let res: String = stacks.iter().map(|stack| stack.last().copied().unwrap()).collect();

    println!("{}", res);
}

type Location = usize;
type Stack = Vec<char>;
#[derive(Debug)]
struct Move {
    number_of_crates: usize,
    origin: Location,
    target: Location,
}

impl Move {
    fn apply(&self, stacks: &mut Vec<Stack>) {
        let mut crates_to_move = self.number_of_crates;

        while let Some(acrate) = stacks[self.origin-1].pop() {
            stacks[self.target-1].push(acrate);
    
            crates_to_move = crates_to_move - 1;
    
            if crates_to_move == 0 {
                break;
            }
        }
    }

    fn apply_mover9001(&self, stacks: &mut Vec<Stack>) {
        let crates_to_move = self.number_of_crates;

        let split_index = stacks[self.origin-1].len() - crates_to_move;

        let mut moved_stack = stacks[self.origin-1].split_off(split_index);

        stacks[self.target-1].append(&mut moved_stack);
    }
}

impl TryFrom<&str> for Move {

    type Error = ();

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

        let res = if let Some(capture) = re.captures_iter(input).next() {
            let number_of_crates: usize = capture[1].parse().unwrap();
            let origin: usize = capture[2].parse().unwrap();
            let target: usize = capture[3].parse().unwrap();

            Ok ( Move {
                number_of_crates,
                origin,
                target
            })
        } else {
            Err(())
        };

        res
    }
}

fn parse_input(filename: &str) -> (Vec<Stack>, Vec<Move>) {
    
    let input_str = std::fs::read_to_string(filename).unwrap();

    let mut line_iterator = input_str.lines();
    let starting_stacks_lines: Vec<&str> = line_iterator
            .by_ref()
            .take_while(|line| line.contains('['))
            .collect();

    let crate_layers: Vec<Vec<char>> = starting_stacks_lines.iter().rev()
                    .map(|line| line.chars().skip(1).step_by(4).collect())
                    .collect();

    let number_of_stacks: usize = crate_layers[0].len();

    let crate_stacks: Vec<Vec<char>> = (0..number_of_stacks)
                                    .map(|stack_no| crate_layers.iter().map(|layer| layer[stack_no]).take_while(|c| *c != ' ').collect())
                                    .collect();

    let moves: Vec<Move> = line_iterator.map(|line| line.try_into()).filter_map(|x| x.ok()).collect();

    (crate_stacks, moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_from_str_works() {
        const INPUT_STR: &str = "move 12323 from 1123 to 13239\n";

        let result: Move = INPUT_STR.try_into().unwrap();

        assert_eq!(result.number_of_crates, 12323);
        assert_eq!(result.origin, 1123);
        assert_eq!(result.target, 13239);
    }

    #[test]
    fn parse_input_works() {
        const EXAMPLE_FILE: &str = "day5/input_example.txt";

        let (stacks, moves) = parse_input(EXAMPLE_FILE);

        assert_eq!(stacks[0], vec!['Z', 'N',]);
        assert_eq!(stacks[1], vec!['M', 'C', 'D']);

        assert_eq!(moves[0].number_of_crates, 1);
        assert_eq!(moves[0].origin, 2);
        assert_eq!(moves[0].target, 1);
        assert_eq!(moves.len(), 4);
    }

    #[test]
    fn do_move_works() {
        const EXAMPLE_FILE: &str = "day5/input_example.txt";

        let (mut stacks, moves) = parse_input(EXAMPLE_FILE);

        moves[0].apply(&mut stacks);

        assert_eq!(stacks[0], vec!['Z','N','D']);
        assert_eq!(stacks[1], vec!['M','C']);
    }

    #[test]
    fn do_move_9001_works() {
        const EXAMPLE_FILE: &str = "day5/input_example.txt";

        let (mut stacks, moves) = parse_input(EXAMPLE_FILE);

        moves[0].apply_mover9001(&mut stacks);
        moves[1].apply_mover9001(&mut stacks);

        assert_eq!(stacks[2], vec!['P','Z','N','D']);
    }

    #[test]
    fn do_example_2_works() {
        const EXAMPLE_FILE: &str = "day5/input_example.txt";

        let (mut stacks, moves) = parse_input(EXAMPLE_FILE);

        for amove in moves {
            amove.apply_mover9001(&mut stacks);

            println!("{:?}", stacks);
        }

        let res: String = stacks.iter().map(|stack| stack.last().copied().unwrap()).collect();

        println!("{}", res);
        assert_eq!(res, "MCD");
    }

}