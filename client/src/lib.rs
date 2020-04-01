mod parser;
mod unparser;

pub use parser::EndInfo;
pub use parser::Error;
pub use parser::GameConfig;
pub use parser::Player;
pub use parser::TurnInfo;
pub use unparser::Direction;
pub use unparser::Order;

use parser::Parser;
use parser::Turn;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    pub row: i32,
    pub col: i32,
}

pub trait Client {
    fn set_up(&mut self, config: GameConfig);
    fn make_turn(&mut self, turn_info: TurnInfo) -> Vec<Order>;
    fn tear_down(&mut self, end_info: EndInfo);
}

pub fn run<T: AsRef<str>>(
    client: &mut impl Client,
    input: impl Iterator<Item = T>,
    output: impl Fn(&str),
) -> Result<(), Error> {
    let mut parser = Parser::new(input);
    client.set_up(parser.next_start_turn()?);
    output("go");
    while let Some(turn) = parser.next_turn() {
        match turn {
            Turn::Normal(turn) => {
                let orders = client.make_turn(turn?);
                unparser::output_orders(orders, &output);
            }
            Turn::End(turn) => {
                client.tear_down(turn?);
                break;
            }
        }
    }
    Ok(())
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
        SetUp(GameConfig),
        MakeTurn(TurnInfo),
        TearDown(EndInfo),
        Output(String),
    }

    struct TestClient<'a> {
        callbacks: &'a RefCell<Vec<Callback>>,
        orders: Vec<Order>,
    }

    impl<'a> Client for TestClient<'a> {
        fn set_up(&mut self, config: GameConfig) {
            self.callbacks.borrow_mut().push(Callback::SetUp(config));
        }

        fn make_turn(&mut self, turn_info: TurnInfo) -> Vec<Order> {
            self.callbacks
                .borrow_mut()
                .push(Callback::MakeTurn(turn_info));
            self.orders.clone()
        }

        fn tear_down(&mut self, end_info: EndInfo) {
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
        let input = setup
            .config_input
            .iter()
            .chain(iter::once(&"turn 1"))
            .chain(setup.turn_input.iter())
            .chain(iter::once(&"turn 2"))
            .chain(setup.turn_input.iter())
            .chain(setup.end_input.iter());

        run(&mut client, input, save_output).unwrap();

        let calls = callbacks.borrow();
        let mut calls = calls.iter();
        assert_matches!(calls.next(), Some(Callback::SetUp(_)));
        assert_matches!(&calls.next(), Some(Callback::Output(s)) if s == "go");
        assert_matches!(calls.next(), Some(Callback::MakeTurn(_)));
        assert_matches!(&calls.next(), Some(Callback::Output(s)) if s == "o 12 34 N");
        assert_matches!(&calls.next(), Some(Callback::Output(s)) if s == "o 56 78 W");
        assert_matches!(&calls.next(), Some(Callback::Output(s)) if s == "go");
        assert_matches!(calls.next(), Some(Callback::MakeTurn(_)));
        assert_matches!(&calls.next(), Some(Callback::Output(s)) if s == "o 12 34 N");
        assert_matches!(&calls.next(), Some(Callback::Output(s)) if s == "o 56 78 W");
        assert_matches!(&calls.next(), Some(Callback::Output(s)) if s == "go");
        assert_matches!(calls.next(), Some(Callback::TearDown(_)));
    }

    #[test]
    fn given_invalid_set_up_input_when_run_then_return_error() {
        let callbacks = RefCell::new(vec![]);
        let mut client = TestClient {
            callbacks: &callbacks,
            orders: vec![],
        };
        let setup = Setup::new();
        let input = setup.config_input.iter().take(2);

        let result = run(&mut client, input, |_| {});

        assert_matches!(result, Err(_));
    }

    #[test]
    fn given_invalid_turn_input_when_run_then_return_error() {
        let callbacks = RefCell::new(vec![]);
        let mut client = TestClient {
            callbacks: &callbacks,
            orders: vec![],
        };
        let setup = Setup::new();
        let input = setup
            .config_input
            .iter()
            .chain(iter::once(&"turn 1"))
            .chain(setup.turn_input.iter().take(1));

        let result = run(&mut client, input, |_| {});

        assert_matches!(result, Err(_));
    }

    #[test]
    fn given_invalid_end_input_when_run_then_return_error() {
        let callbacks = RefCell::new(vec![]);
        let mut client = TestClient {
            callbacks: &callbacks,
            orders: vec![],
        };
        let setup = Setup::new();
        let input = setup
            .config_input
            .iter()
            .chain(setup.end_input.iter().take(2));

        let result = run(&mut client, input, |_| {});

        assert_matches!(result, Err(_));
    }

    #[test]
    fn given_input_after_end_input_when_run_then_ignore_input_after_end_input() {
        let callbacks = RefCell::new(vec![]);
        let mut client = TestClient {
            callbacks: &callbacks,
            orders: vec![],
        };
        let setup = Setup::new();
        let input = setup
            .config_input
            .iter()
            .chain(setup.end_input.iter())
            .chain(iter::once(&"turn 1"))
            .chain(setup.turn_input.iter());

        run(&mut client, input, |_| {}).unwrap();

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
        let input = iter::once(&"INVALID INPUT")
            .chain(setup.config_input.iter())
            .chain(iter::once(&"INVALID INPUT"))
            .chain(setup.end_input.iter());

        run(&mut client, input, |_| {}).unwrap();

        let calls = callbacks.borrow();
        assert_matches!(calls[0], Callback::SetUp(_));
        assert_matches!(calls[1], Callback::TearDown(_));
    }
}
