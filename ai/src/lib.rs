use client::Client;
use rand::Rng;

pub struct Agent {}

impl Client for Agent {
	fn set_up(&mut self, _config: client::GameConfig) {}
	fn make_turn(&mut self, turn_info: client::TurnInfo) -> Vec<client::Order> {
		let mut rng = rand::thread_rng();
		turn_info
			.ant
			.iter()
			.filter(|a| a.id == 0)
			.map(|a| {
				let direction = match rng.gen_range(0, 3) {
					0 => client::Direction::N,
					1 => client::Direction::E,
					2 => client::Direction::S,
					_ => client::Direction::W,
				};
				client::Order {
					pos: a.pos,
					direction,
				}
			})
			.collect()
	}
	fn tear_down(&mut self, _end_info: client::EndInfo) {}
}

#[cfg(test)]
#[macro_use]
extern crate assert_matches;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn when_make_turn_then_return_order_for_every_own_ant() {
		let mut agent = Agent {};
		let turn_info = client::TurnInfo {
			water: vec![],
			food: vec![],
			ant_hill: vec![],
			ant: vec![
				client::Player {
					id: 0,
					pos: client::Position { row: 1, col: 2 },
				},
				client::Player {
					id: 1,
					pos: client::Position { row: 4, col: 5 },
				},
				client::Player {
					id: 0,
					pos: client::Position { row: 6, col: 7 },
				},
			],
			dead_ant: vec![],
		};

		let mut orders = agent.make_turn(turn_info).into_iter();
		assert_matches!(
			orders.next(),
			Some(client::Order {
				pos: client::Position { row: 1, col: 2 },
				..
			})
		);
	}
}
