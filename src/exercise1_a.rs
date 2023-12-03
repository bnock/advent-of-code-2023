use regex::Regex;
use std::error::Error;
use std::fs;

pub fn run(input_file: &str) -> Result<(), Box<dyn Error>> {
    let re = Regex::new(r"[^0-9]").unwrap();

    let contents = fs::read_to_string(input_file)?;

    let mut total = 0;

    for line in contents.lines() {
        let digits = re.replace_all(&line, "");

        let mut to_add: u32;

        if digits.len() == 1 {
            to_add = digits.parse()?;
            to_add *= 11;
        } else {
            let mut number_string = String::new();
            number_string.push(digits.chars().nth(0).unwrap());
            number_string.push(digits.chars().nth(digits.len() - 1).unwrap());

            to_add = number_string.parse()?;
        }

        total += to_add;
    }

    println!("The total is {total}");

    Ok(())
}