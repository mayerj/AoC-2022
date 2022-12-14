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
