#[derive(Debug)]
pub enum Color {
    Red(i32),
    Green(i32),
    Blue(i32),
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Color::Red(x), Color::Red(y))
            | (Color::Green(x), Color::Green(y))
            | (Color::Blue(x), Color::Blue(y)) => x == y,
            _ => false,
        }
    }
}
