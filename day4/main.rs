
struct SectionRange {
    start_id: u64,
    end_id: u64
}

impl SectionRange {
    fn is_fully_contained_in(&self, other: &SectionRange) -> bool {
        self.start_id >= other.start_id && self.end_id <= other.end_id
    }

    fn intersect_with(&self, other: &SectionRange) -> bool {
        (self.start_id >= other.start_id && self.start_id <= other.end_id)
                || (self.end_id >= other.start_id && self.end_id <= other.end_id)
                || (self.start_id <= other.start_id && self.end_id >= other.end_id)
    }
}

impl From<&str> for SectionRange {
    fn from(input: &str) -> SectionRange {
        let parts: Vec<&str> = input.split('-').collect();

        assert_eq!(parts.len(), 2);
        let start_id: u64 = parts[0].parse().unwrap();
        let end_id: u64 = parts[1].parse().unwrap();

        assert!(start_id <= end_id);

        SectionRange{ start_id, end_id }
    }
}

fn main() {
    do_part1();
    do_part2();
}

fn do_part1() {
    let input_str = std::fs::read_to_string("day4/input.txt").unwrap();

    let res = input_str.lines()
            .filter(|line| contains_fully_contained_range(line))
            .count();

    println!("Number of pairs with fully contained ranges is {}", res);
}

fn do_part2() {
    let input_str = std::fs::read_to_string("day4/input.txt").unwrap();

    let res = input_str.lines()
            .filter(|line| contains_intersection(line))
            .count();

    println!("Number of pairs with intersection of ranges is {}", res);
}

fn contains_fully_contained_range(line: &str) -> bool {
    let parts: Vec<&str> = line.split(',').collect();

    let range1 = SectionRange::from(parts[0]);
    let range2 = SectionRange::from(parts[1]);

    range1.is_fully_contained_in(&range2) || range2.is_fully_contained_in(&range1)
}

fn contains_intersection(line: &str) -> bool {
    let parts: Vec<&str> = line.split(',').collect();

    let range1 = SectionRange::from(parts[0]);
    let range2 = SectionRange::from(parts[1]);

    range1.intersect_with(&range2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section_range_is_fully_contained_in_works() {
        let range1 = SectionRange { start_id: 1, end_id: 2 };
        let range2 = SectionRange { start_id: 1, end_id: 4 };

        let res = range1.is_fully_contained_in(&range2);

        assert_eq!(res, true);

        let res = range2.is_fully_contained_in(&range1);

        assert_eq!(res, false);
    }

    #[test]
    fn section_range_from_str_works() {
        let range = SectionRange::from("5-6");

        assert_eq!(range.start_id, 5);
        assert_eq!(range.end_id, 6);
    }

    #[test]
    fn contains_fully_contained_range_works() {
        let res = contains_fully_contained_range("6-6,4-8");
        assert_eq!(res, true);

        let res = contains_fully_contained_range("6-6,4-5");
        assert_eq!(res, false);
    }

    #[test]
    fn intersect_with_works() {
        let range1 = SectionRange { start_id: 1, end_id: 2 };
        let range2 = SectionRange { start_id: 1, end_id: 4 };
        let range3 = SectionRange { start_id: 4, end_id: 100};
        let range4 = SectionRange { start_id: 2, end_id: 8 };
        let range5 = SectionRange { start_id: 3, end_id: 7};

        assert_eq!(range1.intersect_with(&range2), true);
        assert_eq!(range1.intersect_with(&range3), false);
        assert_eq!(range4.intersect_with(&range5), true);
    }

    #[test]
    fn contains_intersection_works() {
        assert_eq!(contains_intersection("5-7,7-9"), true);
        assert_eq!(contains_intersection("2-8,3-7"), true);
    }

}