use std::io::{self, Write};

use crate::utils;

use super::day6_part1::{get_entity_xy_dir, get_item_coordinates};

fn get_visited_canvas(entity: (i32, i32, i32), targets: Vec<(i32, i32)>) -> Vec<(i32, i32, i32)> {
    let (x, y, dir) = entity;
    let mut visited = Vec::new();

    let closest_target = targets
        .into_iter()
        .filter(|&(tx, ty)| match dir {
            0 => tx == x && ty < y,   // Up
            90 => ty == y && tx > x,  // Right
            180 => tx == x && ty > y, // Down
            270 => ty == y && tx < x, // Left
            _ => false,
        })
        .min_by_key(|&(tx, ty)| match dir {
            0 => y - ty,   // Distance for Up
            90 => tx - x,  // Distance for Right
            180 => ty - y, // Distance for Down
            270 => x - tx, // Distance for Left
            _ => i32::MAX,
        });

    if let Some((tx, ty)) = closest_target {
        match dir {
            0 => visited.extend((ty + 1..y).rev().map(|i| (x, i, dir))), // Up
            90 => visited.extend((x + 1..tx).map(|i| (i, y, dir))),      // Right
            180 => visited.extend((y + 1..ty).map(|i| (x, i, dir))),     // Down
            270 => visited.extend((tx + 1..x).rev().map(|i| (i, y, dir))), // Left
            _ => {}
        }

        if let Some(last) = visited.last_mut() {
            last.2 = (dir + 90) % 360;
        }
    }

    visited
}

fn get_unique_visited_xy(visited_canvas: Vec<(i32, i32, i32)>) -> Vec<(i32, i32)> {
    visited_canvas
        .into_iter()
        .map(|(x, y, _)| (x, y))
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect()
}

fn get_visited_to_exit(entity: (i32, i32, i32), canvas_size: (i32, i32)) -> Vec<(i32, i32, i32)> {
    let (x, y, dir) = entity;
    let (canvas_x_max, canvas_y_max) = canvas_size;

    let visited = match dir {
        0 => (0..y).rev().map(|i| (x, i, dir)).collect(), // Up
        90 => ((x + 1)..canvas_x_max).map(|i| (i, y, dir)).collect(), // Right
        180 => ((y + 1)..canvas_y_max).map(|i| (x, i, dir)).collect(), // Down
        270 => (0..x).rev().map(|i| (i, y, dir)).collect(), // Left
        _ => Vec::new(),
    };

    visited
}

fn part1_vec(canvas: &str) -> Vec<(i32, i32)> {
    let canvas_vec: Vec<Vec<char>> = canvas.lines().map(|line| line.chars().collect()).collect();
    let mut visited_canvas: Vec<(i32, i32, i32)> = Vec::new();
    let mut entity = get_entity_xy_dir(canvas_vec.clone());
    let obsticle = get_item_coordinates(canvas_vec.clone(), '#');
    let canvas_x_max = canvas_vec[0].len() as i32;
    let canvas_y_max = canvas_vec.len() as i32;

    while entity.0 >= 0 && entity.0 <= canvas_x_max && entity.1 >= 0 && entity.1 <= canvas_y_max {
        let visited = get_visited_canvas(entity, obsticle.clone());

        if !visited.is_empty() {
            visited_canvas.extend(&visited);
            entity = visited.last().unwrap().clone();
        } else {
            let to_exit_path = get_visited_to_exit(entity, (canvas_x_max, canvas_y_max));
            visited_canvas.extend(&to_exit_path);
            break;
        }
    }

    get_unique_visited_xy(visited_canvas)
}

fn part1(canvas: &str) -> i32 {
    // loop through obstacles and compare with entity
    // get visited canvas then change direction
    // update entity xy dir up to the next obsticle
    // loop until out of bounds
    // count visited canvas
    part1_vec(canvas).iter().count() as i32
}

fn part2(canvas: &str) -> i32 {
    // add part 2 here...
    // for each of the unique_visited in part 1
    // add it as obsticle
    // get visited_canvas
    // when hit is_looping (new fn), loop_count++, break
    // get loop_count
    let unique_visited_xy = part1_vec(canvas);

    let mut loop_count = 0;

    for (uv_no, &uv) in unique_visited_xy.iter().enumerate() {
        print!("\x1b[1A"); // Move cursor up one line
        print!("\r\x1b[2KProgress: {}/{}", uv_no, unique_visited_xy.len());
        io::stdout().flush().unwrap();
        print!("\x1b[1B"); // Move cursor down one line

        let canvas_vec: Vec<Vec<char>> =
            canvas.lines().map(|line| line.chars().collect()).collect();
        let mut obsticle = get_item_coordinates(canvas_vec.clone(), '#');
        obsticle.push(uv);
        let mut visited_canvas: Vec<(i32, i32, i32)> = Vec::new();
        let mut entity = get_entity_xy_dir(canvas_vec.clone());
        let canvas_x_max = canvas_vec[0].len() as i32;
        let canvas_y_max = canvas_vec.len() as i32;

        while entity.0 >= 0 && entity.0 <= canvas_x_max && entity.1 >= 0 && entity.1 <= canvas_y_max
        {
            let visited = get_visited_canvas(entity, obsticle.clone());

            if !visited.is_empty() {
                if visited_canvas.iter().any(|&v| visited.contains(&v)) {
                    loop_count += 1;
                    break;
                }

                visited_canvas.extend(&visited);
                entity = visited.last().unwrap().clone();
            } else {
                let to_exit_path = get_visited_to_exit(entity, (canvas_x_max, canvas_y_max));
                visited_canvas.extend(&to_exit_path);
                break;
            }
        }
    }

    loop_count
}

pub fn run(option: i8) {
    match utils::file_reader::read_file("day6.txt") {
        Ok(contents) => {
            if option == 1 {
                let canvas = part1(&contents);
                println!("Visited canvas: {}", canvas);
            } else if option == 2 {
                let canvas = part2(&contents);
                println!("Visited canvas: {}", canvas);
            } else {
                println!("Unknown option used, expected 1 or 2");
            }
        }
        Err(e) => println!("Err: {}", e),
    }
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
        assert_eq!(part1(room_str), expected);
        assert_eq!(part2(room_str), 6);
    }
}
