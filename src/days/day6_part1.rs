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

fn is_front_clear(entity_xy_dir: (i32, i32, i32), obstacles: Vec<(i32, i32)>) -> bool {
    let (ntt_x, ntt_y, ntt_dir) = entity_xy_dir;

    match ntt_dir {
        0 => {
            let front_x = ntt_x;
            let front_y = ntt_y - 1;
            if obstacles.contains(&(front_x, front_y)) {
                return false;
            }
            return true;
        }
        90 => {
            let front_x = ntt_x + 1;
            let front_y = ntt_y;
            if obstacles.contains(&(front_x, front_y)) {
                return false;
            }
            return true;
        }
        180 => {
            let front_x = ntt_x;
            let front_y = ntt_y + 1;
            if obstacles.contains(&(front_x, front_y)) {
                return false;
            }
            return true;
        }
        270 => {
            let front_x = ntt_x - 1;
            let front_y = ntt_y;
            if obstacles.contains(&(front_x, front_y)) {
                return false;
            }
            return true;
        }
        _ => false,
    }
}

fn move_forward(entity_xy_dir: (i32, i32, i32)) -> (i32, i32, i32) {
    let (ntt_x, ntt_y, ntt_dir) = entity_xy_dir;

    match ntt_dir {
        0 => (ntt_x, ntt_y - 1, ntt_dir),
        90 => (ntt_x + 1, ntt_y, ntt_dir),
        180 => (ntt_x, ntt_y + 1, ntt_dir),
        270 => (ntt_x - 1, ntt_y, ntt_dir),
        _ => (ntt_x, ntt_y, ntt_dir),
    }
}

pub fn run() {
    println!("Day6")
    // convert the map into x y
    // map store obsticle
    // create entity tuple/obj w position, direction and steps taken
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

    #[test]
    fn test_is_front_clear() {
        // direction 0
        let entity = (4, 1, 0);
        let obstacles = vec![(5, 0), (9, 1)];

        assert_eq!(is_front_clear(entity, obstacles), true);

        // direction 90
        let entity = (4, 1, 90);
        let obstacles = vec![(5, 0), (9, 1)];

        assert_eq!(is_front_clear(entity, obstacles), true);

        // direction 180
        let entity = (4, 1, 180);
        let obstacles = vec![(5, 0), (9, 1)];

        assert_eq!(is_front_clear(entity, obstacles), true);

        // direction 270
        let entity = (4, 1, 270);
        let obstacles = vec![(5, 0), (9, 1)];

        assert_eq!(is_front_clear(entity, obstacles), true);
    }

    #[test]
    fn test_is_not_front_clear() {
        // direction 0
        let entity = (4, 1, 0);
        let obstacles = vec![(4, 0), (9, 1)];

        assert_eq!(is_front_clear(entity, obstacles), false);

        // direction 90
        let entity = (4, 1, 90);
        let obstacles = vec![(5, 1), (9, 1)];

        assert_eq!(is_front_clear(entity, obstacles), false);

        // direction 180
        let entity = (4, 1, 180);
        let obstacles = vec![(4, 2), (9, 1)];

        assert_eq!(is_front_clear(entity, obstacles), false);

        // direction 270
        let entity = (4, 1, 270);
        let obstacles = vec![(3, 1), (9, 1)];

        assert_eq!(is_front_clear(entity, obstacles), false);
    }

    #[test]
    fn test_move_forward() {
        // direction 0
        let entity = (4, 1, 0);

        assert_eq!(move_forward(entity), (4, 0, 0));

        // direction 90
        let entity = (4, 1, 90);

        assert_eq!(move_forward(entity), (5, 1, 90));

        // direction 180
        let entity = (4, 1, 180);

        assert_eq!(move_forward(entity), (4, 2, 180));

        // direction 270
        let entity = (4, 1, 270);

        assert_eq!(move_forward(entity), (3, 1, 270));
    }
}
