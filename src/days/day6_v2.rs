fn get_distance_same_line_target(entity: (i32, i32, i32), targets: Vec<(i32, i32)>) -> i32 {
    let (x, y, dir) = entity;
    let mut distance = 0;
    for target in targets.iter() {
        let (tx, ty) = *target;
        if dir == 0 && tx == x && ty < y {
            distance += y - ty;
        } else if dir == 90 && tx > x && ty == y {
            distance += tx - x;
        } else if dir == 180 && tx == x && ty > y {
            distance += ty - y;
        } else if dir == 270 && tx < x && ty == y {
            distance += x - tx;
        }
    }
    distance
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
    fn test_get_distance_same_line_target() {
        let entity = (1, 9, 0); // 0 degree/north
        let targets = vec![(1, 1), (2, 2)];
        assert_eq!(get_distance_same_line_target(entity, targets), 8);

        let entity = (1, 1, 90); // 90 degree/east
        let targets = vec![(8, 1), (2, 2)];
        assert_eq!(get_distance_same_line_target(entity, targets), 7);

        let entity = (1, 1, 180); // 180 degree/south
        let targets = vec![(1, 2), (2, 2)];
        assert_eq!(get_distance_same_line_target(entity, targets), 1);

        let entity = (1, 1, 270); // 270 degree/west
        let targets = vec![(0, 1), (2, 2)];
        assert_eq!(get_distance_same_line_target(entity, targets), 1);
    }
}
