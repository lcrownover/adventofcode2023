use std::collections::{HashMap, VecDeque};

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

pub fn run(lines: &Vec<String>) {
    let mut scored_cards = 0;
    let mut work = VecDeque::new();
    work.extend(0..lines.len());

    while !work.is_empty() {
        let card_index = work.pop_front().unwrap();
        let card_to_score = lines.iter().nth(card_index).unwrap();
        let points = score_card_line(card_to_score.clone());
        for i in 1..=points {
            let card_index_to_add = card_index + i;
            work.push_back(card_index + i)
        }
        scored_cards += 1;
    }

    println!("{scored_cards}")
}
