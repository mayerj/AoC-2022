use std::fs;

struct Elf {
    index: i32,
    calories: i32,
}

fn main() {
    let data = fs::read_to_string("src/data.txt").expect("unable to read files");

    let chunks = data.split("\r\n").collect::<Vec<&str>>();

    let mut elves: Vec<Elf> = Vec::new();
    let mut current: i32 = 0;
    let mut current_index: i32 = 0;

    for line in chunks {
        if line.trim().len() == 0 {
            elves.push(Elf {
                index: current_index,
                calories: current,
            });
            current_index += 1;
            current = 0;
            continue;
        }

        current += line.parse::<i32>().expect("not a number?");
    }

    elves.sort_by(|x, y| y.calories.cmp(&x.calories));

    let mut sum: i32 = 0;
    for n in 0..3 {
        let elf = elves.get(n).expect("no elf");
        sum += elf.calories;

        println!("{} - {} out of {}", elf.index, elf.calories, elves.len())
    }

    println!("{}", sum);
}
