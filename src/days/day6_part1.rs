fn str_to_x_y(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

pub fn run() {
    println!("Day6")
    // convert the map into x y
    // map store obsticle
    // create guard tuple/obj w position, direction and steps taken
    // check destination xy in front of direction
    // where direction 0,90,180,270
    // 0 = (0,1), 90 = (1,0), 180 = (-1,0), 270 = (0,-1)
    // if out of bounds, end
    // if obs update +90 direction
    // else, count steps and update position
}

//test case
//....#.....
//.........#
//..........
//..#.......
//.......#..
//..........
//.#..^.....
//........#.
//#.........
//......#..."
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_str_to_room_map() {
        let room_str = "....#.....\n.........#";

        let expected = vec![
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
        ];

        assert_eq!(str_to_x_y(room_str), expected);
    }
}
