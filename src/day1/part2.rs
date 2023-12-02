use std::collections::HashMap;

pub fn answer(input: &str) -> String {
    input
        .lines()
        .map(extract_calibration_number)
        .sum::<i32>()
        .to_string()
}

fn extract_calibration_number(string: &str) -> i32 {
    let (first, _) = find_number(string).unwrap();

    for i in (0..string.len()).rev() {
        if let Some((second, _)) = find_number(&string[i..]) {
            return format!("{first}{second}").parse::<i32>().unwrap();
        }
    }

    unreachable!()
}

fn find_number(string: &str) -> Option<(char, usize)> {
    // Create a map from word to number, such as "one" -> '1'
    let words_to_number = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .enumerate()
    .map(|(i, w)| (*w, (i + 1).to_string().chars().next().unwrap()))
    .collect::<HashMap<&str, char>>();

    for (i, c) in string.chars().enumerate() {
        if c.is_numeric() {
            return Some((c, i));
        }

        let substring = string.get(i..).unwrap_or_default();
        if let Some(number) = words_to_number
            .keys()
            .find(|word| substring.starts_with(*word))
            .and_then(|word| words_to_number.get(word))
        {
            return Some((*number, i));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
        assert_eq!(answer(input), "281");
    }

    #[test]
    fn test_extract_calibration_number() {
        assert_eq!(extract_calibration_number("two1nine"), 29);
        assert_eq!(extract_calibration_number("eightwothree"), 83);
        assert_eq!(extract_calibration_number("abcone2threexyz"), 13);
        assert_eq!(extract_calibration_number("xtwone3four"), 24);
        assert_eq!(extract_calibration_number("4nineeightseven2"), 42);
        assert_eq!(extract_calibration_number("zoneight234"), 14);
        assert_eq!(extract_calibration_number("7pqrstsixteen"), 76);
    }
}
