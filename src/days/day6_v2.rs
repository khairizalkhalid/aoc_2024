fn get_visited_canvas(entity: (i32, i32, i32), targets: Vec<(i32, i32)>) -> Vec<(i32, i32, i32)> {
    let (x, y, dir) = entity;
    let mut visited: Vec<(i32, i32, i32)> = Vec::new();

    for target in targets.iter() {
        let (tx, ty) = *target;
        match dir {
            0 if tx == x && ty < y => {
                for i in (ty..=y).rev() {
                    visited.push((x, i, dir));
                }
                break;
            }
            90 if tx > x && ty == y => {
                for i in x..=tx {
                    visited.push((i, y, dir));
                }
                break;
            }
            180 if tx == x && ty > y => {
                for i in y..=ty {
                    visited.push((x, i, dir));
                }
                break;
            }
            270 if tx < x && ty == y => {
                for i in (tx..=x).rev() {
                    visited.push((i, y, dir));
                }
                break;
            }
            _ => {}
        }
    }

    if let Some(last) = visited.last_mut() {
        let (last_x, last_y, _) = *last;
        if targets.contains(&(last_x, last_y)) {
            *last = (last_x, last_y, (dir + 90) % 360);
        }
    }

    visited
}

pub fn run() {
    // recurr
    // if obs found change ntt to tobs -1 +90
    // get distance of ntt to tobs and add in steps
    // no obs on path exit
}

#[cfg(test)]
mod test {
    use super::*;

    //#[test]
    //fn test_main() {
    //    let room_str = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";
    //    assert_eq!(true, true);
    //}

    #[test]
    fn test_get_visited_canvas() {
        let entity = (1, 9, 0); // 0 degree/north
        let targets = vec![(1, 1), (2, 2)];
        let expected = vec![
            (1, 9, 0),
            (1, 8, 0),
            (1, 7, 0),
            (1, 6, 0),
            (1, 5, 0),
            (1, 4, 0),
            (1, 3, 0),
            (1, 2, 0),
            (1, 1, 90),
        ];
        assert_eq!(get_visited_canvas(entity, targets), expected);

        let entity = (1, 9, 90); // 90 degree/east
        let targets = vec![(5, 9), (13, 9)];
        let expected = vec![(1, 9, 90), (2, 9, 90), (3, 9, 90), (4, 9, 90), (5, 9, 180)];
        assert_eq!(get_visited_canvas(entity, targets), expected);

        let entity = (1, 9, 180); // 180 degree/south
        let targets = vec![(1, 10), (1, 11)];
        let expected = vec![(1, 9, 180), (1, 10, 270)];
        assert_eq!(get_visited_canvas(entity, targets), expected);

        let entity = (1, 9, 270); // 270 degree/west
        let targets = vec![(0, 9)];
        let expected = vec![(1, 9, 270), (0, 9, 0)];
        assert_eq!(get_visited_canvas(entity, targets), expected);
    }
}
