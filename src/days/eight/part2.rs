use primes::{PrimeSet, Sieve};
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

#[derive(Debug, Clone)]
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

fn check_success(mapline: &MapLine) -> bool {
    mapline.id.ends_with("Z")
}

fn get_starting_maplines(maplines: &HashMap<String, MapLine>) -> Vec<MapLine> {
    let mut starting: Vec<MapLine> = Vec::new();
    for k in maplines.keys() {
        if k.ends_with("A") {
            starting.push(maplines.get(k).unwrap().clone())
        }
    }
    starting
}

fn follow_instruction(
    master_map: &HashMap<String, MapLine>,
    current_mapline: &MapLine,
    instruction: &Direction,
) -> MapLine {
    match instruction {
        Direction::Left => return master_map.get(&current_mapline.left).unwrap().clone(),
        Direction::Right => return master_map.get(&current_mapline.right).unwrap().clone(),
    }
}

trait IncrementExt {
    fn increment(&mut self, key: u64);
}

impl IncrementExt for HashMap<u64, u64> {
    fn increment(&mut self, key: u64) {
        match self.get_mut(&key) {
            Some(v) => *v += 1,
            None => {
                self.insert(key, 1);
                ()
            }
        };
    }
}

fn get_prime_factors(num: &u64) -> HashMap<u64, u64> {
    let mut prime_list = Sieve::new();
    let mut out = HashMap::new();
    let mut n = num.clone();
    for pnum in prime_list.iter() {
        if n == 1 {
            break;
        }
        loop {
            if n.rem_euclid(pnum) != 0 {
                break;
            }
            out.increment(pnum.clone());
            n = n / pnum;
        }
    }
    out
}

fn lcm(nums: &Vec<u64>) -> u64 {
    let mut prime_maps: Vec<HashMap<u64, u64>> = vec![];
    for num in nums {
        // println!("factorizing num {num}");
        prime_maps.push(get_prime_factors(num))
    }
    // println!("prime maps {prime_maps:?}");
    // it looks like each number is just two prime numbers multiplied by eachother
    // so we can just grab all the keys, dedup it, then multiply them together
    let mut multiples: Vec<u64> = prime_maps
        .iter()
        .map(|hm| hm.keys().map(|k| k.clone()).collect::<Vec<u64>>())
        .flatten()
        .collect();
    multiples.sort();
    multiples.dedup();

    // println!("multiples {multiples:?}");
    multiples.iter().product()
}

// for each of the maplines, find how many iterations it takes to get to the corresponding 'Z',
// then take the LCM of all those values
pub fn run(lines: &Vec<String>) {
    let master_map = get_map_lines(lines);
    let instructions_text = get_instructions(lines);
    let starting_maplines = get_starting_maplines(&master_map);
    let mut mapline_cycles: Vec<u64> = vec![];

    for ml in starting_maplines {
        let instructions: Instructions = Instructions::new(instructions_text);
        let mut cycle_counter = 0;
        let mut current_mapline: MapLine = ml.clone();
        for instruction in instructions {
            if check_success(&current_mapline) {
                mapline_cycles.push(cycle_counter);
                break;
            }
            cycle_counter += 1;
            current_mapline = follow_instruction(&master_map, &current_mapline, &instruction)
        }
    }

    let ans = lcm(&mapline_cycles);
    println!("{ans}")
}
