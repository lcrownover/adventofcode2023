#[derive(Debug)]
struct Board {
    rows: Vec<BoardRow>,
    start: (usize, usize),
}
type BoardRow = Vec<char>;
type Position = (usize, usize);
struct Node {
    symbol: char,
    position: Position,
}

impl Board {
    fn from_lines(lines: &Vec<String>) -> Self {
        let mut rows = vec![];
        let mut start: Position = (0, 0);
        for (i, line) in lines.iter().enumerate() {
            let row: Vec<char> = line.trim().chars().collect();
            rows.push(row.clone());
            if row.contains(&'S') {
                start = (
                    row.iter().position(|&c| c == 'S').unwrap() as usize,
                    i as usize,
                );
            }
        }
        Board { rows, start }
    }

    fn get_node(&self, position: Position) -> Option<Node> {
        let row = match self.rows.iter().nth(position.0) {
            Some(row) => row,
            None => return None,
        };
        let mut c = match row.iter().nth(position.1) {
            Some(c) => c,
            None => return None,
        };
        Some(Node {
            symbol: *c,
            position: position,
        })
    }

    fn start_node(&self) -> Node {
        self.get_node(self.start).unwrap()
    }

    fn follow_node(&self, node: &Node) -> Node {
        match node.symbol {
            '-' => {
                let new_pos = (node.position.0 + 1, node.position.1);
                self.get_node(new_pos).unwrap()
            }
            '|' => {
                let new_pos = (node.position.0, node.position.1);
                self.get_node(new_pos).unwrap()
            }
        }
    }
}

pub fn run(lines: &Vec<String>) {
    let board = Board::from_lines(lines);
    println!("board {board:?}");
}
