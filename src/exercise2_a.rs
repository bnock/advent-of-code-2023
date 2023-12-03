use std::error::Error;
use std::fs;

pub fn run(input_file: &str) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(input_file)?;

    let mut total: u32 = 0;

    for line in content.lines() {
        let game = Game::parse(line)?;

        if game.is_possible(12, 13, 14) {
            total += game.game_number;
        }
    }

    println!("The total is {total}");

    Ok(())
}

struct Game {
    game_number: u32,
    draws: Vec<Draw>,
}

impl Game {
    fn parse(input: &str) -> Result<Game, &'static str> {
        let mut pieces = input.split(":");
        let game_info = pieces.next().unwrap();
        let game = pieces.next().unwrap();

        let mut pieces = game_info.trim().split(" ");
        pieces.next();

        let game_number = pieces.next().unwrap();
        let game_number: u32 = game_number.parse().unwrap();

        let mut draws = Vec::new();

        for draw in game.split(";") {
            if let Ok(draw) = Draw::parse(draw) {
                draws.push(draw);
            }
        }

        Ok(Game{ game_number, draws })
    }

    fn is_possible(&self, red: u32, green: u32, blue: u32) -> bool {
        for draw in &self.draws {
            if !draw.is_possible(red, green, blue) {
                return false
            }
        }

        true
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

    fn is_possible(&self, red: u32, green: u32, blue: u32) -> bool {
        self.red <= red && self.green <= green && self.blue <= blue
    }
}