use std::io;
use std::io::prelude::*;

fn main() {
	let mut agent = ai::Agent {};
	let stdin = io::stdin();
	let stdin_iter = stdin.lock().lines().map(|l| l.unwrap());

	client::run(&mut agent, stdin_iter, |output| print!("{}", output)).unwrap();
}
