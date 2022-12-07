use itertools::{concat, Itertools};
use std::fs;

struct Stacks {
    stacks: Vec<Vec<char>>,
}

fn get_sample_structure() -> Vec<Vec<char>> {
    return vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
}

fn get_data_structure() -> Vec<Vec<char>> {
    return vec![
        vec!['D', 'L', 'V', 'T', 'M', 'H', 'F'],
        vec!['H', 'Q', 'G', 'J', 'C', 'T', 'N', 'P'],
        vec!['R', 'S', 'D', 'M', 'P', 'H'],
        vec!['L', 'B', 'V', 'F'],
        vec!['N', 'H', 'G', 'L', 'Q'],
        vec!['W', 'B', 'D', 'G', 'R', 'M', 'P'],
        vec!['G', 'M', 'N', 'R', 'C', 'H', 'L', 'Q'],
        vec!['C', 'L', 'W'],
        vec!['R', 'D', 'L', 'Q', 'J', 'Z', 'M', 'T'],
    ];
}

fn main() {
    assert!(play(get_sample_structure(), "sample.txt") == "CMZ");
    println!("{}", play(get_data_structure(), "data.txt"));
}

fn play(cargo: Vec<Vec<char>>, input: &str) -> String {
    let data = fs::read_to_string(input).expect("unable to read files");

    let mut state: Vec<Vec<char>> = Vec::new();
    for inner in cargo {
        let copy: Vec<char> = inner.iter().map(|x| *x).collect();
        state.push(copy);
    }

    let moves: Vec<(usize, usize, usize)> = data
        .split("\r\n")
        .map(|line| line.split(" ").collect::<Vec<_>>())
        .map(|data| {
            (
                data.get(1).unwrap().parse::<usize>().unwrap(),
                data.get(3).unwrap().parse::<usize>().unwrap() - 1,
                data.get(5).unwrap().parse::<usize>().unwrap() - 1,
            )
        })
        .collect();

    for (count, src, dest) in moves {
        let src_slot = state.get(src).unwrap();

        let mut new_state: Vec<Vec<char>> = Vec::new();
        for (index, inner) in state.iter().enumerate() {
            if index == src {
                let copy = Vec::from_iter(src_slot[0..(src_slot.len() - count)].iter().map(|x| *x));
                new_state.push(copy);
            } else if index == dest {
                let mut moved = (src_slot[(src_slot.len() - count)..src_slot.len()]).to_vec();
                moved.reverse();
                let copy = concat(vec![inner.clone(), moved]).clone();
                new_state.push(copy);
            } else {
                let copy: Vec<char> = inner.iter().map(|x| *x).collect();
                new_state.push(copy);
            }
        }
        println!("Move {} from {} to {}", count, src, dest);
        println!("{:?}", state);
        println!("{:?}", new_state);
        state = new_state;
    }

    let tops = state
        .iter()
        .map(|stack| *stack.get(stack.len() - 1).unwrap())
        .collect::<Vec<char>>();

    let ret = tops.iter().collect();

    println!("{}", ret);
    return ret;
}
