use super::Position;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
	N,
	E,
	S,
	W,
}

#[derive(Debug, Copy, Clone)]
pub struct Order {
	pos: Position,
	direction: Direction,
}

impl Order {
	pub fn new(row: i32, col: i32, direction: Direction) -> Self {
		Self {
			pos: Position { row, col },
			direction,
		}
	}
}

pub fn output_orders(orders: Vec<Order>, output: &mut impl FnMut(&str) -> ()) {
	orders.iter().for_each(|o| {
		output(
			format!(
				"{} {} {}",
				o.pos.row.to_string(),
				o.pos.col.to_string(),
				unparse_direction(o.direction),
			)
			.as_str(),
		);
	});
	output("go");
}

fn unparse_direction(direction: Direction) -> char {
	match direction {
		Direction::N => 'N',
		Direction::E => 'E',
		Direction::S => 'S',
		Direction::W => 'W',
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn given_orders_when_output_orders_then_return_orders_as_a_string() {
		let orders = vec![
			Order {
				pos: Position { row: 0, col: 0 },
				direction: Direction::N,
			},
			Order {
				pos: Position { row: 0, col: 1 },
				direction: Direction::E,
			},
			Order {
				pos: Position { row: 1, col: 0 },
				direction: Direction::S,
			},
			Order {
				pos: Position { row: 42, col: 32 },
				direction: Direction::W,
			},
		];
		let mut outputs = Vec::new();
		let mut save_output = |o: &str| outputs.push(o.to_string());

		output_orders(orders, &mut save_output);

		assert_eq!(outputs[0].as_str(), "0 0 N");
		assert_eq!(outputs[1].as_str(), "0 1 E");
		assert_eq!(outputs[2].as_str(), "1 0 S");
		assert_eq!(outputs[3].as_str(), "42 32 W");
		assert_eq!(outputs[4].as_str(), "go");
	}
}
