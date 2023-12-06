#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn calculate_distance(charge_time: u64, total_time: u64) -> u64 {
    (total_time - charge_time) * charge_time
}

fn parse_race(lines: &Vec<String>) -> Race {
    let mut race_time: u64 = 0;
    let mut race_distance: u64 = 0;
    for line in lines {
        if line.starts_with("Time:") {
            race_time = line
                .split(":")
                .nth(1)
                .unwrap()
                .trim()
                .split_ascii_whitespace()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join("")
                .parse::<u64>()
                .unwrap()
        }
        if line.starts_with("Distance:") {
            race_distance = line
                .split(":")
                .nth(1)
                .unwrap()
                .trim()
                .split_ascii_whitespace()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join("")
                .parse::<u64>()
                .unwrap()
        }
    }
    let race = Race {
        time: race_time,
        distance: race_distance,
    };
    race
}

pub fn run(lines: &Vec<String>) {
    let race = parse_race(lines);

    let mut ways = 0;
    for i in 0..race.time {
        let t = calculate_distance(i, race.time);
        if t > race.distance {
            ways += 1;
        }
    }
    println!("{ways}")
}
