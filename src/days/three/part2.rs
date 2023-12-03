struct Board {
    rows: Vec<Vec<char>>,
}

impl Board {
    fn from_lines(lines: &Vec<String>) -> Self {
        let mut board = Board { rows: vec![] };
        for line in lines {
            board.rows.push(line.chars().collect())
        }
        board
    }
}

/// This should return the character at the location
/// If the character is out of bounds, it just returns a '.'
fn get_val_at_loc(x: isize, y: isize, board: &Board) -> char {
    // println!("getting val at location {},{}", x, y);
    let c = board
        .rows
        .iter()
        .nth(y as usize)
        .unwrap_or(&vec!['.'])
        .iter()
        .nth(x as usize)
        .unwrap_or(&'.')
        .clone();
    c
}

/// This should check all neighbors and return the value of
/// any TWO neighbors multipied together
fn calculate_gear_neighbors(x: isize, y: isize, board: &Board) -> u32 {
    let gear_coord = (x, y);
    let mut number_coords: Vec<(isize, isize)> = vec![];

    let topleft = get_val_at_loc(x - 1, y + 1, board);
    if topleft.is_numeric() {
        number_coords.push((x - 1, y + 1));
    }
    let topmiddle = get_val_at_loc(x, y + 1, board);
    if topmiddle.is_numeric() && !is_dot(topmiddle) {
        number_coords.push((x, y + 1));
    }
    let topright = get_val_at_loc(x + 1, y + 1, board);
    if topright.is_numeric() && !is_dot(topright) {
        number_coords.push((x + 1, y + 1));
    }
    let left = get_val_at_loc(x - 1, y, board);
    if left.is_numeric() && !is_dot(left) {
        number_coords.push((x - 1, y));
    }
    let right = get_val_at_loc(x + 1, y, board);
    if right.is_numeric() && !is_dot(right) {
        number_coords.push((x + 1, y));
    }
    let bottomleft = get_val_at_loc(x - 1, y - 1, board);
    if bottomleft.is_numeric() && !is_dot(bottomleft) {
        number_coords.push((x - 1, y - 1));
    }
    let bottommiddle = get_val_at_loc(x, y - 1, board);
    if bottommiddle.is_numeric() && !is_dot(bottommiddle) {
        number_coords.push((x, y - 1));
    }
    let bottomright = get_val_at_loc(x + 1, y - 1, board);
    if bottomright.is_numeric() && !is_dot(bottomright) {
        number_coords.push((x + 1, y - 1));
    }

    // println!("uncollapsed number coords {:?}", number_coords);
    // any coordinates that are adjacent should be collapsed into one coordinate
    let collapsed_number_coords = collapse_number_coords(gear_coord, number_coords);

    println!("number coords {:?}", collapsed_number_coords);
    if collapsed_number_coords.len() != 2 {
        return 0;
    }

    collapsed_number_coords
        .iter()
        .map(|(x, y)| expand_number(*x, *y, board))
        .product()
}

