use std::{collections::HashSet, fs};

fn main() {
    assert!(play("sample1.txt") == 7);
    assert!(play("sample2.txt") == 5);
    assert!(play("sample3.txt") == 6);
    assert!(play("sample4.txt") == 10);
    assert!(play("sample5.txt") == 11);
    println!("{}", play("data.txt"));
}

fn play(input: &str) -> usize {
    let data = fs::read_to_string(input).expect("unable to read files");

    for (index, _ch) in data.char_indices() {
        if index < 4 {
            continue;
        }

        let sequence = &data[(index - 4)..index];

        if sequence.chars().collect::<HashSet<_>>().len() == 4 {
            return index;
        }
    }

    assert!(false);
    return usize::MAX;
}
