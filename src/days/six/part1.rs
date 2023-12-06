#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

fn calculate_distance(charge_time: u32, total_time: u32) -> u32 {
    (total_time - charge_time) * charge_time
}

fn parse_races(lines: &Vec<String>) -> Vec<Race> {
    let mut races: Vec<Race> = vec![];
    let mut race_times: Vec<u32> = vec![];
    let mut race_distances: Vec<u32> = vec![];
    for line in lines {
        if line.starts_with("Time:") {
            race_times = line
                .split(":")
                .nth(1)
                .unwrap()
                .trim()
                .split_ascii_whitespace()
                .map(|v| v.parse::<u32>().unwrap())
                .collect();
        }
        if line.starts_with("Distance:") {
            race_distances = line
                .split(":")
                .nth(1)
                .unwrap()
                .trim()
                .split_ascii_whitespace()
                .map(|v| v.parse::<u32>().unwrap())
                .collect();
        }
    }
    for i in 0..race_times.len() {
        let race = Race {
            time: race_times[i],
            distance: race_distances[i],
        };
        races.push(race)
    }
    races
}

pub fn run(lines: &Vec<String>) {
    let races = parse_races(lines);
    let mut results: Vec<u32> = vec![];

    for race in races {
        let mut ways = 0;
        for i in 0..race.time {
            let t = calculate_distance(i, race.time);
            if t > race.distance {
                ways += 1;
            }
        }
        results.push(ways)
    }
    println!("{}", results.into_iter().product::<u32>())
}
