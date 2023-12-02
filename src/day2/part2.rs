use crate::color::Color;
use crate::part1::parse_line;

pub fn answer(input: &str) -> String {
    let games = input.split('\n').map(parse_line);
    games
        .map(|(_, colors)| -> u64 {
            let mut highest_red: i32 = i32::MIN;
            let mut highest_green: i32 = i32::MIN;
            let mut highest_blue: i32 = i32::MIN;

            for color in colors {
                match color {
                    Color::Red(x) if x > highest_red => {
                        highest_red = x;
                    }
                    Color::Green(x) if x > highest_green => {
                        highest_green = x;
                    }
                    Color::Blue(x) if x > highest_blue => {
                        highest_blue = x;
                    }
                    _ => {}
                }
            }
            highest_red as u64 * highest_green as u64 * highest_blue as u64
        })
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        assert_eq!(answer(input), "2286");
    }
}
