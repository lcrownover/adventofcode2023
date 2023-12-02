#[derive(Debug)]
struct Hand {
    red: i32,
    blue: i32,
    green: i32,
}

impl Hand {
    fn hands_from_string(hand: String) -> Vec<Hand> {
        let mut hands: Vec<Hand> = vec![];
        let hand_strings: Vec<&str> = hand.split(";").map(|h| h.trim()).collect();
        for hand_string in hand_strings {
            let mut hand = Hand {
                red: 0,
                blue: 0,
                green: 0,
            };
            let dice_strings: Vec<String> = hand_string
                .split(",")
                .map(|d| d.trim().to_string())
                .collect();
            for dice_string in dice_strings {
                let dice_color = dice_string.split(" ").nth(1).unwrap().trim();
                let dice_count = dice_string
                    .split(" ")
                    .nth(0)
                    .unwrap()
                    .trim()
                    .parse::<i32>()
                    .unwrap();
                match dice_color {
                    "blue" => hand.blue = dice_count,
                    "red" => hand.red = dice_count,
                    "green" => hand.green = dice_count,
                    _ => panic!("unsupported color"),
                }
            }
            hands.push(hand)
        }
        hands
    }
}

#[derive(Debug)]
struct Game {
    id: i32,
    hands: Vec<Hand>,
}

impl Game {
    fn from_string(line: String) -> Game {
        let id = line
            .split(":")
            .nth(0)
            .unwrap()
            .strip_prefix("Game")
            .unwrap()
            .trim()
            .to_string()
            .parse::<i32>()
            .unwrap();
        let hands_string = line.split(":").nth(1).unwrap().trim().to_string();
        let hands: Vec<Hand> = Hand::hands_from_string(hands_string);

        Game { id, hands }
    }
}

pub fn run(lines: &Vec<String>) {
    let red_limit = 12;
    let green_limit = 13;
    let blue_limit = 14;

    let mut games: Vec<Game> = vec![];
    for line in lines {
        games.push(Game::from_string(line.to_string()))
    }
    let mut score = 0;
    for game in games {
        let mut game_valid = true;
        // println!("{:?}", game);
        for hand in game.hands {
            if hand.red > red_limit || hand.green > green_limit || hand.blue > blue_limit {
                game_valid = false;
            }
        }
        if game_valid {
            score += game.id
        }
    }

    println!("part1: {}", score)
}
