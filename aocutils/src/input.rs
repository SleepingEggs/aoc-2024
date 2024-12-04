use crate::error::PuzzleError;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

pub fn get_puzzle_input() -> Result<Vec<String>, PuzzleError> {
    let args: Vec<String> = env::args().collect();
    let filename = &args.get(1).ok_or_else(|| PuzzleError)?;
    let file = File::open(filename).map_err(|_| PuzzleError)?;
    let lines: Result<Vec<String>, PuzzleError> = BufReader::new(file)
                                .lines()
                                .map(|x| x.map_err(|_| PuzzleError))
                                .collect();
    return lines
}