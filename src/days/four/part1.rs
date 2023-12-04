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

pub fn run(lines: &Vec<String>) {
    let mut total_score = 0;

    for card_line in lines {
        let mut card_score = CardScore { score: 0 };

        let score_line = card_line.split(":").nth(1).unwrap().trim();
        // println!("score line {score_line}");
        let winning_line = score_line.split("|").nth(0).unwrap().trim();
        // println!("winning line {winning_line}");
        let my_line = score_line.split("|").nth(1).unwrap().trim();
        // println!("my line {my_line}");
        let winning_scores: Vec<usize> = winning_line
            .split_whitespace()
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
        let my_scores: Vec<usize> = my_line
            .split_whitespace()
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
        // println!("winning {:?}", winning_scores);
        // println!("my      {:?}", my_scores);
        for score in my_scores {
            if winning_scores.contains(&score) {
                card_score.increment()
            }
        }
        // println!("adding {}", card_score.score);
        total_score += card_score.score;
        // println!("total {total_score}");
    }

    println!("{total_score}")
}
