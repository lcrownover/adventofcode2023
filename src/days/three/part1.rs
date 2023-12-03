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

    /// This should return the character at the location
    /// If the character is out of bounds, it just returns a '.'
    fn get_val_at_loc(&self, x: isize, y: isize) -> char {
        // println!("getting val at location {},{}", x, y);
        let c = self
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

    fn has_part_neighbor(x: usize, y: usize, board: &Board) -> bool {
        let ix = x as isize;
        let iy = y as isize;

        let topleft = board.get_val_at_loc(ix - 1, iy + 1);
        if !topleft.is_numeric() && !is_dot(topleft) {
            return true;
        }
        let topmiddle = board.get_val_at_loc(ix, iy + 1);
        if !topmiddle.is_numeric() && !is_dot(topmiddle) {
            return true;
        }
        let topright = board.get_val_at_loc(ix + 1, iy + 1);
        if !topright.is_numeric() && !is_dot(topright) {
            return true;
        }
        let left = board.get_val_at_loc(ix - 1, iy);
        if !left.is_numeric() && !is_dot(left) {
            return true;
        }
        let right = board.get_val_at_loc(ix + 1, iy);
        if !right.is_numeric() && !is_dot(right) {
            return true;
        }
        let bottomleft = board.get_val_at_loc(ix - 1, iy - 1);
        if !bottomleft.is_numeric() && !is_dot(bottomleft) {
            return true;
        }
        let bottommiddle = board.get_val_at_loc(ix, iy - 1);
        if !bottommiddle.is_numeric() && !is_dot(bottommiddle) {
            return true;
        }
        let bottomright = board.get_val_at_loc(ix + 1, iy - 1);
        if !bottomright.is_numeric() && !is_dot(bottomright) {
            return true;
        }
        false
    }
}

fn is_dot(c: char) -> bool {
    c == '.'
}

pub fn run(lines: &Vec<String>) {
    let board = Board::from_lines(lines);
    let mut valid_numbers: Vec<u32> = vec![];
    let mut num_tracker = "".to_string();
    let mut part_found = false;
    for (y, row) in board.rows.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            // if we find anything other than a number,
            // process the number tracker and reset it
            if !c.is_numeric() {
                if !num_tracker.is_empty() && part_found {
                    // num_tracker has some digits and we found a part
                    valid_numbers.push(num_tracker.parse::<u32>().unwrap());
                }
                // reset
                num_tracker = "".to_string();
                part_found = false;
            }
            if c.is_numeric() {
                // we found a number
                // add it to the part tracker
                // then check if any of its neighbors are symbols
                num_tracker.push(*c);
                if Board::has_part_neighbor(x, y, &board) {
                    part_found = true;
                }
            }
        }
    }
    // println!("found numbers: {:?}", valid_numbers);
    let total: u32 = valid_numbers.iter().sum();
    println!("{total}")
}
