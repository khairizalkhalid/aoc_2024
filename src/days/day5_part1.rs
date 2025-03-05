use crate::utils;

pub fn split_rules_pages(content: &str) -> (&str, &str) {
    let mut parts = content.split("\n\n");
    let first_part = parts.next().unwrap_or("");
    let second_part = parts.next().unwrap_or("");
    (first_part, second_part)
}

fn rule_to_tuple(rule_str: &str) -> Vec<(&str, &str)> {
    rule_str
        .lines()
        .map(|r| {
            let parts: Vec<&str> = r.split('|').collect();
            (parts[0], parts[1])
        })
        .collect()
}

pub fn get_pages_with_rules<'a>(
    pages_str: &'a str,
    rule_str: &'a str,
    is_against_rule: bool,
) -> Vec<Vec<&'a str>> {
    let rules = rule_to_tuple(rule_str);
    let pages_vecs: Vec<Vec<&str>> = pages_str
        .lines()
        .into_iter()
        .map(|l| l.split(",").collect())
        .collect();

    pages_vecs
        .into_iter()
        .filter(|pt| {
            let is_follow_rule = pt.iter().enumerate().all(|(i, &a)| {
                pt.iter()
                    .skip(i + 1)
                    .all(|&b| rules.iter().any(|&(tr_a, tr_b)| tr_a == a && tr_b == b))
            });
            !is_against_rule == is_follow_rule
        })
        .collect()
}

pub fn run() {
    // sort test rule into a searchable tree
    // iterate thru test check lines
    // iterate thru window and compares with test rule
    // if violate break if ok add in new vec named is_ordered
    // iterate thru is_ordered and find only the middle index value
    // .sum() and print
    //
    // or ... just bruteforce the checking (fuck it)

    // now find middle page number of all follow rules
    // then calculate sum of that middle page
    match utils::file_reader::read_file("day5.txt") {
        Ok(contents) => {
            let (rules, pages) = split_rules_pages(&contents);
            let middle_page_sum: i32 = get_pages_with_rules(pages, rules, false)
                .iter()
                .map(|a| {
                    let index = ((a.iter().len() / 2) as f32).ceil() as usize;
                    a[index].parse::<i32>().unwrap()
                })
                .sum();
            println!("{:?}", middle_page_sum)
        }
        Err(e) => println!("Err: {}", e),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split_rules_pages() {
        let input = "rules\n\npages";
        let (rules, pages) = split_rules_pages(input);
        assert_eq!(rules, "rules");
        assert_eq!(pages, "pages");

        let input_no_pages = "rules";
        let (rules, pages) = split_rules_pages(input_no_pages);
        assert_eq!(rules, "rules");
        assert_eq!(pages, "");
    }

    #[test]
    fn test_rule_to_tuple() {
        let input = "47|53\n97|13";
        let expected = vec![("47", "53"), ("97", "13")];
        assert_eq!(rule_to_tuple(input), expected);

        let input_empty = "";
        let expected_empty: Vec<(&str, &str)> = vec![];
        assert_eq!(rule_to_tuple(input_empty), expected_empty);
    }

    #[test]
    fn test_get_pages_with_rules() {
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
            vec!["75", "47", "61", "53", "29"],
            vec!["97", "61", "53", "29", "13"],
            vec!["75", "29", "13"],
        ];
        assert_eq!(get_pages_with_rules(pages, rules, false), expected);

        let pages_no_match = "75,97,47,61,53
61,13,29
97,13,75,29,47";
        let expected_no_match: Vec<Vec<&str>> = vec![];
        assert_eq!(
            get_pages_with_rules(pages_no_match, rules, false),
            expected_no_match
        );
    }
}
