use crate::utils;

fn split_rules_pages(content: &str) -> (&str, &str) {
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

fn get_pages_with_rules<'a>(pages_str: &'a str, rule_str: &'a str) -> Vec<Vec<&'a str>> {
    let pages_vecs: Vec<Vec<&str>> = pages_str
        .lines()
        .into_iter()
        .map(|l| l.split(",").collect())
        .collect();

    let mut all_follow_rules: Vec<Vec<&str>> = Vec::new();
    pages_vecs.iter().for_each(|pt| {
        let mut is_follow_rule: Vec<bool> = vec![];
        // check from i to each of the current pt against the rule
        // ie:
        // a vs b, a vs c, a vs d,
        // b vs c, b vs d
        // c vs d
        for i in 0..pt.len() {
            for j in i..pt.len() {
                let a = pt[i];
                let b = pt[j];
                for (tr_a, tr_b) in &rule_to_tuple(rule_str) {
                    if *tr_a == a && *tr_b == b {
                        is_follow_rule.push(true); // no need to check further, save time
                        break;
                    }
                    // if all the rule is false, then no push
                    // meaning if the is_follow_rule len is too short compared to the pt len (*)
                }
            }
        }
        // *: total of is_follow_rule pushed should be the same with total unique combination of pt
        // for the less means some of the pt iter did not follow the rule
        let expected_combinations = pt.len() * (pt.len() - 1) / 2;
        if is_follow_rule.len() == expected_combinations {
            all_follow_rules.push(pt.to_vec());
        }
    });
    all_follow_rules
}

pub fn test_run() {
    let test_rules = "47|53
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

    let test_check = "75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
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
    let middle_page_sum: i32 = get_pages_with_rules(test_check, test_rules)
        .iter()
        .map(|a| {
            let index = ((a.iter().len() / 2) as f32).ceil() as usize;
            a[index].parse::<i32>().unwrap()
        })
        .sum();
    println!("{:?}", middle_page_sum)
}

pub fn run() {
    match utils::file_reader::read_file("day5.txt") {
        Ok(contents) => {
            ({
                let (rules, pages) = split_rules_pages(&contents);
                let middle_page_sum: i32 = get_pages_with_rules(pages, rules)
                    .iter()
                    .map(|a| {
                        let index = ((a.iter().len() / 2) as f32).ceil() as usize;
                        a[index].parse::<i32>().unwrap()
                    })
                    .sum();
                println!("{:?}", middle_page_sum)
            })
        }
        Err(e) => println!("Err: {}", e),
    }
}
