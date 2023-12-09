#[derive(Debug)]
struct Dataset {
    sets: Vec<Vec<i64>>,
}

fn build_dataset(nums: &Vec<i64>) -> Dataset {
    let mut sets: Vec<Vec<i64>> = vec![];
    sets.push(nums.clone());
    let mut processing_nums: Vec<i64> = nums.clone();
    loop {
        let mut lesser_set: Vec<i64> = vec![];
        let pnums = &processing_nums.as_slice()[1..processing_nums.len().clone()];

        for (i, num) in pnums.iter().enumerate() {
            lesser_set.push(num - processing_nums.iter().nth(i).unwrap())
        }
        sets.push(lesser_set.clone());
        processing_nums = lesser_set.clone();
        if lesser_set.iter().all(|&v| v == 0) {
            break;
        }
    }
    Dataset { sets }
}

fn get_prediction(dataset: &Dataset) -> i64 {
    // reverse through the sets
    // println!("{:?}", dataset);
    let mut last_num: i64 = 0;
    for set in dataset.sets.iter().rev() {
        last_num = set.last().unwrap() + last_num;
        if last_num == 0 {
            continue;
        }
    }
    last_num
}

fn nums_from_line(line: &String) -> Vec<i64> {
    line.trim()
        .split_ascii_whitespace()
        .map(|v| v.parse::<i64>().unwrap())
        .collect()
}

pub fn run(lines: &Vec<String>) {
    let mut total = 0;
    for line in lines {
        let nums = nums_from_line(line);
        let ds = build_dataset(&nums);
        total += get_prediction(&ds);
    }
    println!("{total}")
}
