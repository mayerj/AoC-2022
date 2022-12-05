use std::fs;

fn main() {
    assert!(play("sample.txt") == 2);
    println!("{}", play("data.txt"));
}

fn play(input: &str) -> usize {
    let data = fs::read_to_string(input).expect("unable to read files");

    let chunks = data.split("\r\n").collect::<Vec<&str>>();

    let pairs = chunks.iter().map(|line| line.split_once(",").unwrap());

    let ranges = pairs.map(|(first, second)| (to_range(first), to_range(second)));

    return ranges
        .filter(|x| is_subset(x.0, x.1) || is_subset(x.1, x.0))
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
