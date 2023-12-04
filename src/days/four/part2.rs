use std::collections::{HashMap, VecDeque};

struct CardScore {
    score: usize,
}

impl CardScore {
    fn increment(&mut self) {
        if self.score == 0 {
            self.score = 1;
            return;
        }
        self.score *= 2;
    }
}

fn score_card_line(line: String) -> usize {
    let mut card_score = 0;
    let score_line = line.split(":").nth(1).unwrap().trim();
    let winning_line = score_line.split("|").nth(0).unwrap().trim();
    let my_line = score_line.split("|").nth(1).unwrap().trim();
    let winning_scores: Vec<usize> = winning_line
        .split_whitespace()
        .map(|v| v.parse::<usize>().unwrap())
        .collect();
    let my_scores: Vec<usize> = my_line
        .split_whitespace()
        .map(|v| v.parse::<usize>().unwrap())
        .collect();
    for score in my_scores {
        if winning_scores.contains(&score) {
            card_score += 1;
        }
    }
    card_score
}

use std::io::{stdin, stdout, Read, Write};

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

pub fn run(lines: &Vec<String>) {
    let mut scored_cards = 0;
    let mut work: Vec<usize> = (0..lines.len()).collect();

    while !work.is_empty() {
        // println!("processing queue {:?}", work);
        let card_index = work.iter().nth(0).unwrap().clone();
        work.remove(0);
        let card_to_score = lines.iter().nth(card_index).unwrap();
        // println!("  scoring card {card_to_score}");
        let points = score_card_line(card_to_score.clone());
        // println!("  points {points}, adding cards...");
        for i in 1..=points {
            let card_index_to_add = card_index + i;
            // println!("    adding card {} at index {}", card_index_to_add+1, card_index_to_add);
            // offset by 1
            work.push(card_index + i)
        }
        scored_cards += 1;
        work.sort();
        // println!("work {}", work.len());
        // pause()
    }

    println!("{scored_cards}")
}
