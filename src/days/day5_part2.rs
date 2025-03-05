use crate::{days::day5_part1::split_rules_pages, utils};

pub fn run() {
    match utils::file_reader::read_file("day5.txt") {
        Ok(contents) => {
            let (rules, pages) = split_rules_pages(&contents);
            println!("{}{}", rules, pages)
            // get pages which violate the rules
            // rearrage them
            // calculate the middle
        }
        Err(e) => println!("Err: {}", e),
    }
}

#[cfg(test)]
mod test {
    use crate::days::day5_part1::get_pages_with_rules;

    #[test]
    fn test_get_pages_against_rules() {
        let rules = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13";

        let pages = "75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        let expected = vec![
            vec!["75", "97", "47", "61", "53"],
            vec!["61", "13", "29"],
            vec!["97", "13", "75", "29", "47"],
        ];
        assert_eq!(get_pages_with_rules(pages, rules, true), expected);

        let pages_match = "75,47,61,53,29
97,61,53,29,13
75,29,13";
        let expected_no_match: Vec<Vec<&str>> = vec![];
        assert_eq!(
            get_pages_with_rules(pages_match, rules, true),
            expected_no_match
        );
    }
}
