#![allow(unused_variables)]
#![allow(dead_code)]

mod days;
mod parser;

use anyhow::Result;

#[allow(dead_code)]
fn main() -> Result<()> {
    // day1()?;
    // day2()?;
    // day3()?;
    // day4()?;
    // day5()?;
    // day6()?;
    // day7()?;
    // day8()?;
    // day9()?;
    day10()?;
    Ok(())
}

#[allow(dead_code)]
fn day10() -> Result<()> {
    let test_input = parser::parse_input("inputs/10/test.txt")?;
    let test_input = parser::remove_empty_elements(test_input);
    let prod_input = parser::parse_input("inputs/10/prod.txt")?;
    let prod_input = parser::remove_empty_elements(prod_input);

    // part1
    days::ten::part1::run(&test_input);
    // days::ten::part1::run(&prod_input);

    // part2
    // days::ten::part2::run(&test_input);
    // days::ten::part2::run(&prod_input);

    Ok(())
}

#[allow(dead_code)]
fn day9() -> Result<()> {
    let test_input = parser::parse_input("inputs/9/test.txt")?;
    let test_input = parser::remove_empty_elements(test_input);
    let prod_input = parser::parse_input("inputs/9/prod.txt")?;
    let prod_input = parser::remove_empty_elements(prod_input);

    // part1
    // days::nine::part1::run(&test_input);
    // days::nine::part1::run(&prod_input);

    // part2
    days::nine::part2::run(&test_input);
    days::nine::part2::run(&prod_input);

    Ok(())
}

#[allow(dead_code)]
fn day8() -> Result<()> {
    let test_input = parser::parse_input("inputs/8/test.txt")?;
    let test_input = parser::remove_empty_elements(test_input);
    let test2_input = parser::parse_input("inputs/8/test2.txt")?;
    let test2_input = parser::remove_empty_elements(test2_input);
    let prod_input = parser::parse_input("inputs/8/prod.txt")?;
    let prod_input = parser::remove_empty_elements(prod_input);

    // part1
    // days::eight::part1::run(&test_input);
    // days::eight::part1::run(&prod_input);

    // part2
    days::eight::part2::run(&test2_input);
    days::eight::part2::run(&prod_input);

    Ok(())
}

#[allow(dead_code)]
fn day7() -> Result<()> {
    let test_input = parser::parse_input("inputs/7/test.txt")?;
    let test_input = parser::remove_empty_elements(test_input);
    let prod_input = parser::parse_input("inputs/7/prod.txt")?;
    let prod_input = parser::remove_empty_elements(prod_input);

    // part1
    // days::seven::part1::run(&test_input);
    // days::seven::part1::run(&prod_input);

    // part2
    days::seven::part2::run(&test_input);
    days::seven::part2::run(&prod_input);

    Ok(())
}

#[allow(dead_code)]
fn day6() -> Result<()> {
    let test_input = parser::parse_input("inputs/6/test.txt")?;
    let test_input = parser::remove_empty_elements(test_input);
    let prod_input = parser::parse_input("inputs/6/prod.txt")?;
    let prod_input = parser::remove_empty_elements(prod_input);

    // part1
    // days::six::part1::run(&test_input);
    // days::six::part1::run(&prod_input);

    // part2
    days::six::part2::run(&test_input);
    days::six::part2::run(&prod_input);

    Ok(())
}

#[allow(dead_code)]
fn day5() -> Result<()> {
    let test_input = parser::parse_input("inputs/5/test.txt")?;
    // let test_input = parser::remove_empty_elements(test_input);
    let prod_input = parser::parse_input("inputs/5/prod.txt")?;
    // let prod_input = parser::remove_empty_elements(prod_input);

    // part1
    // days::five::part1::run(&test_input);
    // days::five::part1::run(&prod_input);

    // part2
    // days::five::part2::run(&test_input);
    days::five::part2::run(&prod_input);

    Ok(())
}

#[allow(dead_code)]
fn day4() -> Result<()> {
    let test_input = parser::parse_input("inputs/4/test.txt")?;
    let test_input = parser::remove_empty_elements(test_input);
    let prod_input = parser::parse_input("inputs/4/prod.txt")?;
    let prod_input = parser::remove_empty_elements(prod_input);

    // part1
    // days::four::part1::run(&test_input);
    // days::four::part1::run(&prod_input);

    // part2
    days::four::part2::run(&test_input);
    days::four::part2::run(&prod_input);

    Ok(())
}

#[allow(dead_code)]
fn day3() -> Result<()> {
    let test_input = parser::parse_input("inputs/3/test.txt")?;
    let test_input = parser::remove_empty_elements(test_input);
    let test2_input = parser::parse_input("inputs/3/test2.txt")?;
    let test2_input = parser::remove_empty_elements(test2_input);
    let prod_input = parser::parse_input("inputs/3/prod.txt")?;
    let prod_input = parser::remove_empty_elements(prod_input);

    // part1
    // days::three::part1::run(&test_input);
    // days::three::part1::run(&prod_input);

    // part2
    days::three::part2::run(&test_input);
    days::three::part2::run(&test2_input);
    days::three::part2::run(&prod_input);

    Ok(())
}

#[allow(dead_code)]
fn day2() -> Result<()> {
    let test_input = parser::parse_input("inputs/2/test.txt")?;
    let test_input = parser::remove_empty_elements(test_input);
    let prod_input = parser::parse_input("inputs/2/prod.txt")?;
    let prod_input = parser::remove_empty_elements(prod_input);

    // part1
    days::two::part1::run(&test_input);
    days::two::part1::run(&prod_input);

    // part2
    days::two::part2::run(&test_input);
    days::two::part2::run(&prod_input);

    Ok(())
}

#[allow(dead_code)]
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
