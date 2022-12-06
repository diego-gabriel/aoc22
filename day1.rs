mod utils;
use crate::utils::read_lines;

fn main() {
	let lines = read_lines();
	let mut current_calories = 0;
	let mut elves = Vec::new();

	for line in lines {
		if line != "" {
			let calories: i32 = line.parse().unwrap();
			current_calories += calories;
		} else {
			elves.push(current_calories);
			current_calories = 0;
		}
	}

	elves.sort();
	elves.reverse();

	println!("{}", elves[0]);
	println!("{}", elves[0..=2].iter().sum::<i32>());
}