use std::error::Error;
use std::io;
use advent;

struct Exercise {
    func: fn(&str) -> Result<(), Box<dyn Error>>,
    label: &'static str,
    input_file: &'static str,
}

fn main() -> Result<(), Box<dyn Error>> {
    let exercises = vec![
        Exercise { func: advent::exercise1_a::run, label: "Exercise 1a", input_file: "input_1.txt" },
        Exercise { func: advent::exercise1_b::run, label: "Exercise 1b", input_file: "input_1.txt" },
    ];

    println!("Choose an exercise to run:");

    for (index, exercise) in exercises.iter().enumerate() {
        let label = exercise.label;
        println!("{index}. {label}");
    }

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    let choice: usize = choice.trim().parse()?;

    if let Some(exercise) = exercises.get(choice) {
        (exercise.func)(&format!("inputs/{}", exercise.input_file)[..])?;
    }

    Ok(())
}