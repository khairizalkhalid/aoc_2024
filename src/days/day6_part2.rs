use std::io::{self, Write};

use crate::{
    days::day6_part1::{
        get_entity_xy_dir, get_item_coordinates, get_visited_canvas, is_front_clear,
        is_front_out_of_bounds, move_forward, str_to_2d_canvas, turn_right,
    },
    utils,
};

fn is_looping(entity_xy_dir: (i32, i32, i32), items: &[(i32, i32, i32)]) -> bool {
    items.contains(&entity_xy_dir)
}

#[allow(dead_code)]
pub fn test_run() -> i32 {
    let room_str = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";

    let visited_canvas = get_visited_canvas(room_str.to_string());
    let marked = get_item_coordinates(visited_canvas.clone(), 'X');
    let canvas = str_to_2d_canvas(room_str);
    let obsticle = get_item_coordinates(canvas.clone(), '#');
    let mut looping_marker: Vec<(i32, i32)> = vec![];
    let starting_point = get_entity_xy_dir(canvas.clone());
    let (x, y, _) = starting_point;

    for m in marked {
        if m == (x, y) {
            continue;
        }
        let mut entity = starting_point;
        let mut visited_with_direction: Vec<(i32, i32, i32)> = vec![];
        let mut new_obs = obsticle.clone();
        new_obs.push(m);
        loop {
            if is_looping(entity, &visited_with_direction) {
                looping_marker.push(m);
                break;
            } else if !is_front_clear(entity, new_obs.clone()) {
                entity = turn_right(entity);
            } else if is_front_out_of_bounds(entity, canvas.clone()) {
                break;
            } else {
                visited_with_direction.push(entity);
                entity = move_forward(entity);
            }
        }
    }

    looping_marker.iter().count() as i32
}

pub fn run() {
    // for each of the mark visitted, add into a coordinate
    // add new obsticle 'O' in the canvas
    // move the entity
    // use new visitted marker '+/-/|' (now need to enhance with direction too)
    // check for is_front_loop where it is checking for the visitted marker with entity direction
    // if it is a loop, store this new obsticle xy else continue foreach mark visited
    // finally count new obsticle created
    match utils::file_reader::read_file("day6.txt") {
        Ok(contents) => {
            let visited_canvas = get_visited_canvas(contents.clone());
            let marked = get_item_coordinates(visited_canvas.clone(), 'X');
            let canvas = str_to_2d_canvas(&contents);
            let obsticle = get_item_coordinates(canvas.clone(), '#');
            let mut count_marker = 0;
            let starting_point = get_entity_xy_dir(canvas.clone());
            let (x, y, _) = starting_point;

            for (i, &m) in marked.iter().enumerate() {
                if m == (x, y) {
                    continue;
                }
                let mut entity = starting_point;
                let mut visited_with_direction: Vec<(i32, i32, i32)> = vec![];
                let mut new_obs = obsticle.clone();
                new_obs.push(m);
                loop {
                    print!("\r\x1b[2Kentity at {:?}", entity);
                    print!("\x1b[1A"); // Move cursor up one line
                    print!(
                        "\r\x1b[2Krunning {} out of {} marked... found {} ",
                        i,
                        marked.iter().len(),
                        count_marker
                    );
                    io::stdout().flush().unwrap();
                    print!("\x1b[1B"); // Move cursor down one line

                    if is_looping(entity, &visited_with_direction) {
                        count_marker += 1;
                        break;
                    } else if !is_front_clear(entity, new_obs.clone()) {
                        entity = turn_right(entity);
                    } else if is_front_out_of_bounds(entity, canvas.clone()) {
                        break;
                    } else {
                        visited_with_direction.push(entity);
                        entity = move_forward(entity);
                    }
                }
            }

            println!("looping count {}", count_marker);
        }
        Err(e) => println!("Err: {}", e),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_front_looping() {
        let entity = (4, 1, 0);
        let items = vec![(4, 1, 0), (9, 1, 0)];

        assert_eq!(is_looping(entity, &items), true);

        let entity = (4, 1, 90);
        let items = vec![(4, 1, 90), (9, 1, 0)];

        assert_eq!(is_looping(entity, &items), true);

        let entity = (4, 1, 180);
        let items = vec![(4, 1, 180), (9, 1, 0)];

        assert_eq!(is_looping(entity, &items), true);

        let entity = (4, 1, 270);
        let items = vec![(4, 1, 270), (9, 1, 0)];

        assert_eq!(is_looping(entity, &items), true);
    }

    #[test]
    fn test_test_run() {
        assert_eq!(test_run(), 6);
    }
}
