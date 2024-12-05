use aocutils::timer;
use aocutils::error::PuzzleError;
use aocutils::input::get_puzzle_input;
use regex::Regex;

fn main() -> Result<(), PuzzleError> {
    let _timer = timer::Timer::new();
    let my_regex = Regex::new(r"mul\((?<first_num>[0-9]{1,3}),(?<second_num>[0-9]{1,3})\)").unwrap();
    let lines: Vec<String> = get_puzzle_input()?;
    let mut input: String = lines.join("").to_owned();
    if cfg!(feature = "part2") {
        let (start, remaining_start) = input.split_once("don't()").unwrap();
        let (_, remaining) = remaining_start.split_once("do()").unwrap();
        let mut new_thing = start.to_string().to_owned();
        new_thing.push_str(remaining.split("do()").map(|doer| {
            let Some(_pos) = doer.find("don't()") else {
                return doer;
            };

            let (start, _) = doer.split_once("don't()").unwrap();
            start
        }).collect::<Vec<&str>>().join("").as_str());

        input = new_thing;
    }
    let mut result = 0;
    result += my_regex
        .captures_iter(input.as_str())
        .map(|caps| {
            let left_match = caps.name("first_num_a").unwrap_or_else(|| caps.name("first_num").unwrap());
            let right_match = caps.name("second_num_a").unwrap_or_else(|| caps.name("second_num").unwrap());
            let left = left_match.as_str().parse::<i64>().unwrap();
            let right = right_match.as_str().parse::<i64>().unwrap();
            left * right
        })
        .fold(0, |acc, mul| acc + mul);

    println!("result {}", result);
    Ok(())
}
