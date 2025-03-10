fn str_to_2d_canvas(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn get_obsticle_coordinate(canvas: Vec<Vec<char>>) -> Vec<(i32, i32)> {
    let obsticle: char = '#';
    let mut obsticle_x_y: Vec<(i32, i32)> = vec![];

    for (y, row) in canvas.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == obsticle {
                obsticle_x_y.push((x as i32, y as i32));
            }
        }
    }

    obsticle_x_y
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
    fn test_str_to_2d_canvas() {
        let room_str = "....#.....\n.........#";

        let expected = vec![
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
        ];

        assert_eq!(str_to_2d_canvas(room_str), expected);
    }

    #[test]
    fn test_get_obsticle_coordinate() {
        let two_d_canvas = vec![
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
        ];

        let expected = vec![(4, 0), (9, 1)];

        assert_eq!(get_obsticle_coordinate(two_d_canvas), expected);
    }
}
