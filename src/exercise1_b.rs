use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub fn run(input_file: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(input_file)?;

    let mut total = 0;

    for line in contents.lines() {
        let first = find_first_number(&line)?;
        let last = find_last_number(&line)?;

        let mut to_add = String::new();
        to_add.push(first);
        to_add.push(last);

        let to_add: u32 = to_add.parse()?;

        total += to_add;
    }

    println!("Total is {total}");

    Ok(())
}

fn find_first_number(string: &str) -> Result<char, &'static str> {
    for (index, c) in string.chars().enumerate() {
        if let Some(number) = check_digit(c) {
            return Ok(number);
        } else {
            if let Some(number) = check_textual_number(&string[0..index + 1]) {
                return Ok(number);
            } else {
                continue;
            }
        }
    }

    Err("No number result found")
}

fn find_last_number(string: &str) -> Result<char, &'static str> {
    for (index, c) in string.chars().rev().enumerate() {
        if let Some(number) = check_digit(c) {
            return Ok(number);
        } else {
            if let Some(number) = check_textual_number(&string[string.len() - index - 1..string.len()]) {
                return Ok(number);
            } else {
                continue;
            }
        }
    }

    Err("No number result found")
}

fn check_digit(value: char) -> Option<char> {
    match value.to_string().parse::<u32>() {
        Ok(_) => Some(value),
        _ => None,
    }
}

fn check_textual_number(value: &str) -> Option<char> {
    let map = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    for (k, v) in map.iter() {
        if value.contains(k) {
            return Some(*v)
        }
    }

    None
}