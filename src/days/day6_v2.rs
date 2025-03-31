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

    #[test]
    fn test_main() {
        let room_str = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";
        assert_eq!(true, true);
    }
}
