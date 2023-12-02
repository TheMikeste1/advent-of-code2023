mod part1;
mod part2;

fn main() {
    let input = include_str!("./input.txt").trim_end();
    let part1_answer = part1::answer(input);
    println!("Part 1: {part1_answer}");
    let part2_answer = part2::answer(input);
    println!("Part 2: {part2_answer}");
}
