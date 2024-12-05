use aocutils::timer;
use aocutils::error::PuzzleError;
use aocutils::input::get_puzzle_input;

fn process_line(line: &Vec<i32>) -> bool {
    let mut previous_value = &0;
    let mut previous_diff = 0;
    for (index, value) in line.iter().enumerate() {
        if index == 0 {
            previous_value = value;
            previous_diff = 0;
            continue;
        }
        let new_diff = previous_value - value;
        if new_diff == 0 || new_diff.abs() > 3 {
            return false
        }
        if (new_diff < 0 && previous_diff > 0) || (new_diff > 0 && previous_diff < 0) {
            return false
        }
        previous_diff = new_diff;
        previous_value = value;
    }
    return true
}
fn main() -> Result<(), PuzzleError> {
    let _timer = timer::Timer::new();
    let lines = get_puzzle_input()?;
    let int_lines: Vec<Vec<i32>> = lines.into_iter().map(|x| x.split_ascii_whitespace().map(|y| y.parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect();
    let mut result = 0;
    for int_line in &int_lines {
        let  is_good = process_line(&int_line);
        if is_good {
            result += 1;
        }
        else if cfg!(feature = "part2") {
            for index in 0..int_line.len() {
                let mut new_line = int_line.clone();
                new_line.remove(index);
                let is_good_2 = process_line(&new_line);
                if is_good_2 {
                    result += 1;
                    break;
                }
            }
        }
    }
    println!("result {}", result);

    Ok(())
}
