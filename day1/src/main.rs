use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::env;
use aocutils::timer;

fn main() -> Result<()> {
    let _timer = timer::Timer::new();
    let args: Vec<String> = env::args().collect();
    let filename = &args.get(1).expect("Please provide one argument");
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    let  (mut left_list, mut right_list): (Vec<i32>, Vec<i32>) = lines.map(|x| x.unwrap()
        .to_string()
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|y| y.parse::<i32>().unwrap())
        .collect()
    ).map(|x: Vec<i32>| (x.get(0).unwrap().clone(), x.get(1).unwrap().clone())).unzip();

    if cfg!(feature = "part2") {
        println!("not implemented yet");
        let mut my_map: HashMap<&i32, i32> = HashMap::new();
        for entry in &right_list {
            *my_map.entry(entry).or_insert(0) += 1;
        }
        let result = left_list.into_iter().fold(0, |acc, a| acc + a * my_map.get(&a).or(Some(&0)).unwrap());
        println!("result {}", result);
    }
    else {
        left_list.sort();
        right_list.sort();

        let result = left_list.into_iter().zip(right_list.into_iter()).fold(0, |acc, b| acc + (b.0 - b.1).abs());
        println!("result {}", result);
    }

    Ok(())
}
