use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(c: &char) -> Self {
        match c {
            'L' => return Direction::Left,
            'R' => return Direction::Right,
            _ => panic!("unknown direction"),
        }
    }
}

#[derive(Debug)]
struct Instructions {
    original: VecDeque<Direction>,
    remaining: VecDeque<Direction>,
}

impl Instructions {
    fn new(text: &String) -> Self {
        let mut original: VecDeque<Direction> = VecDeque::new();
        for c in text.chars() {
            original.push_back(Direction::from_char(&c))
        }
        let remaining: VecDeque<Direction> = original.clone();
        Instructions {
            original: original,
            remaining: remaining,
        }
    }
}

impl Iterator for Instructions {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let maybe_current = self.remaining.pop_front();
        match maybe_current {
            Some(d) => return Some(d),
            None => {
                self.remaining = self.original.clone();
                return Some(self.remaining.pop_front().unwrap());
            }
        }
    }
}

#[derive(Debug)]
struct MapLine {
    id: String,
    left: String,
    right: String,
}

impl MapLine {
    fn from_string(text: &String) -> MapLine {
        let id = text.split("=").nth(0).unwrap().trim().to_string();
        let dirs = text
            .split("=")
            .nth(1)
            .unwrap()
            .trim()
            .replace("(", "")
            .replace(")", "");
        let left = dirs.split(",").nth(0).unwrap().trim().to_string();
        let right = dirs.split(",").nth(1).unwrap().trim().to_string();
        MapLine { id, left, right }
    }
}

fn get_instructions(lines: &Vec<String>) -> &String {
    let instruction_text = lines.iter().nth(0).unwrap();
    instruction_text
}

fn get_map_lines(lines: &Vec<String>) -> HashMap<String, MapLine> {
    let maplines = lines.as_slice()[1..lines.len()].to_vec();
    let mut hm: HashMap<String, MapLine> = HashMap::new();
    for line in maplines {
        let ml: MapLine = MapLine::from_string(&line);
        hm.insert(ml.id.clone(), ml);
    }
    hm
}

pub fn run(lines: &Vec<String>) {
    let instructions_text = get_instructions(lines);
    let mut instructions: Instructions = Instructions::new(instructions_text);
    let maplines = get_map_lines(lines);

    let mut counter = 0;
    let mut next_mapline = maplines.get("AAA").unwrap();
    loop {
        if next_mapline.id == "ZZZ" {
            break;
        }
        counter += 1;
        let direction = instructions.next().unwrap();
        next_mapline = match direction {
            Direction::Left => maplines.get(&next_mapline.left).unwrap(),
            Direction::Right => maplines.get(&next_mapline.right).unwrap(),
        };
    }
    println!("{counter}");
}
