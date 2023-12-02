mod days;
mod parser;

use anyhow::Result;

fn main() -> Result<()> {
    day1()?;
    day2()?;
    Ok(())
}

fn day2() -> Result<()> {
    let test_input = parser::parse_input("inputs/2/test.txt")?;
    let test_input = parser::remove_empty_elements(test_input);
    // let prod_input = parser::parse_input("inputs/2/prod.txt")?;
    // let prod_input = parser::remove_empty_elements(prod_input);

    // part1
    days::two::part1::run(&test_input);
    // days::two::part1::run(&prod_input);

    // part2
    // let test_input = parser::parse_input("inputs/2/test.txt")?;
    // let test_input = parser::remove_empty_elements(test_input);
    // days::two::part2::run(&test_input);
    // days::two::part2::run(&prod_input);

    Ok(())
}

fn day1() -> Result<()> {
    // let test_input = parser::parse_input("inputs/1/test.txt")?;
    // let test_input = parser::remove_empty_elements(test_input);
    let prod_input = parser::parse_input("inputs/1/prod.txt")?;
    let prod_input = parser::remove_empty_elements(prod_input);

    // part1
    // days::one::part1::run(&test_input);
    // days::one::part1::run(&prod_input);

    // part2
    let test_input = parser::parse_input("inputs/1/test2.txt")?;
    let test_input = parser::remove_empty_elements(test_input);
    days::one::part2::run(&test_input);
    days::one::part2::run(&prod_input);

    Ok(())
}
