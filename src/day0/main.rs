use std::io;

fn main() {
	let mut buffer = String::new();
	let stdin = io::stdin();
	stdin.read_line(&mut buffer).expect("Failed to read line");
	println!("Hello, World.");
	println!("{}", buffer);
}