/// This should take a list of coords
/// and return a filtered list of coords where we remove
/// any elements where its neighbor is already accounted for
///
/// Example:
///
/// input => [(2, 2), (3, 2), (2, 0)]
///
/// output => [(2, 2), (2, 0)]
///
fn collapse_number_coords(
    gear_coord: (isize, isize),
    number_coords: Vec<(isize, isize)>,
) -> Vec<(isize, isize)> {
    // println!("processing coords {:?}", number_coords);
    let mut bottoms: Vec<(isize, isize)> = vec![];
    let mut middles: Vec<(isize, isize)> = vec![];
    let mut tops: Vec<(isize, isize)> = vec![];
    // println!("sorting coords");
    for coord in number_coords {
        if coord.1 > gear_coord.1 {
            tops.push(coord);
        }
        if coord.1 == gear_coord.1 {
            middles.push(coord);
        }
        if coord.1 < gear_coord.1 {
            bottoms.push(coord);
        }
    }
    // println!("tops {:?}", tops);
    // println!("middles {:?}", middles);
    // println!("bottoms {:?}", bottoms);

    let mut valid_coords = vec![];
    if !tops.is_empty() {
        match tops.len() {
            // full number on top, just grab the first
            3 => valid_coords.push(tops.iter().nth(0).unwrap().clone()),
            // if they're not adjacent, add them both
            2 => {
                let (ax, ay) = tops.iter().nth(0).unwrap().clone();
                let (bx, by) = tops.iter().nth(1).unwrap().clone();
                if is_adjacent_x(ax, ay, bx, by) {
                    valid_coords.push((ax, ay))
                } else {
                    valid_coords.extend(tops)
                }
            }
            _ => valid_coords.push(tops.iter().nth(0).unwrap().clone()),
        }
    }
    if !bottoms.is_empty() {
        match bottoms.len() {
            // full number on top, just grab the first
            3 => valid_coords.push(bottoms.iter().nth(0).unwrap().clone()),
            // if they're not adjacent, add them both
            2 => {
                let (ax, ay) = bottoms.iter().nth(0).unwrap().clone();
                let (bx, by) = bottoms.iter().nth(1).unwrap().clone();
                if is_adjacent_x(ax, ay, bx, by) {
                    valid_coords.push((ax, ay))
                } else {
                    valid_coords.extend(bottoms)
                }
            }
            _ => valid_coords.push(bottoms.iter().nth(0).unwrap().clone()),
        }
    }
    valid_coords.extend(middles);
    valid_coords
}

#[allow(dead_code)]
/// checks if point A is adjacent to point B
fn is_adjacent_x(ax: isize, ay: isize, bx: isize, by: isize) -> bool {
    if ay != by {
        return false;
    }
    (ax - bx).abs() <= 1
}

/// looks left and right of the coordinates and builds a number
/// until it runs into a non-numeric symbol
fn expand_number(x: isize, y: isize, board: &Board) -> u32 {
    // println!("expanding number at {x}, {y}");
    let mut num: Vec<char> = vec![];
    let mut xt = x;
    num.push(get_val_at_loc(xt, y, board));
    loop {
        xt -= 1;
        let next_left = get_val_at_loc(xt, y, board);
        if next_left.is_numeric() {
            // println!("found number to the left at {xt}, {y} : {next_left}");
            num.insert(0, next_left);
            continue;
        }
        break;
    }
    xt = x;
    loop {
        xt += 1;
        let next_right = get_val_at_loc(xt, y, board);
        if next_right.is_numeric() {
            // println!("found number to the right at {xt}, {y} : {next_right}");
            num.push(next_right);
            continue;
        }
        break;
    }
    let s: String = num.iter().collect();
    println!("expanded number {s}");
    return s.parse::<u32>().expect("failed to parse number");
}

fn is_dot(c: char) -> bool {
    c == '.'
}

fn is_gear(c: char) -> bool {
    c == '*'
}

#[allow(dead_code)]
fn print_board(board: &Board) {
    for row in board.rows.iter() {
        let s: String = row.iter().collect();
        println!("{}", s)
    }
}

pub fn run(lines: &Vec<String>) {
    let board = Board::from_lines(lines);
    print_board(&board);

    let mut total_gear_nums = 0;

    // let mut num_tracker = "".to_string();
    // let mut gear_found = false;

    // iterate over the characters and once it finds a gear
    // we check all the neighbors.
    // if we find EXACTLY TWO numbers in the neighbors,
    // expand those numbers to their boundaries and multiply them together
    // then add to the total gear nums
    for (y, row) in board.rows.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if is_gear(*c) {
                println!("found gear at {x},{y}");
                total_gear_nums += calculate_gear_neighbors(x as isize, y as isize, &board)
            }
        }
    }
    println!("{total_gear_nums}")
}
