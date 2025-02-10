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
}
