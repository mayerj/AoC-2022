use std::fs;

fn main() {
    assert!(play("sample.txt") == 21);
    println!("{:?}", play("data.txt"));
}

fn play(input: &str) -> usize {
    let data = fs::read_to_string(input).expect("unable to read files");

    let mut grid: Vec<Vec<usize>> = Vec::new();
    for line in data.split("\r\n") {
        let l = Vec::from_iter(
            line.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap()),
        );

        grid.push(l);
    }

    println!("{:?}", grid);

    let mut sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if is_visible(y, x, &grid) {
                sum += 1;
                println!("Visible: {}, {}", y, x);
            } else {
                println!("Not Visible: {}, {}", y, x);
            }
        }
    }

    println!("Sum: {}", sum);
    return sum;
}

fn is_visible(y: usize, x: usize, grid: &Vec<Vec<usize>>) -> bool {
    //println!("Checking {}, {}", y, x);

    if y == 0 || y == grid.len() - 1 {
        return true;
    }

    if x == 0 || x == grid[y].len() - 1 {
        return true;
    }

    let height = grid[y][x];

    if grid[y][0..x].iter().all(|val| *val < height) {
        return true;
    }

    if grid[y][(x + 1)..grid[y].len()]
        .iter()
        .all(|val| *val < height)
    {
        return true;
    }

    if grid[0..y].iter().all(|v| v[x] < height) {
        return true;
    }

    if grid[(y + 1)..grid[y].len()].iter().all(|v| v[x] < height) {
        return true;
    }

    return false;
}
