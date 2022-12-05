use std::fs;

fn main() {
    assert!(play("sample.txt", &is_subset) == 2);
    println!("{}", play("data.txt", &is_subset));
    assert!(play("sample.txt", &is_overlap) == 4);
    println!("{}", play("data.txt", &is_overlap));
}

fn play(input: &str, predicate: &dyn Fn((i32, i32), (i32, i32)) -> bool) -> usize {
    let data = fs::read_to_string(input).expect("unable to read files");

    let chunks = data.split("\r\n").collect::<Vec<&str>>();

    let pairs = chunks.iter().map(|line| line.split_once(",").unwrap());

    let ranges = pairs.map(|(first, second)| (to_range(first), to_range(second)));

    return ranges
        .filter(|x| predicate(x.0, x.1) || predicate(x.1, x.0))
        .count();
}

fn to_range(input: &str) -> (i32, i32) {
    return input
        .split_once("-")
        .map(|(first, second)| (first.parse().unwrap(), second.parse().unwrap()))
        .unwrap();
}

fn is_subset(first: (i32, i32), second: (i32, i32)) -> bool {
    return first.0 <= second.0 && first.1 >= second.1;
}

fn is_overlap(first: (i32, i32), second: (i32, i32)) -> bool {
    let range = first.0..=first.1;

    return range.contains(&second.0) || range.contains(&second.1);
}
