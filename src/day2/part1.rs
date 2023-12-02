use crate::color::Color;

pub fn answer(input: &str) -> String {
    let games = input.split('\n').map(parse_line);
    games
        .filter_map(|(id, colors)| {
            if colors.iter().all(|color| in_range(color)) {
                Some(id)
            } else {
                None
            }
        })
        .sum::<u32>()
        .to_string()
}

fn in_range(color: &Color) -> bool {
    match color {
        Color::Red(0..=12) | Color::Green(0..=13) | Color::Blue(0..=14) => true,
        _ => false,
    }
}

pub fn parse_line(line: &str) -> (u32, Vec<Color>) {
    let id: u32 = line["Game ".len()..line.find(':').unwrap()]
        .parse()
        .unwrap();
    let mut colors = vec![];
    for section in line[line.find(':').unwrap() + 2..].split(';') {
        for number_color in section.split(',') {
            let mut parts = number_color.trim().split(' ');
            let number = parts.next().unwrap().parse().unwrap();
            let color = parts.next();
            match color {
                Some("red") => colors.push(Color::Red(number)),
                Some("green") => colors.push(Color::Green(number)),
                Some("blue") => colors.push(Color::Blue(number)),
                _ => panic!("Unknown color"),
            }
        }
    }
    (id, colors)
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

        assert_eq!(answer(input), "8");
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            (
                1,
                vec![
                    Color::Blue(3),
                    Color::Red(4),
                    Color::Red(1),
                    Color::Green(2),
                    Color::Blue(6),
                    Color::Green(2)
                ]
            )
        );
        assert_eq!(
            parse_line("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            (
                2,
                vec![
                    Color::Blue(1),
                    Color::Green(2),
                    Color::Green(3),
                    Color::Blue(4),
                    Color::Red(1),
                    Color::Green(1),
                    Color::Blue(1)
                ]
            )
        );
        assert_eq!(
            parse_line("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
            (
                3,
                vec![
                    Color::Green(8),
                    Color::Blue(6),
                    Color::Red(20),
                    Color::Blue(5),
                    Color::Red(4),
                    Color::Green(13),
                    Color::Green(5),
                    Color::Red(1)
                ]
            )
        );
        assert_eq!(
            parse_line("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
            (
                4,
                vec![
                    Color::Green(1),
                    Color::Red(3),
                    Color::Blue(6),
                    Color::Green(3),
                    Color::Red(6),
                    Color::Green(3),
                    Color::Blue(15),
                    Color::Red(14)
                ]
            )
        );
        assert_eq!(
            parse_line("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
            (
                5,
                vec![
                    Color::Red(6),
                    Color::Blue(1),
                    Color::Green(3),
                    Color::Blue(2),
                    Color::Red(1),
                    Color::Green(2)
                ]
            )
        );
    }
}
