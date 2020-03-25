use client::Client;

pub struct Agent {}

impl Client for Agent {
	fn set_up(&mut self, _config: client::GameConfig) {}
	fn make_turn(&mut self, _turn_info: client::TurnInfo) -> Vec<client::Order> {
		vec![]
	}
	fn tear_down(&mut self, _end_info: client::EndInfo) {}
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
