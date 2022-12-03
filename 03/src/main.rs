use std::{collections::HashSet, fs};

fn main() {
    assert!(play("sample.txt") == 157);
    println!("{}", play("data.txt"));
}

fn to_priority(item: &u8) -> Option<u8> {
    return match item {
        97..=122 => Some((item - 97) + 1),
        65..=90 => Some((item - 65) + 27),
        _ => Option::None,
    };
}

fn play(input: &str) -> i32 {
    let data = fs::read_to_string(input).expect("unable to read files");

    let chunks = data.split("\r\n").collect::<Vec<&str>>();

    let rucksacks = chunks
        .iter()
        .map(|pack| Rucksack::new(pack))
        .collect::<Vec<_>>();

    let sum = rucksacks
        .iter()
        .map(|sack| sack.get_duplicate_item())
        .map(|item| to_priority(&item).unwrap() as i32)
        .sum();

    return sum;
}

struct Rucksack {
    pack: String,
    front: Vec<u8>,
    back: Vec<u8>,
}
impl Rucksack {
    pub fn new(pack: &str) -> Self {
        let (front, back) = pack.split_at(pack.len() / 2);

        Self {
            pack: pack.to_string(),
            front: Vec::from(front),
            back: Vec::from(back),
        }
    }

    pub fn get_duplicate_item(self: &Self) -> u8 {
        let front_set: HashSet<&u8> = HashSet::from_iter(self.front.iter());
        let back_set: HashSet<&u8> = HashSet::from_iter(self.back.iter());

        let intersection: Vec<&&u8> = front_set.intersection(&back_set).collect();

        assert!(intersection.len() == 1);

        return ***intersection.get(0).unwrap();
    }
}
