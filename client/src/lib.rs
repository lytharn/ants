mod parser;

pub use parser::Error;
pub use parser::GameConfig;

pub trait Client {
	fn set_up(&mut self, config: Result<GameConfig, parser::Error>);
}

pub fn run<T: AsRef<str>>(
	client: &mut impl Client,
	input: impl Iterator<Item = T>,
	mut output: impl FnMut(&str) -> (),
) {
	let mut input = input.skip_while(|line| line.as_ref() != "turn 0");
	client.set_up(parser::extract_game_config(&mut input));
	output("go");
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestClient {}

	impl Client for TestClient {
		fn set_up(&mut self, _config: Result<GameConfig, parser::Error>) {}
	}

	#[test]
	fn when_turn_0_then_run_set_up_and_return_go() {
		let mut output_str = String::new();
		let output = |o: &str| {
			output_str = o.to_string();
		};
		let mut client = TestClient {};
		let input = vec![
			"turn 0",
			"loadtime 3000",
			"turntime 1000",
			"rows 20",
			"cols 30",
			"turns 500",
			"viewradius2 55",
			"attackradius2 5",
			"spawnradius2 1",
			"player_seed 42",
			"ready",
		];

		run(&mut client, input.iter(), output);

		assert_eq!(output_str, "go");
	}
}
