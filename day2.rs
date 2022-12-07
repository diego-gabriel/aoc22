mod utils;
use crate::utils::read_lines;

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;

const WIN: i32 = 6;
const DRAW: i32 = 3;
const LOSE: i32 = 0;

fn shape(k: char) -> i32 {
	return match k {
		'A' | 'X' => ROCK,
		'B' | 'Y' => PAPER,
		_ => SCISSORS,
	}
}

fn correct_shape(s: i32) -> i32 {
	if s == 0 {
		return SCISSORS;
	} else {
		return s;
	}
}

fn figure_shape(outcome: char, s: i32) -> i32 {
	return match outcome {
		'X' => correct_shape((s+2)%3),
		'Z' => correct_shape((s+1)%3),
		_ => s,
	}
}

fn score(p1: i32, p2: i32) -> i32 {
	let result = match p2-p1 {
		1 | -2 => WIN,
		0 => DRAW,
		_ => LOSE,
	};

	return result+p2;
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