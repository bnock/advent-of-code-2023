use std::error::Error;
use std::fs;

pub fn run(input_file: &str) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(input_file)?;

    let mut total: u32 = 0;

    for line in content.lines() {
        let game = Game::parse(line)?;

        total += game.power();
    }

    println!("The total is {total}");

    Ok(())
}

struct Game {
    draws: Vec<Draw>,
}

impl Game {
    fn parse(input: &str) -> Result<Game, &'static str> {
        let mut pieces = input.split(":");
        pieces.next();
        let game = pieces.next().unwrap();

        let mut draws = Vec::new();

        for draw in game.split(";") {
            if let Ok(draw) = Draw::parse(draw) {
                draws.push(draw);
            }
        }

        Ok(Game{ draws })
    }

    fn power(&self) -> u32 {
        let mut min_red: u32 = 0;
        let mut min_green: u32 = 0;
        let mut min_blue: u32 = 0;

        for draw in &self.draws {
            if draw.red > min_red {
                min_red = draw.red;
            }

            if draw.green > min_green {
                min_green = draw.green;
            }

            if draw.blue > min_blue {
                min_blue = draw.blue;
            }
        }

        return min_red * min_green * min_blue;
    }
}

struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

impl Draw {
    fn parse(input: &str) -> Result<Draw, &'static str> {
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;

        for color_number in input.split(",") {
            let mut pieces = color_number.trim().split(" ");

            let number = pieces.next().unwrap();
            let number: u32 = number.parse().unwrap();

            match pieces.next().unwrap() {
                "red" => red += number,
                "green" => green += number,
                "blue" => blue += number,
                _ => return Err("Unknown color"),
            }
        }

        Ok(Draw { red, green, blue })
    }
}