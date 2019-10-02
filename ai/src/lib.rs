use client::Client;

pub struct Agent {}

impl Client for Agent {
	fn set_up(&mut self, _config: Result<client::GameConfig, client::Error>) {}
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
