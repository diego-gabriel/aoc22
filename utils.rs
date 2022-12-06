use std::io::Read;

pub fn read_lines() -> Vec<String> {
	let mut file = std::fs::File::open("in.in").unwrap();
	let mut file_content = String::new();

	file.read_to_string(&mut file_content).unwrap();

	return file_content.split("\n").map(|line| line.to_string()).collect();
}