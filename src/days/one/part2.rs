use std::collections::HashMap;

struct LineCount {
    first: Option<i32>,
    last: Option<i32>,
    first_num: Option<i32>,
    first_num_index: usize,
    last_num: Option<i32>,
    last_num_index: usize,
}

impl LineCount {
    fn total(&self) -> i32 {
        if self.first_num.is_none() {
            panic!("first number is not set")
        }
        if self.last_num.is_none() {
            panic!("last number is not set")
        }
        let n = format!("{}{}", self.first_num.unwrap(), self.last_num.unwrap());
        n.parse::<i32>().unwrap()
    }
}

pub fn run(lines: &Vec<String>) {
    let mut total = 0;
    let mut words = HashMap::new();
    words.insert("one", 1);
    words.insert("two", 2);
    words.insert("three", 3);
    words.insert("four", 4);
    words.insert("five", 5);
    words.insert("six", 6);
    words.insert("seven", 7);
    words.insert("eight", 8);
    words.insert("nine", 9);

    for line in lines {
        let mut lc = LineCount {
            first: None,
            last: None,
            first_num: None,
            first_num_index: 0,
            last_num: None,
            last_num_index: 0,
        };
        println!("\n\n--------------------------");
        println!("{}", line);
        println!("--------------------------");

        // start from the head of the line and check for numbers or the number string
        // println!("starting integer scan from the left");
        for (i, c) in line.char_indices() {
            // println!("i {} c {}", i, c);
            // this is in the case that its a number
            if c.is_numeric() {
                if lc.first_num == None {
                    println!("-> first num   {}", c.to_string().parse::<i32>().unwrap());
                    println!("-> first index {}", i);
                    // if n is not set, set it to the first number
                    lc.first_num = Some(c.to_string().parse::<i32>().unwrap());
                    lc.first_num_index = i;
                    continue;
                }
            }
        }
        // reverse the order and do the same
        // println!("starting integer scan from the right");
        for (i, c) in line.char_indices().rev() {
            // println!("i {} c {}", i, c);
            if c.is_numeric() {
                if lc.last_num == None {
                    println!("<- last num    {}", c.to_string().parse::<i32>().unwrap());
                    println!("<- last index  {}", i);
                    // if n is not set, set it to the first number
                    lc.last_num = Some(c.to_string().parse::<i32>().unwrap());
                    lc.last_num_index = i;
                    continue;
                }
            }
        }

        // now we start matching the words
        // println!("starting word scan");
        for (w, v) in words.iter() {
            let word = *w;
            // println!("-> searching for word {}", word);
            let maybe_idx = line.find(word);
            if maybe_idx.is_some() {
                let i = maybe_idx.unwrap();
                // println!("found word at index {}", i);
                // if the word is found earlier than the first_num_index
                if i <= lc.first_num_index {
                    // println!( "start of word '{}' was found earlier than first number, index {}. setting first number to {}", word, i, *v);
                    lc.first_num = Some(*v);
                    lc.first_num_index = i;
                    println!("-> first num   {}", *v);
                    println!("-> first index {}", i);
                }
            }
            let maybe_idx = line.rfind(word);
            if maybe_idx.is_some() {
                let i = maybe_idx.unwrap();
                // if the start of the word is found later than the last index
                if i >= lc.last_num_index {
                    // println!( "start of word '{}' was found later than last number, index {}. setting last number to {}", word, i, *v);
                    lc.last_num = Some(*v);
                    lc.last_num_index = i;
                    println!("<- last num    {}", *v);
                    println!("<- last index  {}", i);
                }
            }
        }

        let wt = lc.total();
        println!("line total for {} is {}", line, wt);
        total += wt;
    }
    println!("\n\n{}", total)
}
