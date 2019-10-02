#[derive(Debug, PartialEq)]
pub enum Error {
	CannotParseGameConfig,
}

#[derive(Debug)]
pub struct GameConfig {
	load_time: i32,
	turn_time: i32,
	rows: i32,
	cols: i32,
	turns: i32,
	view_radius2: i32,
	attack_radius2: i32,
	food_gathering_radius2: i32,
	player_seed: i64,
}

pub fn extract_game_config<T: AsRef<str>>(
	input: &mut impl Iterator<Item = T>,
) -> Result<GameConfig, Error> {
	let mut load_time: Option<i32> = None;
	let mut turn_time: Option<i32> = None;
	let mut rows: Option<i32> = None;
	let mut cols: Option<i32> = None;
	let mut turns: Option<i32> = None;
	let mut view_radius2: Option<i32> = None;
	let mut attack_radius2: Option<i32> = None;
	let mut food_gathering_radius2: Option<i32> = None;
	let mut player_seed: Option<i64> = None;

	for line in input {
		let mut type_value = line.as_ref().split_whitespace();
		let parameter_type = type_value.next();
		let parameter_value = type_value.next();
		match parameter_type {
			Some("loadtime") => load_time = parameter_value.and_then(|v| v.parse().ok()),
			Some("turntime") => turn_time = parameter_value.and_then(|v| v.parse().ok()),
			Some("rows") => rows = parameter_value.and_then(|v| v.parse().ok()),
			Some("cols") => cols = parameter_value.and_then(|v| v.parse().ok()),
			Some("turns") => turns = parameter_value.and_then(|v| v.parse().ok()),
			Some("viewradius2") => view_radius2 = parameter_value.and_then(|v| v.parse().ok()),
			Some("attackradius2") => attack_radius2 = parameter_value.and_then(|v| v.parse().ok()),
			Some("spawnradius2") => {
				food_gathering_radius2 = parameter_value.and_then(|v| v.parse().ok())
			}
			Some("player_seed") => player_seed = parameter_value.and_then(|v| v.parse().ok()),
			Some("ready") => break,
			_ => (),
		}
	}
	match (
		load_time,
		turn_time,
		rows,
		cols,
		turns,
		view_radius2,
		attack_radius2,
		food_gathering_radius2,
		player_seed,
	) {
		(
			Some(load_time),
			Some(turn_time),
			Some(rows),
			Some(cols),
			Some(turns),
			Some(view_radius2),
			Some(attack_radius2),
			Some(food_gathering_radius2),
			Some(player_seed),
		) => Ok(GameConfig {
			load_time,
			turn_time,
			rows,
			cols,
			turns,
			view_radius2,
			attack_radius2,
			food_gathering_radius2,
			player_seed,
		}),
		_ => Err(Error::CannotParseGameConfig),
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	struct Setup {
		input: Vec<&'static str>,
	}

	impl Setup {
		fn new() -> Self {
			Self {
				input: vec![
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
			}
		}
	}

	#[test]
	fn given_correct_input_when_extract_game_config_then_return_game_config() {
		let setup = Setup::new();

		let config = extract_game_config(&mut setup.input.iter()).unwrap();

		assert_eq!(config.load_time, 3000);
		assert_eq!(config.turn_time, 1000);
		assert_eq!(config.rows, 20);
		assert_eq!(config.cols, 30);
		assert_eq!(config.turns, 500);
		assert_eq!(config.view_radius2, 55);
		assert_eq!(config.attack_radius2, 5);
		assert_eq!(config.food_gathering_radius2, 1);
		assert_eq!(config.player_seed, 42);
	}

	#[test]
	fn given_input_when_extract_game_config_then_read_iterator_up_to_ready_string() {
		let input = vec!["before ready", "ready", "after ready"];
		let mut iter = input.iter();

		let _ = extract_game_config(&mut iter);

		assert_eq!(Some(&"after ready"), iter.next());
	}

	macro_rules! missing_parameter_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
		let setup = Setup::new();
		let mut iter = setup.input.iter().filter(|&&l| l.contains($value));

		let result = extract_game_config(&mut iter).unwrap_err();

		assert_eq!(Error::CannotParseGameConfig, result);
        }
    )*
    }
	}

	missing_parameter_tests! {
		given_input_with_missing_parameter_loadtime_when_extract_game_config_then_return_error: "loadtime",
		given_input_with_missing_parameter_turntime_when_extract_game_config_then_return_error: "turntime",
		given_input_with_missing_parameter_rows_when_extract_game_config_then_return_error: "rows",
		given_input_with_missing_parameter_cols_when_extract_game_config_then_return_error: "cols",
		given_input_with_missing_parameter_turns_when_extract_game_config_then_return_error: "turns",
		given_input_with_missing_parameter_viewradius2_when_extract_game_config_then_return_error: "viewradius2",
		given_input_with_missing_parameter_spawnradius2_when_extract_game_config_then_return_error: "spawnradius2",
		given_input_with_missing_parameter_player_seed_when_extract_game_config_then_return_error: "player_seed",
	}

	macro_rules! invalid_parameter_value_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
		let setup = Setup::new();
		let mut iter = setup.input.iter().map(|l| {
			let mut type_value = l.split_whitespace();
			let parameter_type = type_value.next().unwrap();
			if parameter_type == $value {
				format!("{} INVALID_VALUE", parameter_type)
			} else {
				l.to_string()
			}
		});

		let result = extract_game_config(&mut iter).unwrap_err();

		assert_eq!(Error::CannotParseGameConfig, result);
        }
    )*
    }
	}

	invalid_parameter_value_tests! {
		given_input_with_invalid_parameter_value_loadtime_when_extract_game_config_then_return_error: "loadtime",
		given_input_with_invalid_parameter_value_turntime_when_extract_game_config_then_return_error: "turntime",
		given_input_with_invalid_parameter_value_rows_when_extract_game_config_then_return_error: "rows",
		given_input_with_invalid_parameter_value_cols_when_extract_game_config_then_return_error: "cols",
		given_input_with_invalid_parameter_value_turns_when_extract_game_config_then_return_error: "turns",
		given_input_with_invalid_parameter_value_viewradius2_when_extract_game_config_then_return_error: "viewradius2",
		given_input_with_invalid_parameter_value_spawnradius2_when_extract_game_config_then_return_error: "spawnradius2",
		given_input_with_invalid_parameter_value_player_seed_when_extract_game_config_then_return_error: "player_seed",
	}
}
