use core::fmt;
use std::fs;

#[derive(Eq, PartialEq)]
enum Selection {
    Rock,
    Paper,
    Scissors,
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

fn to_selection(input: &str) -> Option<Selection> {
    return match input {
        "X" | "A" => Some(Selection::Rock),
        "Y" | "B" => Some(Selection::Paper),
        "Z" | "C" => Some(Selection::Scissors),
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
    let data = fs::read_to_string("data.txt").expect("unable to read files");

    let chunks = data.split("\r\n").collect::<Vec<&str>>();

    let mut total = 0;

    for line in chunks {
        let choices = line.split(" ").collect::<Vec<_>>();

        let opponent_choice = to_selection(choices.get(0).unwrap()).unwrap();
        let my_choice = to_selection(choices.get(1).unwrap()).unwrap();

        let choice_points = get_points(&my_choice);

        let win_points = get_win_points(&my_choice, &opponent_choice);

        total += choice_points + win_points;

        println!(
            "{} {} {} {}",
            choice_points, win_points, my_choice, opponent_choice
        )
    }

    println!("{}", total);
}
