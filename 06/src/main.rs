use std::{collections::HashSet, fs};

fn main() {
    let packet_marker = 4;
    let message_marker = 14;

    assert!(play("sample1.txt", packet_marker) == 7);
    assert!(play("sample2.txt", packet_marker) == 5);
    assert!(play("sample3.txt", packet_marker) == 6);
    assert!(play("sample4.txt", packet_marker) == 10);
    assert!(play("sample5.txt", packet_marker) == 11);
    println!("{}", play("data.txt", packet_marker));

    assert!(play("sample1.txt", message_marker) == 19);
    assert!(play("sample2.txt", message_marker) == 23);
    assert!(play("sample3.txt", message_marker) == 23);
    assert!(play("sample4.txt", message_marker) == 29);
    assert!(play("sample5.txt", message_marker) == 26);
    println!("{}", play("data.txt", message_marker));
}

fn play(input: &str, marker_length: usize) -> usize {
    let data = fs::read_to_string(input).expect("unable to read files");

    for (index, _ch) in data.char_indices() {
        if index < marker_length {
            continue;
        }

        let sequence = &data[(index - marker_length)..index];

        if sequence.chars().collect::<HashSet<_>>().len() == marker_length {
            return index;
        }
    }

    assert!(false);
    return usize::MAX;
}
