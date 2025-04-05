use super::day6_part1::{get_entity_xy_dir, get_item_coordinates};

fn get_visited_canvas(entity: (i32, i32, i32), targets: Vec<(i32, i32)>) -> Vec<(i32, i32, i32)> {
    let (x, y, dir) = entity;
    let mut visited: Vec<(i32, i32, i32)> = Vec::new();

    let mut targets: Vec<(i32, i32)> = targets;
    match dir {
        0 => {
            targets.sort_by(|a, b| a.1.cmp(&b.1));
        }
        90 => {
            targets.sort_by(|a, b| a.0.cmp(&b.0));
        }
        180 => {
            targets.sort_by(|a, b| a.1.cmp(&b.1));
        }
        270 => {
            targets.sort_by(|a, b| a.0.cmp(&b.0));
        }
        _ => {
            println!("Invalid direction: {}", dir);
            return visited;
        }
    }

    for target in targets.iter() {
        let (tx, ty) = *target;
        match dir {
            0 if tx == x && ty < y => {
                for i in (ty..y).skip(1).rev() {
                    visited.push((x, i, dir));
                }
                break;
            }
            90 if tx > x && ty == y => {
                for i in (x..tx).skip(1) {
                    visited.push((i, y, dir));
                }
                break;
            }
            180 if tx == x && ty > y => {
                for i in (y..ty).skip(1) {
                    visited.push((x, i, dir));
                }
                break;
            }
            270 if tx < x && ty == y => {
                for i in (tx..x).skip(1).rev() {
                    visited.push((i, y, dir));
                }
                break;
            }
            _ => {}
        }
    }

    if let Some(last) = visited.last_mut() {
        let (last_x, last_y, _) = *last;
        *last = (last_x, last_y, (dir + 90) % 360);
    }

    visited
}

fn get_unique_visited_xy(visited_canvas: Vec<(i32, i32, i32)>) -> Vec<(i32, i32)> {
    visited_canvas
        .iter()
        .map(|(x, y, _)| (*x, *y))
        .collect::<Vec<(i32, i32)>>()
        .into_iter()
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect::<Vec<(i32, i32)>>()
}

fn get_visited_to_exit(entity: (i32, i32, i32), canvas_size: (i32, i32)) -> Vec<(i32, i32, i32)> {
    let (x, y, dir) = entity;
    let mut visited: Vec<(i32, i32, i32)> = Vec::new();
    let (canvas_x_max, canvas_y_max) = canvas_size;
    match dir {
        0 => {
            for i in 0..y {
                visited.push((x, i, dir));
            }
        }
        90 => {
            for i in (x..canvas_x_max).skip(1) {
                visited.push((i, y, dir));
            }
        }
        180 => {
            for i in (y..canvas_y_max).skip(1) {
                visited.push((x, i, dir));
            }
        }
        270 => {
            for i in 0..x {
                visited.push((i, y, dir));
            }
        }
        _ => {}
    }
    visited
}

pub fn run(canvas: &str) -> i32 {
    // loop through obstacles and compare with entity
    // get visited canvas then change direction
    // update entity xy dir up to the next obsticle
    // loop until out of bounds
    // count visited canvas

    let canvas_vec: Vec<Vec<char>> = canvas.lines().map(|line| line.chars().collect()).collect();
    let mut visited_canvas: Vec<(i32, i32, i32)> = Vec::new();
    let mut entity = get_entity_xy_dir(canvas_vec.clone());
    let obsticle = get_item_coordinates(canvas_vec.clone(), '#');
    let canvas_x_max = canvas_vec[0].len() as i32;
    let canvas_y_max = canvas_vec.len() as i32;

    while entity.0 >= 0 && entity.0 <= canvas_x_max && entity.1 >= 0 && entity.1 <= canvas_y_max {
        let visited = get_visited_canvas(entity, obsticle.clone());
        visited_canvas.extend(&visited);

        if !visited.is_empty() {
            entity = visited.last().unwrap().clone();
        } else {
            let to_exit_path = get_visited_to_exit(entity, (canvas_x_max, canvas_y_max));
            visited_canvas.extend(&to_exit_path);
            break;
        }
    }

    let unique_visited_xy = get_unique_visited_xy(visited_canvas);
    unique_visited_xy.iter().count() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_visited_canvas() {
        let entity = (1, 9, 0); // 0 degree/north
        let targets = vec![(1, 1), (2, 2)];
        let expected = vec![
            (1, 8, 0),
            (1, 7, 0),
            (1, 6, 0),
            (1, 5, 0),
            (1, 4, 0),
            (1, 3, 0),
            (1, 2, 90),
        ];
        assert_eq!(get_visited_canvas(entity, targets), expected);

        let entity = (1, 9, 90); // 90 degree/east
        let targets = vec![(5, 9), (13, 9)];
        let expected = vec![(2, 9, 90), (3, 9, 90), (4, 9, 180)];
        assert_eq!(get_visited_canvas(entity, targets), expected);

        let entity = (1, 9, 180); // 180 degree/south
        let targets = vec![(1, 12), (1, 11)];
        let expected = vec![(1, 10, 270)];
        assert_eq!(get_visited_canvas(entity, targets), expected);

        let entity = (2, 9, 270); // 270 degree/west
        let targets = vec![(0, 9)];
        let expected = vec![(1, 9, 0)];
        assert_eq!(get_visited_canvas(entity, targets), expected);

        let entity = (2, 9, 270); // 270 degree/west
        let targets = vec![(1, 9)];
        let expected = vec![];
        assert_eq!(get_visited_canvas(entity, targets), expected);
    }

    #[test]
    fn test_get_unique_visited_xy() {
        let visited_canvas = vec![(1, 2, 0), (1, 2, 90), (1, 2, 180), (1, 2, 270)];
        let expected = vec![(1, 2)];
        assert_eq!(get_unique_visited_xy(visited_canvas), expected);
    }

    #[test]
    fn test_get_visited_to_exit() {
        let entity = (1, 1, 0);
        let canvas_size = (5, 10);
        let expected = vec![(1, 0, 0)];
        assert_eq!(get_visited_to_exit(entity, canvas_size), expected);

        let entity = (1, 1, 90);
        let canvas_size = (5, 10);
        let expected = vec![(2, 1, 90), (3, 1, 90), (4, 1, 90)];
        assert_eq!(get_visited_to_exit(entity, canvas_size), expected);

        let entity = (1, 1, 180);
        let canvas_size = (5, 10);
        let expected = vec![
            (1, 2, 180),
            (1, 3, 180),
            (1, 4, 180),
            (1, 5, 180),
            (1, 6, 180),
            (1, 7, 180),
            (1, 8, 180),
            (1, 9, 180),
        ];
        assert_eq!(get_visited_to_exit(entity, canvas_size), expected);

        let entity = (1, 1, 270);
        let canvas_size = (5, 10);
        let expected = vec![(0, 1, 270)];
        assert_eq!(get_visited_to_exit(entity, canvas_size), expected);
    }

    #[test]
    fn test_run() {
        let room_str = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";
        let expected = 41;
        assert_eq!(run(room_str), expected);
    }
}
