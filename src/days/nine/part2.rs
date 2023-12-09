use std::collections::VecDeque;

#[derive(Debug)]
struct Dataset {
    sets: Vec<VecDeque<i64>>,
}

fn build_dataset(nums: &VecDeque<i64>) -> Dataset {
    let mut sets: Vec<VecDeque<i64>> = vec![];
    sets.push(nums.clone());
    let mut processing_nums: VecDeque<i64> = nums.clone();
    loop {
        let mut lesser_set: VecDeque<i64> = VecDeque::new();
        let mut pnums = processing_nums.clone();
        pnums.pop_front();

        for (i, num) in pnums.iter().enumerate() {
            lesser_set.push_back(num - processing_nums.iter().nth(i).unwrap())
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
    let mut first_num: i64 = 0;
    for set in dataset.sets.iter().rev() {
        first_num = set.clone().pop_front().unwrap() - first_num;
        if first_num == 0 {
            continue;
        }
    }
    first_num
}

fn nums_from_line(line: &String) -> VecDeque<i64> {
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
        // println!("{ds:?}");
        total += get_prediction(&ds);
    }
    println!("{total}")
}
