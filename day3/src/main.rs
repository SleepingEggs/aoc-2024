use aocutils::timer;
use aocutils::error::PuzzleError;
use aocutils::input::get_puzzle_input;
use regex::Regex;

fn main() -> Result<(), PuzzleError> {
    let _timer = timer::Timer::new();
    let my_regex = Regex::new(r"mul\((?<first_num>[0-9]{1,3}),(?<second_num>[0-9]{1,3})\)").unwrap();
    let lines = get_puzzle_input()?;

    let mut result = 0;
    for line in &lines {
        result += my_regex
            .captures_iter(line)
            .map(|caps| {
                let left = caps.name("first_num").unwrap().as_str().parse::<i64>().unwrap();
                let right = caps.name("second_num").unwrap().as_str().parse::<i64>().unwrap();
                left * right
            })
            .fold(0, |acc, mul| acc + mul);
    }

    println!("result {}", result);
    Ok(())
}
