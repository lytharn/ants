mod parser;
mod unparser;

pub use parser::EndInfo;
pub use parser::Error;
pub use parser::GameConfig;
pub use parser::TurnInfo;
pub use unparser::Direction;
pub use unparser::Order;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
	row: i32,
	col: i32,
}

pub trait Client {
	fn set_up(&mut self, config: Result<GameConfig, Error>);
	fn make_turn(&mut self, turn_info: TurnInfo) -> Vec<Order>;
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
		if line.as_ref() == "end" {
			client.tear_down(parser::extract_end_info(&mut input));
			return;
		} else if line.as_ref().starts_with("turn ") {
			let orders = client.make_turn(parser::extract_turn_info(&mut input));
			unparser::output_orders(orders, &mut output);
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
		orders: Vec<Order>,
	}

	impl<'a> Client for TestClient<'a> {
		fn set_up(&mut self, config: Result<GameConfig, Error>) {
			self.callbacks.borrow_mut().push(Callback::SetUp(config));
		}

		fn make_turn(&mut self, turn_info: TurnInfo) -> Vec<Order> {
			self.callbacks
				.borrow_mut()
				.push(Callback::MakeTurn(turn_info));
			self.orders.clone()
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
			orders: vec![
				Order::new(12, 34, Direction::N),
				Order::new(56, 78, Direction::W),
			],
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
		let mut calls = calls.iter();
		assert_matches!(calls.next(), Some(Callback::SetUp(Ok(_))));
		assert_matches!(&calls.next(), Some(Callback::Output(s)) if s == "go");
		assert_matches!(calls.next(), Some(Callback::MakeTurn(_)));
		assert_matches!(&calls.next(), Some(Callback::Output(s)) if s == "12 34 N");
		assert_matches!(&calls.next(), Some(Callback::Output(s)) if s == "56 78 W");
		assert_matches!(&calls.next(), Some(Callback::Output(s)) if s == "go");
		assert_matches!(calls.next(), Some(Callback::MakeTurn(_)));
		assert_matches!(&calls.next(), Some(Callback::Output(s)) if s == "12 34 N");
		assert_matches!(&calls.next(), Some(Callback::Output(s)) if s == "56 78 W");
		assert_matches!(&calls.next(), Some(Callback::Output(s)) if s == "go");
		assert_matches!(calls.next(), Some(Callback::TearDown(Ok(_))));
	}

	#[test]
	fn given_input_after_end_input_when_run_then_ignore_input_after_end_input() {
		let callbacks = RefCell::new(vec![]);
		let mut client = TestClient {
			callbacks: &callbacks,
			orders: vec![],
		};
		let setup = Setup::new();

		run(
			&mut client,
			setup
				.config_input
				.iter()
				.chain(setup.end_input.iter())
				.chain(iter::once(&"turn 1"))
				.chain(setup.turn_input.iter()),
			|_| {},
		);

		let make_turn_called = callbacks.borrow().iter().any(|c| {
			if let Callback::MakeTurn(_) = c {
				true
			} else {
				false
			}
		});
		assert_eq!(make_turn_called, false);
	}

	#[test]
	fn given_invalid_input_when_run_then_ignore_invalid_input() {
		let callbacks = RefCell::new(vec![]);
		let mut client = TestClient {
			callbacks: &callbacks,
			orders: vec![],
		};
		let setup = Setup::new();

		run(
			&mut client,
			iter::once(&"INVALID INPUT")
				.chain(setup.config_input.iter())
				.chain(iter::once(&"INVALID INPUT"))
				.chain(setup.end_input.iter()),
			|_| {},
		);

		let calls = callbacks.borrow();
		assert_matches!(calls[0], Callback::SetUp(Ok(_)));
		assert_matches!(calls[1], Callback::TearDown(Ok(_)));
	}
}
