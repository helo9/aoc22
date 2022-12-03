use std::collections::HashSet;

const INPUT_FILE: &str = "day3/input.txt";

fn main() {
    do_part1();

    do_part2();
}

fn do_part1 () {
    let input_str = std::fs::read_to_string(INPUT_FILE).unwrap();
    
    let result: u32 = input_str.lines()
             .map(|line| find_wrong_packed_item(line))
             .map(|item| get_item_priority(item))
             .sum();
    
    println!("Result for part1 is {}!", result);
}

fn do_part2() {
    let input_str = std::fs::read_to_string(INPUT_FILE).unwrap();

    let mut sum: u32 = 0;
    
    for group_slice in input_str.lines().collect::<Vec<_>>().chunks(3) {

        let group: &[&str; 3] = group_slice.try_into().unwrap();

        let badge_item = find_common_item(group);

        sum += get_item_priority(badge_item);
    }

    println!("Result for part1 is {}!", sum);
}

fn find_wrong_packed_item(content: &str) -> char {
    let item_number = content.len();

    let content_first_compartment = &content[0..item_number/2];
    let content_scnd_compartment = &content[item_number/2..];

    let mut items_first_compartment = HashSet::new();

    for item in content_first_compartment.chars() {
        items_first_compartment.insert(item);
    }

    for item in content_scnd_compartment.chars() {
        if items_first_compartment.contains(&item) {
            return item;
        }
    }

    todo!();
}

fn find_common_item(contents: &[&str; 3]) -> char {
    let items1: HashSet<char> = contents[0].chars().collect();
    let items2: HashSet<char> = contents[1].chars().collect();
    let items3: HashSet<char> = contents[2].chars().collect();

    for item_in_1_and_2 in items1.intersection(&items2) {
        if items3.contains(item_in_1_and_2) {
            return *item_in_1_and_2;
        }
    }
    
    todo!();
}

fn get_item_priority(item: char) -> u32 {
    const SMALL_CAP_OFFSET: u32 = 96;
    const BIG_CAP_OFFSET: u32 = 38;
    
    match item {
        'a'..='z' => {
            <char as Into<u32>>::into(item) - SMALL_CAP_OFFSET
        },
        'A'..='Z' => {
            <char as Into<u32>>::into(item) - BIG_CAP_OFFSET
        },
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn find_wrong_packed_item_works() {
        const INPUT_STR: &str = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let result = find_wrong_packed_item(INPUT_STR);
        assert_eq!(result, 'p');

        const INPUT_STR2: &str = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        let result = find_wrong_packed_item(INPUT_STR2);
        assert_eq!(result, 'L');
    }

    #[test]
    fn get_item_priority_works() {
        let result = get_item_priority('b');
        assert_eq!(result, 2);

        let result = get_item_priority('B');
        assert_eq!(result, 28);

        let result = get_item_priority('Z');
        assert_eq!(result, 52);
    }

    #[test]
    fn find_common_item_works() {
        const INPUT_GROUP: [&str; 3] = [
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg"
        ];

        let result = find_common_item(&INPUT_GROUP);
        assert_eq!(result, 'r');
    }
}