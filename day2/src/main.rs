use aocutils::timer;
use aocutils::error::PuzzleError;
use aocutils::input::get_puzzle_input;

fn main() -> Result<(), PuzzleError> {
    let _timer = timer::Timer::new();
    let lines = get_puzzle_input()?;
    let int_lines: Vec<Vec<i32>> = lines.into_iter().map(|x| x.split_ascii_whitespace().map(|y| y.parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect();
    let mut result = 0;
    for int_line in &int_lines {
        let mut previous_value = &0;
        let mut previous_diff = 0;
        let mut is_good = true;
        for (index, value) in int_line.iter().enumerate() {
            if index == 0 {
                previous_value = value;
                previous_diff = 0;
                continue;
            }
            let new_diff = previous_value - value;
            if new_diff == 0 || new_diff.abs() > 3 {
                is_good = false;
                break;
            }
            if (new_diff < 0 && previous_diff > 0) || (new_diff > 0 && previous_diff < 0) {
                is_good = false;
                break;
            }
            previous_diff = new_diff;
            previous_value = value;
        }
        if is_good {
            result += 1;
        }
    }
    println!("result {}", result);

    Ok(())
}
