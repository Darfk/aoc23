#[derive(Debug)]
struct Draw {
    red: u32,
    blue: u32,
    green: u32,
}

#[derive(Debug)]
struct Game {
    number: u32,
    draws: Vec<Draw>,
}

impl Game {
    fn from_line(line: &str) -> Self {
        let (game_str, draws_str) = line.split_once(":").unwrap();
        let game_number: u32 = game_str
            .chars()
            .skip("Game ".len())
            .collect::<String>()
            .parse()
            .unwrap();
        let draw_strs = draws_str.trim().split(";").map(str::trim);

        let mut draws = Vec::<Draw>::new();

        for draw_str in draw_strs {
            let count_colours = draw_str.split(",").map(str::trim).collect::<Vec<&str>>();
            let (mut red, mut green, mut blue): (u32, u32, u32) = (0, 0, 0);
            for count_colour in count_colours {
                let (count, colour) = count_colour.split_once(" ").unwrap();
                match colour {
                    "red" => red += count.parse::<u32>().unwrap(),
                    "green" => green += count.parse::<u32>().unwrap(),
                    "blue" => blue += count.parse::<u32>().unwrap(),
                    _ => panic!("unknown colour"),
                }
            }
            draws.push(Draw { red, blue, green });
        }

        Game {
            number: game_number,
            draws,
        }
    }
}

pub fn part1() -> u32 {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;

    let lines = advent::read_input_lines("input/day2/input.txt");
    let games = lines
        .iter()
        .map(|line| Game::from_line(line))
        .collect::<Vec<_>>();
    
    let mut solution = 0u32;

    for game in games {
        let possible = game
            .draws
            .iter()
            .all(|draw| draw.red <= MAX_RED && draw.green <= MAX_GREEN && draw.blue <= MAX_BLUE);
        if possible {
            solution += game.number;
        }
    }
    return solution;
}

pub fn part2() -> u32 {
    let lines = advent::read_input_lines("input/day2/input.txt");
    let games = lines
        .iter()
        .map(|line| Game::from_line(line))
        .collect::<Vec<_>>();
    
    let mut solution = 0u32;

    for game in games {
        let max_red = game.draws.iter().map(|draw| draw.red).max().unwrap();
        let max_green = game.draws.iter().map(|draw| draw.green).max().unwrap();
        let max_blue = game.draws.iter().map(|draw| draw.blue).max().unwrap();

        solution += max_red * max_green * max_blue;
    }

    return solution;
}

#[cfg(test)]
mod tests {
    use super::Game;

    #[test]
    fn test_read_games() {
        let example_input = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let games = example_input
            .lines()
            .map(Game::from_line)
            .collect::<Vec<_>>();

        dbg!(games);
    }
}
