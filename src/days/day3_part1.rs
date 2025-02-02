use regex::Regex;

pub fn test_run() {
    // split string by regex exp `mul\(\d+,\d+\)`
    // for each set of string
    // remove the word 'mul'
    // split using comma, first item remove infront while the second remove the end
    // parse to i32
    // calculate multiplication between the two i32
    // push to a new vec
    // then iter sum it up.

    let test_case = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    let reg = Regex::new(r"mul\(\d+,\d+\)").unwrap();

    let Some(caps) = reg.captures(test_case) else {
        println!("not found");
        return;
    };
    println!("{:?}", &caps)
}
