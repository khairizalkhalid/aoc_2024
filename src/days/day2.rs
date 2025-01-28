//test data
//7 6 4 2 1
//1 2 7 8 9
//9 7 6 2 1
//1 3 2 4 5
//8 6 4 4 1
//1 3 6 7 9

//test result
//7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
//1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
//9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
//1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
//8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
//1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.

pub fn run() {
    // convert data into a X of Y
    // repeat for each Y
    // push safetyCheck result into Z
    // safetyCheck:
    // - loop thru X until max-1
    // - calculate value of cur index minus (cur + 1)
    // - set the result in a binding called safeRange
    // - abs safeRange > 2 : return unsafe
    // - set a binding called isIncreasing with a case:
    // - - > 0 : true
    // - - = 0 : return unsafe
    // - - < 0 : false
    // - second loop: check (isIncreasing && safeRange < 0) || (!isIncreasing && safeRange > 0): return unsafe
    // - return safe
    // count safe in Z
}
