use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Hand {
    text: String,
    bid: u64,
    hand_type: HandType,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl Hand {
    fn new(text: String, bid: u64) -> Self {
        Hand {
            text: text.clone(),
            bid: bid,
            hand_type: Hand::classify(text),
        }
    }

    fn outranks(&self, other: &Hand) -> bool {
        if self.hand_type == other.hand_type {
            let order = vec![
                '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
            ];
            for i in 0..self.text.len() {
                let mine = order
                    .iter()
                    .position(|&v| v == self.text.chars().nth(i).unwrap())
                    .unwrap();

                let theirs = order
                    .iter()
                    .position(|&v| v == other.text.chars().nth(i).unwrap())
                    .unwrap();

                if mine == theirs {
                    continue;
                }

                return mine > theirs;
            }
        }
        self.hand_type > other.hand_type
    }
    fn classify(text: String) -> HandType {
        // Five of a kind
        // Four of a kind
        // Full house ( 3 of a kind, 2 of a kind)
        // Three of a kind
        // Two pairs
        // One pair
        // High card (all are different)
        let mut m: HashMap<char, u32> = HashMap::new();
        for c in text.chars() {
            if m.contains_key(&c) {
                let count = m.get(&c).unwrap();
                m.insert(c, count + 1);
                continue;
            }

            m.insert(c, 1);
        }

        // map is built, now we classify based on patterns

        // check for five of a kind
        if m.values().any(|&v| v == 5) {
            return HandType::FiveOfAKind;
        }
        // four of a kind
        if m.values().any(|&v| v == 4) {
            return HandType::FourOfAKind;
        }
        // full house
        if m.values().any(|&v| v == 3) && m.values().any(|&v| v == 2) {
            return HandType::FullHouse;
        }
        // three of a kind

        if m.values().any(|&v| v == 3) {
            return HandType::ThreeOfAKind;
        }
        // two pairs
        if m.values().filter(|&v| *v == 2).count() == 2 {
            return HandType::TwoPair;
        }

        // one pair
        if m.values().any(|&v| v == 2) {
            return HandType::OnePair;
        }
        // high card
        HandType::HighCard
    }
}

fn sort_hands(hands: Vec<Hand>) -> Vec<Hand> {
    let mut sorted_hands: Vec<Hand> = hands.clone();
    // lets write a bubble sort
    let n = sorted_hands.len();
    for i in 0..n {
        // println!("{:?}", sorted_hands);
        let mut swapped: bool = false;
        for j in 0..n - i - 1 {
            if sorted_hands[j].outranks(&sorted_hands[j + 1]) {
                sorted_hands.swap(j, j + 1);
                swapped = true
            }
        }
        if !swapped {
            break;
        }
    }
    sorted_hands
}

fn score_hands(hands: Vec<Hand>) -> u64 {
    let mut total: u64 = 0;
    for (i, hand) in hands.iter().enumerate() {
        total += hand.bid * (1 + i as u64)
    }
    total
}

pub fn run(lines: &Vec<String>) {
    let mut hands: Vec<Hand> = vec![];
    for line in lines {
        let text = line.split_ascii_whitespace().nth(0).unwrap();
        let bid = line
            .split_ascii_whitespace()
            .nth(1)
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let h: Hand = Hand::new(text.to_string(), bid);

        hands.push(h.clone());
    }

    let sorted_hands = sort_hands(hands.clone());
    println!("{:?}", score_hands(sorted_hands))
}
