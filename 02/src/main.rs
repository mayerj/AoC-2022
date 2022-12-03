use core::fmt;
use std::fs;

#[repr(i32)]
#[derive(Eq, PartialEq, Clone, Copy)]
enum Selection {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl fmt::Display for Selection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Selection::Rock => write!(f, "Rock"),
            Selection::Paper => write!(f, "Paper"),
            Selection::Scissors => write!(f, "Scissors"),
        }
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
enum NeededResult {
    Win,
    Loss,
    Draw,
}

impl fmt::Display for NeededResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NeededResult::Win => write!(f, "Win"),
            NeededResult::Loss => write!(f, "Loss"),
            NeededResult::Draw => write!(f, "Draw"),
        }
    }
}

fn to_selection(input: &str) -> Option<Selection> {
    return match input {
        "X" | "A" => Some(Selection::Rock),
        "Y" | "B" => Some(Selection::Paper),
        "Z" | "C" => Some(Selection::Scissors),
        _ => Option::None,
    };
}

fn to_result(input: &str) -> Option<NeededResult> {
    return match input {
        "X" | "A" => Some(NeededResult::Loss),
        "Y" | "B" => Some(NeededResult::Draw),
        "Z" | "C" => Some(NeededResult::Win),
        _ => Option::None,
    };
}

fn get_points(input: &Selection) -> i32 {
    return match input {
        Selection::Rock => 1,
        Selection::Paper => 2,
        Selection::Scissors => 3,
    };
}

fn to_selection_from_i32(input: i32) -> Option<Selection> {
    return match input {
        0 => Some(Selection::Rock),
        1 => Some(Selection::Paper),
        2 => Some(Selection::Scissors),
        _ => Option::None,
    };
}
fn get_my_choice(opponent: &Selection, needed_result: &NeededResult) -> Selection {
    let opponent_value = *opponent as i32;
    return match needed_result {
        NeededResult::Draw => opponent.clone(),
        NeededResult::Win => to_selection_from_i32((3 + opponent_value + 1) % 3).unwrap(),
        NeededResult::Loss => to_selection_from_i32((3 + opponent_value - 1) % 3).unwrap(),
    };
}

fn get_win_points(my_selection: &Selection, opponent: &Selection) -> i32 {
    if my_selection == opponent {
        return 3;
    }

    if opponent == &Selection::Rock {
        return if my_selection == &Selection::Paper {
            6
        } else {
            0
        };
    }

    if opponent == &Selection::Paper {
        return if my_selection == &Selection::Scissors {
            6
        } else {
            0
        };
    }

    if opponent == &Selection::Scissors {
        return if my_selection == &Selection::Rock {
            6
        } else {
            0
        };
    }

    return 0;
}

fn main() {
    assert!(play("sample.txt") == 12);
    println!("{}", play("data.txt"));
}

fn play(input: &str) -> i32 {
    let data = fs::read_to_string(input).expect("unable to read files");

    let chunks = data.split("\r\n").collect::<Vec<&str>>();

    let mut total = 0;

    for line in chunks {
        let choices = line.split(" ").collect::<Vec<_>>();

        let opponent_choice = to_selection(choices.get(0).unwrap()).unwrap();
        let needed_result = to_result(choices.get(1).unwrap()).unwrap();

        let my_choice = get_my_choice(&opponent_choice, &needed_result);

        let choice_points = get_points(&my_choice);

        let win_points = get_win_points(&my_choice, &opponent_choice);

        total += choice_points + win_points;

        println!(
            "{} {} {} {}",
            choice_points, win_points, my_choice, opponent_choice
        )
    }

    println!("{}", total);

    return total;
}
