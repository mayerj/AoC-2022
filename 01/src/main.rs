use std::fs;

fn main() {
    let data = fs::read_to_string("src/data.txt").expect("unable to read files");

    let chunks = data.split("\r\n").collect::<Vec<&str>>();

    let mut max: i32 = 0;
    let mut max_index: i32 = 0;
    let mut current: i32 = 0;
    let mut current_index: i32 = 0;

    for line in chunks {
        if line.trim().len() == 0 {
            if current > max {
                max = current;
                max_index = current_index;
            }
            current_index += 1;
            current = 0;
            continue;
        }

        current += line.parse::<i32>().expect("not a number?");
    }

    print!("{} - {} out of {}", max_index, max, current_index)
}
