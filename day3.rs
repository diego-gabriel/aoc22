mod utils;
use crate::utils::read_lines;
use std::collections::HashSet;
use std::iter::FromIterator;

fn priority(k: char) -> i32 {
	let v = k as i32;
	#[allow(non_snake_case)]
	let val_A = 'A' as i32;
	let val_a = 'a' as i32;

	if v < val_a {
		return 27 + (v-val_A); 
	}

	return 1+(v - val_a);
}

fn main() {
	let lines = read_lines();
	let mut total_priorities: i32 = 0;
	let mut total_badge_priorities: i32 = 0;

	for line in &lines {
		let chars: Vec<char> = line.chars().collect();
		let mut left = HashSet::new();
		let mut right = HashSet::new();
		let size = chars.len()/2;

		for i in 0..size{
			left.insert(chars[i]);
		}
		for i in size..chars.len() {
			right.insert(chars[i]);
		}

		let intersection: Vec<char> = left.intersection(&right).cloned().collect();
		total_priorities += priority(intersection[0]);
	}

	for chunk in lines.chunks(3) {
		let line1: HashSet<char> = HashSet::from_iter(chunk[0].chars());
		let line2: HashSet<char> = HashSet::from_iter(chunk[1].chars());
		let line3: HashSet<char> = HashSet::from_iter(chunk[2].chars());

		let intersection1 = HashSet::from_iter(line1.intersection(&line2).cloned());
		let final_intersection: Vec<char> = intersection1.intersection(&line3).cloned().collect();

		total_badge_priorities += priority(final_intersection[0]);
	}

	println!("{}", total_priorities);
	println!("{}", total_badge_priorities);
}