mod parser;

pub use parser::EndInfo;
pub use parser::Error;
pub use parser::GameConfig;
pub use parser::TurnInfo;

pub trait Client {
	fn set_up(&mut self, config: Result<GameConfig, Error>);
	fn make_turn(&mut self, turn_info: TurnInfo);
	fn tear_down(&mut self, end_info: Result<EndInfo, Error>);
}

pub fn run<T: AsRef<str>>(
	client: &mut impl Client,
	input: impl Iterator<Item = T>,
	mut output: impl FnMut(&str) -> (),
) {
	let mut input = input.skip_while(|line| line.as_ref() != "turn 0");
	client.set_up(parser::extract_game_config(&mut input));
	output("go");
	while let Some(line) = input.next() {
		match line.as_ref() {
			"end" => client.tear_down(parser::extract_end_info(&mut input)),
			_ => {
				client.make_turn(parser::extract_turn_info(&mut input));
				output("go")
			}
		}
	}
}

#[cfg(test)]
#[macro_use]
extern crate assert_matches;

#[cfg(test)]
mod tests {
	use super::*;
	use std::cell::RefCell;
	use std::iter;

	struct Setup {
		config_input: Vec<&'static str>,
		turn_input: Vec<&'static str>,
		end_input: Vec<&'static str>,
	}

	impl Setup {
		fn new() -> Self {
			Self {
				config_input: vec![
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
				],
				turn_input: vec!["f 6 5", "w 7 6", "a 10 9 0", "h 7 12 0", "go"],
				end_input: vec![
					"end",
					"players 2",
					"score 11 12",
					"f 6 5",
					"d 7 8 1",
					"a 9 9 0",
					"go",
				],
			}
		}
	}

	#[derive(Debug)]
	enum Callback {
		SetUp(Result<GameConfig, Error>),
		MakeTurn(TurnInfo),
		TearDown(Result<EndInfo, Error>),
		Output(String),
	}

	struct TestClient<'a> {
		callbacks: &'a RefCell<Vec<Callback>>,
	}

	impl<'a> Client for TestClient<'a> {
		fn set_up(&mut self, config: Result<GameConfig, Error>) {
			self.callbacks.borrow_mut().push(Callback::SetUp(config));
		}

		fn make_turn(&mut self, turn_info: TurnInfo) {
			self.callbacks
				.borrow_mut()
				.push(Callback::MakeTurn(turn_info));
		}

		fn tear_down(&mut self, end_info: Result<EndInfo, Error>) {
			self.callbacks
				.borrow_mut()
				.push(Callback::TearDown(end_info));
		}
	}

	#[test]
	fn given_normal_input_when_run_then_do_according_to_input() {
		let callbacks = RefCell::new(vec![]);
		let save_output = |o: &str| callbacks.borrow_mut().push(Callback::Output(o.to_string()));
		let mut client = TestClient {
			callbacks: &callbacks,
		};
		let setup = Setup::new();

		run(
			&mut client,
			setup
				.config_input
				.iter()
				.chain(iter::once(&"turn 1"))
				.chain(setup.turn_input.iter())
				.chain(iter::once(&"turn 2"))
				.chain(setup.turn_input.iter())
				.chain(setup.end_input.iter()),
			save_output,
		);

		let calls = callbacks.borrow();
		assert_matches!(calls[0], Callback::SetUp(Ok(_)));
		assert_matches!(&calls[1], Callback::Output(s) if s == "go");
		assert_matches!(calls[2], Callback::MakeTurn(_));
		assert_matches!(&calls[3], Callback::Output(s) if s == "go");
		assert_matches!(calls[4], Callback::MakeTurn(_));
		assert_matches!(&calls[5], Callback::Output(s) if s == "go");
		assert_matches!(calls[6], Callback::TearDown(Ok(_)));
	}
}
