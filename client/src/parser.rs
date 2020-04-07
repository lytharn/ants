use super::Position;

#[derive(Debug, PartialEq)]
pub enum Error {
    CannotParseGameConfig,
    CannotParseTurnInfo,
    CannotParseEndInfo,
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Player {
    pub id: i32,
    pub pos: Position,
}

#[derive(Debug)]
pub struct TurnInfo {
    pub water: Vec<Position>, // Sent once
    pub food: Vec<Position>,
    pub ant_hill: Vec<Player>,
    pub ant: Vec<Player>,
    pub dead_ant: Vec<Player>,
}

#[derive(Debug)]
pub struct EndInfo {
    scores: Vec<i32>,
    turn_info: TurnInfo,
}

#[derive(Debug)]
pub enum Turn {
    Normal(Result<TurnInfo, Error>),
    End(Result<EndInfo, Error>),
}

pub struct Parser<T, I>
where
    T: AsRef<str>,
    I: Iterator<Item = T>,
{
    input: I,
}

impl<T, I> Parser<T, I>
where
    T: AsRef<str>,
    I: Iterator<Item = T>,
{
    pub fn new(input: impl IntoIterator<Item = T, IntoIter = I>) -> Self {
        Self { input: input.into_iter() }
    }

    pub fn next_start_turn(&mut self) -> Result<GameConfig, Error> {
        self.input
            .by_ref()
            .skip_while(|l| l.as_ref() != "turn 0")
            .next();
        self.extract_game_config()
    }

    pub fn next_turn(&mut self) -> Option<Turn> {
        while let Some(line) = self.input.next() {
            if line.as_ref().starts_with("turn ") {
                return Some(Turn::Normal(self.extract_turn_info()));
            } else if line.as_ref() == "end" {
                return Some(Turn::End(self.extract_end_info()));
            }
        }
        None
    }

    fn extract_game_config(&mut self) -> Result<GameConfig, Error> {
        let mut load_time: Option<i32> = None;
        let mut turn_time: Option<i32> = None;
        let mut rows: Option<i32> = None;
        let mut cols: Option<i32> = None;
        let mut turns: Option<i32> = None;
        let mut view_radius2: Option<i32> = None;
        let mut attack_radius2: Option<i32> = None;
        let mut food_gathering_radius2: Option<i32> = None;
        let mut player_seed: Option<i64> = None;

        for line in self.input.by_ref() {
            let mut type_value = line.as_ref().split_whitespace();
            let parameter_type = type_value.next();
            let parameter_value = type_value.next();
            match (parameter_type, parameter_value) {
                (Some("loadtime"), Some(v)) => load_time = v.parse().ok(),
                (Some("turntime"), Some(v)) => turn_time = v.parse().ok(),
                (Some("rows"), Some(v)) => rows = v.parse().ok(),
                (Some("cols"), Some(v)) => cols = v.parse().ok(),
                (Some("turns"), Some(v)) => turns = v.parse().ok(),
                (Some("viewradius2"), Some(v)) => view_radius2 = v.parse().ok(),
                (Some("attackradius2"), Some(v)) => attack_radius2 = v.parse().ok(),
                (Some("spawnradius2"), Some(v)) => food_gathering_radius2 = v.parse().ok(),
                (Some("player_seed"), Some(v)) => player_seed = v.parse().ok(),
                (Some("ready"), _) => break,
                _ => (),
            }
        }

        Ok(GameConfig {
            load_time: load_time.ok_or(Error::CannotParseGameConfig)?,
            turn_time: turn_time.ok_or(Error::CannotParseGameConfig)?,
            rows: rows.ok_or(Error::CannotParseGameConfig)?,
            cols: cols.ok_or(Error::CannotParseGameConfig)?,
            turns: turns.ok_or(Error::CannotParseGameConfig)?,
            view_radius2: view_radius2.ok_or(Error::CannotParseGameConfig)?,
            attack_radius2: attack_radius2.ok_or(Error::CannotParseGameConfig)?,
            food_gathering_radius2: food_gathering_radius2.ok_or(Error::CannotParseGameConfig)?,
            player_seed: player_seed.ok_or(Error::CannotParseGameConfig)?,
        })
    }

    fn extract_turn_info(&mut self) -> Result<TurnInfo, Error> {
        let mut water: Vec<Position> = vec![];
        let mut food: Vec<Position> = vec![];
        let mut ant: Vec<Player> = vec![];
        let mut ant_hill: Vec<Player> = vec![];
        let mut dead_ant: Vec<Player> = vec![];

        for line in self.input.by_ref() {
            let mut l = line.as_ref().split_whitespace();
            let parameter = (l.next(), l.next(), l.next(), l.next());
            match parameter {
                (Some("w"), Some(row), Some(col), _) => parse_position(row, col)
                    .into_iter()
                    .for_each(|pos| water.push(pos)),
                (Some("f"), Some(row), Some(col), _) => parse_position(row, col)
                    .into_iter()
                    .for_each(|pos| food.push(pos)),
                (Some("a"), Some(row), Some(col), Some(id)) => parse_player(row, col, id)
                    .into_iter()
                    .for_each(|player| ant.push(player)),
                (Some("h"), Some(row), Some(col), Some(id)) => parse_player(row, col, id)
                    .into_iter()
                    .for_each(|player| ant_hill.push(player)),
                (Some("d"), Some(row), Some(col), Some(id)) => parse_player(row, col, id)
                    .into_iter()
                    .for_each(|player| dead_ant.push(player)),
                (Some("go"), _, _, _) => {
                    return Ok(TurnInfo {
                        water,
                        food,
                        ant,
                        ant_hill,
                        dead_ant,
                    })
                }
                _ => (),
            }
        }
        Err(Error::CannotParseTurnInfo)
    }

    fn extract_end_info(&mut self) -> Result<EndInfo, Error> {
        let mut no_of_players: Option<usize> = None;
        let mut scores: Option<Vec<i32>> = None;

        for line in self.input.by_ref() {
            let mut l = line.as_ref().split_whitespace();
            let parameter = (l.next(), l.next());
            match parameter {
                (Some("players"), Some(players)) => no_of_players = players.parse().ok(),
                (Some("score"), first_score) => {
                    scores = Some(
                        first_score
                            .into_iter()
                            .chain(l)
                            .filter_map(|p| p.parse().ok())
                            .collect(),
                    );
                    break;
                }
                _ => (),
            }
        }

        let turn_info = self
            .extract_turn_info()
            .or(Err(Error::CannotParseEndInfo))?;
        match (no_of_players, scores) {
            (Some(no_of_players), Some(scores)) if no_of_players == scores.len() => {
                Ok(EndInfo { scores, turn_info })
            }
            _ => Err(Error::CannotParseEndInfo),
        }
    }
}

fn parse_player(row: &str, col: &str, id: &str) -> Option<Player> {
    let id_pos = (id.parse(), parse_position(row, col));
    match id_pos {
        (Ok(id), Some(pos)) => Some(Player { id, pos }),
        _ => None,
    }
}

fn parse_position(row: &str, col: &str) -> Option<Position> {
    let row_col = (row.parse().ok(), col.parse().ok());
    match row_col {
        (Some(row), Some(col)) => Some(Position { row, col }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter;

    struct Setup {
        game_config_input: Vec<&'static str>,
        turn_info_input: Vec<&'static str>,
    }

    impl Setup {
        fn new() -> Self {
            Self {
                game_config_input: vec![
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
                turn_info_input: vec![
                    "f 7 4",
                    "f 8 5",
                    "w 7 6",
                    "w 9 7",
                    "a 10 9 0",
                    "a 11 10 1",
                    "h 7 12 1",
                    "h 5 4 0",
                    "d 14 13 0",
                    "d 15 12 1",
                    "go",
                ],
            }
        }
    }

    #[test]
    fn given_correct_input_when_next_start_turn_then_return_game_config() {
        let setup = Setup::new();
        let mut parser = Parser::new(setup.game_config_input);

        let config = parser.next_start_turn().unwrap();

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
    fn given_correct_input_and_invalid_when_next_start_turn_then_return_game_config() {
        let setup = Setup::new();
        let input = setup
            .game_config_input
            .iter()
            .flat_map(|&l| iter::once("INVALID_INPUT").chain(iter::once(l)));

        let mut parser = Parser::new(input);

        let result = parser.next_start_turn();

        assert_matches!(result, Ok(_));
    }

    #[test]
    fn given_input_with_no_turn_0_when_next_start_turn_then_error() {
        let setup = Setup::new();
        let input = setup
            .game_config_input
            .iter()
            .filter(|l| !l.contains("turn 0"));
        let mut parser = Parser::new(input);

        let result = parser.next_start_turn().unwrap_err();

        assert_eq!(result, Error::CannotParseGameConfig);
    }

    macro_rules! missing_parameter_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
		let setup = Setup::new();
		let input = setup.game_config_input.iter().filter(|l| !l.contains($value));
		let mut parser = Parser::new(input);

		let result = parser.next_start_turn().unwrap_err();

		assert_eq!(Error::CannotParseGameConfig, result);
        }
    )*
    }
	}

    missing_parameter_tests! {
        given_input_with_missing_parameter_loadtime_when_next_start_turn_then_return_error: "loadtime",
        given_input_with_missing_parameter_turntime_when_next_start_turn_then_return_error: "turntime",
        given_input_with_missing_parameter_rows_when_next_start_turn_then_return_error: "rows",
        given_input_with_missing_parameter_cols_when_next_start_turn_then_return_error: "cols",
        given_input_with_missing_parameter_turns_when_next_start_turn_then_return_error: "turns",
        given_input_with_missing_parameter_viewradius2_when_next_start_turn_then_return_error: "viewradius2",
        given_input_with_missing_parameter_spawnradius2_when_next_start_turn_then_return_error: "spawnradius2",
        given_input_with_missing_parameter_player_seed_when_next_start_turn_then_return_error: "player_seed",
    }

    macro_rules! invalid_parameter_value_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
		let setup = Setup::new();
		let input = setup.game_config_input.iter().map(|l| {
			let mut type_value = l.split_whitespace();
			let parameter_type = type_value.next().unwrap();
			if parameter_type == $value {
				format!("{} INVALID_VALUE", parameter_type)
			} else {
				l.to_string()
			}
		});
		let mut parser = Parser::new(input);

		let result = parser.next_start_turn().unwrap_err();

		assert_eq!(Error::CannotParseGameConfig, result);
        }
    )*
    }
	}

    invalid_parameter_value_tests! {
        given_input_with_invalid_parameter_value_loadtime_when_next_start_turn_then_return_error: "loadtime",
        given_input_with_invalid_parameter_value_turntime_when_next_start_turn_then_return_error: "turntime",
        given_input_with_invalid_parameter_value_rows_when_next_start_turn_then_return_error: "rows",
        given_input_with_invalid_parameter_value_cols_when_next_start_turn_then_return_error: "cols",
        given_input_with_invalid_parameter_value_turns_when_next_start_turn_then_return_error: "turns",
        given_input_with_invalid_parameter_value_viewradius2_when_next_start_turn_then_return_error: "viewradius2",
        given_input_with_invalid_parameter_value_spawnradius2_when_next_start_turn_then_return_error: "spawnradius2",
        given_input_with_invalid_parameter_value_player_seed_when_next_start_turn_then_return_error: "player_seed",
    }

    fn create_player(id: i32, row: i32, col: i32) -> Player {
        Player {
            id,
            pos: Position { row, col },
        }
    }

    #[test]
    fn given_correct_input_when_extract_turn_info_then_return_turn_info() {
        let setup = Setup::new();
        let input = iter::once("turn 1").chain(setup.turn_info_input);
        let mut parser = Parser::new(input);

        let result = parser.next_turn();

        if let Some(Turn::Normal(Ok(turn_info))) = result {
            assert_eq!(turn_info.food[0], Position { row: 7, col: 4 });
            assert_eq!(turn_info.food[1], Position { row: 8, col: 5 });
            assert_eq!(turn_info.water[0], Position { row: 7, col: 6 });
            assert_eq!(turn_info.water[1], Position { row: 9, col: 7 });
            assert_eq!(turn_info.ant[0], create_player(0, 10, 9));
            assert_eq!(turn_info.ant[1], create_player(1, 11, 10));
            assert_eq!(turn_info.ant_hill[0], create_player(1, 7, 12));
            assert_eq!(turn_info.ant_hill[1], create_player(0, 5, 4));
            assert_eq!(turn_info.dead_ant[0], create_player(0, 14, 13));
            assert_eq!(turn_info.dead_ant[1], create_player(1, 15, 12));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn given_correct_input_and_invalid_when_next_turn_then_return_turn_info() {
        let setup = Setup::new();
        let input = iter::once("turn 1")
            .chain(setup.turn_info_input)
            .flat_map(|l| iter::once("INVALID_INPUT").chain(iter::once(l)));

        let mut parser = Parser::new(input);

        let result = parser.next_turn();

        assert_matches!(result, Some(Turn::Normal(_)));
    }

    #[test]
    fn given_input_with_missing_go_when_next_turn_then_return_error() {
        let input = iter::once("turn 1");
        let mut parser = Parser::new(input);

        let result = parser.next_turn();

        assert_matches!(result, Some(Turn::Normal(Err(Error::CannotParseTurnInfo))));
    }

    #[test]
    fn given_correct_input_when_extract_end_info_then_return_end_info() {
        let setup = Setup::new();
        let input = vec!["end", "players 4", "score 3 5 7 0"]
            .into_iter()
            .chain(setup.turn_info_input);
        let mut parser = Parser::new(input);

        let result = parser.next_turn();

        if let Some(Turn::End(Ok(end_info))) = result {
            assert_eq!(end_info.scores.len(), 4);
            assert_eq!(end_info.scores[0], 3);
            assert_eq!(end_info.scores[1], 5);
            assert_eq!(end_info.scores[2], 7);
            assert_eq!(end_info.scores[3], 0);

            let turn_info = end_info.turn_info;
            assert_eq!(turn_info.food[0], Position { row: 7, col: 4 });
            assert_eq!(turn_info.food[1], Position { row: 8, col: 5 });
            assert_eq!(turn_info.water[0], Position { row: 7, col: 6 });
            assert_eq!(turn_info.water[1], Position { row: 9, col: 7 });
            assert_eq!(turn_info.ant[0], create_player(0, 10, 9));
            assert_eq!(turn_info.ant[1], create_player(1, 11, 10));
            assert_eq!(turn_info.ant_hill[0], create_player(1, 7, 12));
            assert_eq!(turn_info.ant_hill[1], create_player(0, 5, 4));
            assert_eq!(turn_info.dead_ant[0], create_player(0, 14, 13));
            assert_eq!(turn_info.dead_ant[1], create_player(1, 15, 12));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn given_input_with_missing_player_parameter_when_extract_end_info_then_return_error() {
        let setup = Setup::new();
        let input = vec!["end", "score 3 5 7 0"]
            .into_iter()
            .chain(setup.turn_info_input);
        let mut parser = Parser::new(input);

        let result = parser.next_turn();

        assert_matches!(result, Some(Turn::End(Err(Error::CannotParseEndInfo))));
    }

    #[test]
    fn given_input_with_missing_score_parameter_when_extract_end_info_then_return_error() {
        let setup = Setup::new();
        let input = vec!["end", "players 4"]
            .into_iter()
            .chain(setup.turn_info_input);
        let mut parser = Parser::new(input);

        let result = parser.next_turn();

        assert_matches!(result, Some(Turn::End(Err(Error::CannotParseEndInfo))));
    }

    #[test]
    fn given_input_with_invalid_parameter_value_player_when_extract_end_info_then_return_error() {
        let setup = Setup::new();
        let input = vec!["end", "players INVALID_VALUE", "score 3 5 7 0"]
            .into_iter()
            .chain(setup.turn_info_input);
        let mut parser = Parser::new(input);

        let result = parser.next_turn();

        assert_matches!(result, Some(Turn::End(Err(Error::CannotParseEndInfo))));
    }

    #[test]
    fn given_input_with_invalid_parameter_value_score_when_extract_end_info_then_return_error() {
        let setup = Setup::new();
        let input = vec!["end", "players 4", "score 3 5 INVALID_VALUE 0"]
            .into_iter()
            .chain(setup.turn_info_input);
        let mut parser = Parser::new(input);

        let result = parser.next_turn();

        assert_matches!(result, Some(Turn::End(Err(Error::CannotParseEndInfo))));
    }

    #[test]
    fn given_no_input_score_when_extract_end_info_then_return_error() {
        let input = iter::empty::<&str>();
        let mut parser = Parser::new(input);

        let result = parser.next_turn();

        assert_matches!(result, None);
    }
}
