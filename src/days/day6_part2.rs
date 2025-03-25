use crate::days::day6_part1::{
    get_entity_xy_dir, is_front_clear, is_front_out_of_bounds, mark_visited, move_forward,
    str_to_2d_canvas, turn_right,
};

fn is_front_looping(entity_xy_dir: (i32, i32, i32), items: &[(i32, i32, i32)]) -> bool {
    let (ntt_x, ntt_y, ntt_dir) = entity_xy_dir;

    match ntt_dir {
        0 => {
            let front_x = ntt_x;
            let front_y = ntt_y - 1;
            if items.contains(&(front_x, front_y, ntt_dir)) {
                return true;
            }
            return false;
        }
        90 => {
            let front_x = ntt_x + 1;
            let front_y = ntt_y;
            if items.contains(&(front_x, front_y, ntt_dir)) {
                return true;
            }
            return false;
        }
        180 => {
            let front_x = ntt_x;
            let front_y = ntt_y + 1;
            if items.contains(&(front_x, front_y, ntt_dir)) {
                return true;
            }
            return false;
        }
        270 => {
            let front_x = ntt_x - 1;
            let front_y = ntt_y;
            if items.contains(&(front_x, front_y, ntt_dir)) {
                return true;
            }
            return false;
        }
        _ => false,
    }
}

use super::day6_part1::get_item_coordinates;

pub fn test_run() -> i32 {
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

    let marked = get_item_coordinates(canvas.clone(), 'X');
    let canvas2 = str_to_2d_canvas(room_str);
    let mut looping_marker: Vec<(i32, i32)> = vec![];

    for m in marked {
        let mut entity2 = get_entity_xy_dir(canvas2.clone());
        let mut visited_with_direction: Vec<(i32, i32, i32)> = vec![];
        let mut new_obs = obsticle.clone();
        new_obs.push(m);
        loop {
            if is_front_looping(entity2, &visited_with_direction) {
                looping_marker.push(m);
                break;
            } else if !is_front_clear(entity2, new_obs.clone()) {
                entity2 = turn_right(entity2);
            } else if is_front_out_of_bounds(entity2, canvas2.clone()) {
                break;
            } else {
                visited_with_direction.push(entity2);
                entity2 = move_forward(entity2);
            }
        }
    }

    looping_marker.iter().count() as i32
}

pub fn run() {
    println!("Day Six Part Two.");
    // for each of the mark visitted, add into a coordinate
    // add new obsticle 'O' in the canvas
    // move the entity
    // use new visitted marker '+/-/|' (now need to enhance with direction too)
    // check for is_front_loop where it is checking for the visitted marker with entity direction
    // if it is a loop, store this new obsticle xy else continue foreach mark visited
    // finally count new obsticle created
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_front_looping() {
        let entity = (4, 1, 0);
        let items = vec![(4, 0, 0), (9, 1, 0)];

        assert_eq!(is_front_looping(entity, &items), true);

        let entity = (4, 1, 90);
        let items = vec![(5, 1, 90), (9, 1, 0)];

        assert_eq!(is_front_looping(entity, &items), true);

        let entity = (4, 1, 180);
        let items = vec![(4, 2, 180), (9, 1, 0)];

        assert_eq!(is_front_looping(entity, &items), true);

        let entity = (4, 1, 270);
        let items = vec![(3, 1, 270), (9, 1, 0)];

        assert_eq!(is_front_looping(entity, &items), true);
    }

    #[test]
    fn test_test_run() {
        assert_eq!(test_run(), 6);
    }
}
