use std::fs;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    assert!(check_view_str(9, "41354433332632554423635766486545647485859588", true) == 4);
    assert!(check_view(5, &vec![&5, &2]) == 1);
    assert!(check_view(5, &vec![&3]) == 1);
    assert!(check_view(5, &vec![&1, &2]) == 2);
    assert!(check_view(5, &vec![&3, &5, &3]) == 2);

    assert!(check_view(5, &vec![&3, &5, &3]) == 2);
    assert!(check_view(5, &vec![&3, &3]) == 2);
    assert!(check_view(5, &vec![&3]) == 1);
    assert!(check_view(5, &vec![&4, &9]) == 2);

    assert!(play("sample.txt") == (21, 8));
    println!("{:?}", play("data.txt"));
}

fn play(input: &str) -> (usize, usize) {
    let data = fs::read_to_string(input).expect("unable to read files");

    let mut grid: Vec<Vec<usize>> = Vec::new();
    for line in data.lines() {
        let l = Vec::from_iter(
            line.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap()),
        );

        grid.push(l);
    }

    log::debug!("Grid: {:?}", grid);

    let mut sum = 0;
    let mut max_score = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if is_visible(y, x, &grid) {
                sum += 1;
            }

            let (my_max, west, east, north, south) = get_max_score(y, x, &grid);

            if my_max == 0 {
                continue;
            }

            //println!("my_max: {}", my_max);
            if grid[y][x] == 9 {
                println!(
                    "max_score: {} ({}, {}, {}, {}) at ({}, {}) {}",
                    my_max, west, east, north, south, y, x, grid[y][x]
                );
            }
            if my_max > max_score {
                max_score = my_max;
            }
        }
    }

    println!("{}", max_score);

    return (sum, max_score);
}

fn cant_see_over(height: &usize, other_height: &usize) -> bool {
    return height < other_height;
}

fn check_view_str(height: usize, tree_line_str: &str, reverse: bool) -> usize {
    let mut numbers: Vec<usize> = tree_line_str
        .split("")
        .filter(|x| x.len() != 0)
        .map(|x| x.parse().unwrap())
        .collect();

    if reverse {
        numbers.reverse();
    }

    let ref_numbers = &numbers.iter().map(|x| x).collect();

    return check_view(height, ref_numbers);
}

fn check_view(height: usize, tree_line: &Vec<&usize>) -> usize {
    if tree_line.len() == 0 {
        log::debug!("check_view {}: {:?} == 0", height, tree_line);
        return 0;
    }
    let mut sum = 0;

    for &next in tree_line {
        if height == *next {
            sum += 1;
            break;
        }

        if cant_see_over(&height, &next) {
            sum += 1;
            break;
        }

        sum += 1;
    }

    log::debug!("check_view {}: {:?} == {}", height, tree_line, sum);
    return sum;
}

fn get_max_score(
    y: usize,
    x: usize,
    grid: &Vec<Vec<usize>>,
) -> (usize, usize, usize, usize, usize) {
    let height = grid[y][x];

    let west = check_view(height, &grid[y][0..x].iter().rev().collect());

    let east = check_view(height, &grid[y][(x + 1)..grid[y].len()].iter().collect());

    let north = check_view(height, &grid[0..y].iter().map(|v| &v[x]).rev().collect());

    let south = check_view(
        height,
        &grid[(y + 1)..grid[y].len()]
            .iter()
            .map(|v| &v[x])
            //.rev()
            .collect(),
    );

    //println!("{}, {} = {} {} {} {}", y, x, west, east, north, south);
    return (west * east * north * south, west, east, north, south);
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
