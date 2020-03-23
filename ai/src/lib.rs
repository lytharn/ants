use client::Client;

pub struct Agent {}

impl Client for Agent {
	fn set_up(&mut self, _config: Result<client::GameConfig, client::Error>) {}
	fn make_turn(&mut self, _turn_info: client::TurnInfo) {}
	fn tear_down(&mut self, _end_info: Result<client::EndInfo, client::Error>) {}
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
