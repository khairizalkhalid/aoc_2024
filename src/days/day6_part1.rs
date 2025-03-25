use crate::utils;

pub fn str_to_2d_canvas(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

pub fn get_item_coordinates(canvas: Vec<Vec<char>>, item: char) -> Vec<(i32, i32)> {
    let mut obsticle_x_y: Vec<(i32, i32)> = vec![];

    for (y, row) in canvas.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == item {
                obsticle_x_y.push((x as i32, y as i32));
            }
        }
    }

    obsticle_x_y
}

pub fn get_entity_xy_dir(canvas: Vec<Vec<char>>) -> (i32, i32, i32) {
    let entity_form_dir = vec![('^', 0), ('>', 90), ('v', 180), ('<', 270)];

    for (y, row) in canvas.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            for (dir, dir_val) in entity_form_dir.iter() {
                if c == *dir {
                    return (x as i32, y as i32, *dir_val);
                }
            }
        }
    }

    (0, 0, 0)
}

pub fn is_front_clear(entity_xy_dir: (i32, i32, i32), obstacles: Vec<(i32, i32)>) -> bool {
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

pub fn is_front_out_of_bounds(entity_xy_dir: (i32, i32, i32), canvas: Vec<Vec<char>>) -> bool {
    let (ntt_x, ntt_y, ntt_dir) = entity_xy_dir;

    match ntt_dir {
        0 => {
            let front_y = ntt_y - 1;
            if front_y < 0 {
                return true;
            }
            return false;
        }
        90 => {
            let front_x = ntt_x + 1;
            if front_x >= canvas[0].len() as i32 {
                return true;
            }
            return false;
        }
        180 => {
            let front_y = ntt_y + 1;
            if front_y >= canvas.len() as i32 {
                return true;
            }
            return false;
        }
        270 => {
            let front_x = ntt_x - 1;
            if front_x < 0 {
                return true;
            }
            return false;
        }
        _ => false,
    }
}

pub fn mark_visited(canvas: Vec<Vec<char>>, entity_xy_dir: (i32, i32, i32)) -> Vec<Vec<char>> {
    let (ntt_x, ntt_y, _ntt_dir) = entity_xy_dir;

    canvas
        .into_iter()
        .enumerate()
        .map(|(row_i, row)| {
            if row_i == ntt_y as usize {
                row.into_iter()
                    .enumerate()
                    .map(
                        |(col_i, col)| {
                            if col_i == ntt_x as usize {
                                'X'
                            } else {
                                col
                            }
                        },
                    )
                    .collect()
            } else {
                row
            }
        })
        .collect()
}

pub fn move_forward(entity_xy_dir: (i32, i32, i32)) -> (i32, i32, i32) {
    let (ntt_x, ntt_y, ntt_dir) = entity_xy_dir;

    match ntt_dir {
        0 => (ntt_x, ntt_y - 1, ntt_dir),
        90 => (ntt_x + 1, ntt_y, ntt_dir),
        180 => (ntt_x, ntt_y + 1, ntt_dir),
        270 => (ntt_x - 1, ntt_y, ntt_dir),
        _ => (ntt_x, ntt_y, ntt_dir),
    }
}

pub fn turn_right(entity_xy_dir: (i32, i32, i32)) -> (i32, i32, i32) {
    let (ntt_x, ntt_y, ntt_dir) = entity_xy_dir;

    if ntt_dir == 270 {
        return (ntt_x, ntt_y, 0);
    }

    (ntt_x, ntt_y, ntt_dir + 90)
}

fn count_visited(canvas: Vec<Vec<char>>) -> i32 {
    count_mark(canvas, 'X')
}

pub fn count_mark(canvas: Vec<Vec<char>>, mark: char) -> i32 {
    canvas
        .iter()
        .map(|r| r.iter().filter(|&&c| c == mark).count() as i32)
        .sum()
}

#[allow(dead_code)]
fn test_run() -> i32 {
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

    let room_str = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";

    let mut canvas = str_to_2d_canvas(room_str);
    let obsticle = get_item_coordinates(canvas.clone(), '#');
    let mut entity = get_entity_xy_dir(canvas.clone());

    loop {
        if !is_front_clear(entity, obsticle.clone()) {
            entity = turn_right(entity);
        } else if is_front_out_of_bounds(entity, canvas.clone()) {
            break;
        } else {
            entity = move_forward(entity);
            canvas = mark_visited(canvas, entity);
        }
    }

    count_visited(canvas)
}

pub fn get_visited_canvas(contents: String) -> Vec<Vec<char>> {
    let mut canvas = str_to_2d_canvas(&contents);
    let obsticle = get_item_coordinates(canvas.clone(), '#');
    let mut entity = get_entity_xy_dir(canvas.clone());

    loop {
        if !is_front_clear(entity, obsticle.clone()) {
            entity = turn_right(entity);
        } else if is_front_out_of_bounds(entity, canvas.clone()) {
            break;
        } else {
            entity = move_forward(entity);
            canvas = mark_visited(canvas, entity);
        }
    }
    canvas
}

pub fn run() {
    // convert the map into x y
    // map store obsticle
    // create entity tuple/obj w position, direction and steps taken
    // check destination xy in front of direction
    // where direction 0,90,180,270
    // 0 = (0,1), 90 = (1,0), 180 = (-1,0), 270 = (0,-1)
    // if out of bounds, end
    // if obs update +90 direction
    // else, count steps and update position
    match utils::file_reader::read_file("day6.txt") {
        Ok(contents) => {
            let canvas = get_visited_canvas(contents);
            println!("Visited: {}", count_visited(canvas));
        }
        Err(e) => println!("Err: {}", e),
    }
}

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

        assert_eq!(get_item_coordinates(two_d_canvas, '#'), expected);
    }

    #[test]
    fn test_get_entity_xy_dir() {
        let two_d_canvas = vec![
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '^', '.', '.', '.', '.', '#'],
        ];

        let expected = (4, 1, 0);

        assert_eq!(get_entity_xy_dir(two_d_canvas), expected);

        let two_d_canvas = vec![
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '>', '.', '.', '.', '.', '#'],
        ];

        let expected = (4, 1, 90);

        assert_eq!(get_entity_xy_dir(two_d_canvas), expected);

        let two_d_canvas = vec![
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', 'v', '.', '.', '.', '.', '#'],
        ];

        let expected = (4, 1, 180);

        assert_eq!(get_entity_xy_dir(two_d_canvas), expected);

        let two_d_canvas = vec![
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '<', '.', '.', '.', '.', '#'],
        ];

        let expected = (4, 1, 270);

        assert_eq!(get_entity_xy_dir(two_d_canvas), expected);
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
    fn test_is_front_out_of_bounds() {
        // direction 0
        let entity = (4, 0, 0);
        let canvas = vec![
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
        ];

        assert_eq!(is_front_out_of_bounds(entity, canvas), true);

        // direction 90
        let entity = (9, 0, 90);
        let canvas = vec![
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
        ];

        assert_eq!(is_front_out_of_bounds(entity, canvas), true);

        // direction 180
        let entity = (4, 1, 180);
        let canvas = vec![
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
        ];

        assert_eq!(is_front_out_of_bounds(entity, canvas), true);

        // direction 270
        let entity = (0, 1, 270);
        let canvas = vec![
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
        ];

        assert_eq!(is_front_out_of_bounds(entity, canvas), true);
    }

    #[test]
    fn test_is_not_front_out_of_bounds() {
        // direction 0
        let entity = (4, 1, 0);
        let canvas = vec![
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
        ];

        assert_eq!(is_front_out_of_bounds(entity, canvas), false);

        // direction 90
        let entity = (8, 1, 90);
        let canvas = vec![
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
        ];

        assert_eq!(is_front_out_of_bounds(entity, canvas), false);

        // direction 180
        let entity = (4, 0, 180);
        let canvas = vec![
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
        ];

        assert_eq!(is_front_out_of_bounds(entity, canvas), false);

        // direction 270
        let entity = (9, 1, 270);
        let canvas = vec![
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
        ];

        assert_eq!(is_front_out_of_bounds(entity, canvas), false);
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

    #[test]
    fn test_turn_right() {
        // direction 0
        let entity = (4, 1, 0);

        assert_eq!(turn_right(entity), (4, 1, 90));

        // direction 90
        let entity = (4, 1, 90);

        assert_eq!(turn_right(entity), (4, 1, 180));

        // direction 180
        let entity = (4, 1, 180);

        assert_eq!(turn_right(entity), (4, 1, 270));

        // direction 270
        let entity = (4, 1, 270);

        assert_eq!(turn_right(entity), (4, 1, 0));
    }

    #[test]
    fn test_mark_visited() {
        let two_d_canvas = vec![
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
        ];

        let entity = (4, 1, 0);

        let expected = vec![
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', 'X', '.', '.', '.', '.', '#'],
        ];

        assert_eq!(mark_visited(two_d_canvas, entity), expected);
    }

    #[test]
    fn test_count_visited() {
        let two_d_canvas = vec![
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', 'X', '.', '.', '.', '.', '#'],
        ];

        assert_eq!(count_visited(two_d_canvas), 1);

        let two_d_canvas = vec![
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', 'X', '.', '.', '.', '.', '#'],
            vec!['.', '.', '.', '.', 'X', '.', '.', '.', '.', '#'],
        ];

        assert_eq!(count_visited(two_d_canvas), 2);

        let two_d_canvas = vec![
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', 'X', 'X', 'X', 'x', 'x', '#'],
            vec!['.', '.', '.', '.', 'X', '.', '.', '.', '.', '#'],
        ];

        assert_eq!(count_visited(two_d_canvas), 4);
    }

    #[test]
    fn test_test_run() {
        assert_eq!(test_run(), 41);
    }

    #[test]
    fn test_get_visited_canvas() {
        let room_str = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";

        let expected = str_to_2d_canvas("....#.....\n....XXXXX#\n....X...X.\n..#.X...X.\n..XXXXX#X.\n..X.X.X.X.\n.#XXXXXXX.\n.XXXXXXX#.\n#XXXXXXX..\n......#X..");

        assert_eq!(get_visited_canvas(room_str.to_string()), expected);
    }
}
