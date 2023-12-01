pub fn run(lines: &Vec<String>) {
    let mut total = 0;
    for line in lines {
        let mut n: String = "".to_string();
        let mut last_n: String = "".to_string();
        for c in line.chars() {
            if c.is_numeric() {
                if n == "" {
                    n = c.to_string()
                }
                last_n = c.to_string()
            }
        }
        n = format!("{}{}", n, last_n);
        total += n.parse::<i32>().unwrap()
    }
    println!("{}", total)
}
