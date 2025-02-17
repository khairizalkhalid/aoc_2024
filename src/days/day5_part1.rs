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

    let tuple_rule: Vec<(&str, &str)> = test_rules
        .lines()
        .map(|r| {
            let parts: Vec<&str> = r.split('|').collect();
            (parts[0], parts[1])
        })
        .collect();

    let p_test: Vec<Vec<&str>> = test_check
        .lines()
        .into_iter()
        .map(|l| l.split(",").collect())
        .collect();

    let mut all_follow_rules: Vec<Vec<&str>> = Vec::new();
    p_test.iter().for_each(|pt| {
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
                for (tr_a, tr_b) in &tuple_rule {
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

    println!("{:?}", all_follow_rules);

    // now find middle page number of all follow rules
    // then calculate sum of that middle page
}
