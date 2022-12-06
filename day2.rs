mod utils;
use crate::utils::read_lines;

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;

const WIN: i32 = 6;
const DRAW: i32 = 3;
const LOSE: i32 = 0;

fn shape(k: char) -> i32 {
	if k == 'A' || k == 'X' {
		return ROCK;
	}
	if k == 'B' || k == 'Y' {
		return PAPER;
	}
	return SCISSORS;
}


fn figure_shape(outcome: char, s: i32) -> i32 {
	if outcome == 'X' {
		if s == ROCK {
			return SCISSORS;
		}
		if s == PAPER {
			return ROCK;
		}
		return PAPER;
	}
	if outcome == 'Z' {
		if s == ROCK {
			return PAPER;
		}
		if s == PAPER {
			return SCISSORS;
		}
		return ROCK;
	}
	return s;
}

fn result(comparison: i32) -> i32 {
	if comparison == 1 || comparison == -2 {
		return WIN;
	}

	if comparison == 0 {
		return DRAW;
	}

	return LOSE;
}

fn score(p1: i32, p2: i32) -> i32 {
	return result(p2-p1)+p2;
}

fn main() {
	let lines = read_lines();
	let mut total_score = 0;
	let mut total_score_2 = 0;

	for line in lines {
		let p1 = line.chars().nth(0).unwrap();
		let p2 = line.chars().nth(2).unwrap();

		total_score += score(shape(p1), shape(p2));
		total_score_2 += score(shape(p1), figure_shape(p2, shape(p1)));

	}
	println!("{}", total_score);
	println!("{}", total_score_2);
}