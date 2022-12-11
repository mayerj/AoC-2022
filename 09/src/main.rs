use std::{collections::HashSet, fmt::Debug, fs};

use rope::{LongRope, Point, Rope};

use crate::rope::Ropelike;
mod rope;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    assert_eq!(
        Rope::new_with_tail((0, 0), (0, -1)).move_head('U'),
        Rope::new_with_tail((0, 1), (0, 0))
    );
    assert_eq!(Point::new(4, 1).manhattan(Point::new(3, 0)), 2u32);
    assert!(Point::new(4, 0).is_touching(Point::new(3, 0)));
    assert!(Point::new(4, 1).is_touching(Point::new(3, 0)));
    assert_eq!(
        Rope::new_with_tail((4, 0), (3, 0)).move_head('U'),
        Rope::new_with_tail((4, 1), (3, 0))
    );

    assert_eq!(play("sample.txt"), 13);
    println!("{:?}", play("data.txt"));
    assert_eq!(play2("sample.txt"), 1);
    assert_eq!(play2("sample2.txt"), 36);
    println!("{:?}", play2("data.txt"));
}

fn play(input: &str) -> usize {
    return play_internal::<Rope>(input);
}
fn play2(input: &str) -> usize {
    return play_internal::<LongRope>(input);
}

fn play_internal<T: Ropelike + Debug>(input: &str) -> usize {
    let data = fs::read_to_string(input).expect("unable to read files");

    let mut states: Vec<T> = vec![T::new(0, 0)];
    for line in data.lines() {
        let (direction, count) = parse_line(line);

        println!("{} - {}", direction, count);
        for _i in 0..count {
            let state: &T = states.get(states.len() - 1).unwrap();
            let new_state = state.move_head(direction);

            log::debug!("{:?} to {:?}", state, new_state);
            states.push(new_state)
        }
    }

    for state in &states {
        println!("{:?}", state);
    }

    let mut locations: HashSet<Point> = HashSet::new();
    for state in states {
        locations.insert(state.get_tail().clone());
    }

    return locations.len();
}

fn parse_line(line: &str) -> (char, usize) {
    return line
        .split_once(' ')
        .map(|(direction, count_str)| {
            return (
                direction.chars().nth(0).unwrap(),
                count_str.parse().unwrap(),
            );
        })
        .unwrap();
}
