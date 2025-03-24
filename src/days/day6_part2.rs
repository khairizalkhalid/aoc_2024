
fn is_front_looping(entity_xy_dir: (i32, i32, i32), items: Vec<(i32, i32, char)>) -> bool {
    let (ntt_x, ntt_y, ntt_dir) = entity_xy_dir;

    match ntt_dir {
        0 => {
            let front_x = ntt_x;
            let front_y = ntt_y - 1;
            if items.contains(&(front_x, front_y, '|')) {
                return true;
            }
            return false;
        }
        90 => {
            let front_x = ntt_x + 1;
            let front_y = ntt_y;
            if items.contains(&(front_x, front_y, '-')) {
                return true;
            }
            return false;
        }
        180 => {
            let front_x = ntt_x;
            let front_y = ntt_y + 1;
            if items.contains(&(front_x, front_y, '|')) {
                return true;
            }
            return false;
        }
        270 => {
            let front_x = ntt_x - 1;
            let front_y = ntt_y;
            if items.contains(&(front_x, front_y, '-')) {
                return true;
            }
            return false;
        }
        _ => false,
    }
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
        let items = vec![(4, 0, '|'), (9, 1, '|')];

        assert_eq!(is_front_looping(entity, items), true);

        let entity = (4, 1, 90);
        let items = vec![(5, 1, '-'), (9, 1, '|')];

        assert_eq!(is_front_looping(entity, items), true);

        let entity = (4, 1, 180);
        let items = vec![(4, 2, '|'), (9, 1, '|')];

        assert_eq!(is_front_looping(entity, items), true);

        let entity = (4, 1, 270);
        let items = vec![(3, 1, '-'), (9, 1, '-')];

        assert_eq!(is_front_looping(entity, items), true);
    }
}
