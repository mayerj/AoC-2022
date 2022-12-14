use std::{collections::HashSet, fs};

use cpu::Cpu;
mod cpu;

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();

    assert_eq!(play("sample.txt", &HashSet::from([7])), -7);
    assert_eq!(
        play(
            "sample2.txt",
            &HashSet::from([20usize, 60, 100, 140, 180, 220])
        ),
        13140
    );
    println!(
        "{:?}",
        play(
            "data.txt",
            &HashSet::from([20usize, 60, 100, 140, 180, 220])
        )
    );
    assert_eq!(
        play2("sample2.txt"),
        "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
    );
    println!("{:?}", play2("data.txt"));
}

fn play(input: &str, cycles_to_sum: &HashSet<usize>) -> isize {
    let data = fs::read_to_string(input).expect("unable to read files");

    let mut cpu: Cpu = Cpu::load_from_lines(data.lines());

    let mut sum: isize = 0;
    for cycle_count in 0..=*cycles_to_sum.iter().max().unwrap() + 1 {
        if cycles_to_sum.contains(&(cycle_count + 1)) {
            log::debug!(
                "adding to sum (cycle {}) {}",
                cycle_count,
                cycle_count as isize * cpu.registers.x
            );
            sum += (cycle_count as isize + 1) * cpu.registers.x;
        }

        cpu.run_cycle();
    }

    return sum;
}

fn play2(input: &str) -> String {
    let data = fs::read_to_string(input).expect("unable to read files");

    let mut cpu: Cpu = Cpu::load_from_lines(data.lines());

    let mut image: Vec<String> = Vec::new();
    let mut line: Vec<char> = Vec::new();
    for cycle_count in 0..40 * 6 {
        let cycle: isize = cycle_count % 40;

        let sprite_pos = cpu.registers.x % 40;

        let sprite_range = sprite_pos - 1..=sprite_pos + 1;
        if sprite_range.contains(&cycle) {
            line.push('#');
        } else {
            line.push('.');
        }

        log::debug!(
            "position {} sprite at {:?} ({})",
            cycle,
            sprite_range,
            line.last().unwrap()
        );

        cpu.run_cycle();

        if cycle == 39 {
            assert_eq!(line.len(), 40);
            image.push(String::from_iter(line.iter()));
            line = Vec::new();
        }
    }

    if line.len() > 0 {
        image.push(String::from_iter(line.iter()));
    }

    let result = image.join("\n");
    println!("{}", result);
    return result;
}
