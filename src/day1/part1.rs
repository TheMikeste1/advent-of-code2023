pub fn answer(input: &str) -> String {
    input
        .lines()
        .map(extract_calibration_number)
        .sum::<i32>()
        .to_string()
}

fn extract_calibration_number(string: &str) -> i32 {
    let first_char = string.chars().find(|&c| c.is_numeric()).unwrap();
    let second_char = string.chars().rev().find(|&c| c.is_numeric()).unwrap();
    format!("{first_char}{second_char}").parse::<i32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        assert_eq!(answer(input), "142");
    }

    #[test]
    fn test_extract_calibration_number() {
        assert_eq!(extract_calibration_number("1abc2"), 12);
        assert_eq!(extract_calibration_number("pqr3stu8vwx"), 38);
        assert_eq!(extract_calibration_number("a1b2c3d4e5f"), 15);
        assert_eq!(extract_calibration_number("treb7uchet"), 77);
    }
}
