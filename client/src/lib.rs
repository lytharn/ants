mod parser;
mod unparser;

use ai::Config;
use ai::TurnTaker;
use parser::Parser;
use parser::Turn;
use unparser::Unparser;

pub use parser::Error;

pub struct Client<I, O> {
    parser: Parser<I>,
    unparser: Unparser<O>,
}

impl<T, I, O> Client<I, O>
where
    T: AsRef<str>,
    I: Iterator<Item = T>,
    O: Fn(&str),
{
    pub fn new(input: impl IntoIterator<Item = T, IntoIter = I>, output: O) -> Self {
        Self {
            parser: Parser::new(input),
            unparser: Unparser::new(output),
        }
    }

    pub fn set_up(&mut self) -> Result<Config, Error> {
        self.parser.next_start_turn()
    }

    pub fn run(&mut self, turn_taker: &mut impl TurnTaker) -> Result<(), Error> {
        self.unparser.output_go();
        while let Some(turn) = self.parser.next_turn() {
            match turn {
                Turn::Normal(turn) => {
                    let orders = turn_taker.take_turn(turn?);
                    self.unparser.output_orders(orders);
                }
                Turn::End(turn) => {
                    turn_taker.end(turn?);
                    break;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
#[macro_use]
extern crate assert_matches;

#[cfg(test)]
mod tests {
    use super::*;
    use ai::Direction;
    use ai::EndInfo;
    use ai::Order;
    use ai::TurnInfo;
    use std::cell::RefCell;
    use std::iter;

    fn a_start_turn_input() -> impl Iterator<Item = &'static str> {
        vec![
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
        ]
        .into_iter()
    }

    fn a_normal_turn_input(turn: &str) -> impl Iterator<Item = &str> {
        iter::once(turn).chain(vec!["f 6 5", "w 7 6", "a 10 9 0", "h 7 12 0", "go"].into_iter())
    }

    fn a_end_turn_input() -> impl Iterator<Item = &'static str> {
        vec![
            "end",
            "players 2",
            "score 11 12",
            "f 6 5",
            "d 7 8 1",
            "a 9 9 0",
            "go",
        ]
        .into_iter()
    }

    #[derive(Debug)]
    enum Callback {
        TakeTurn(TurnInfo),
        End(EndInfo),
        Output(String),
    }

    struct TestTurnTaker<'a> {
        callbacks: &'a RefCell<Vec<Callback>>,
        orders: Vec<Order>,
    }

    impl<'a> TurnTaker for TestTurnTaker<'a> {
        fn take_turn(&mut self, turn_info: TurnInfo) -> Vec<Order> {
            self.callbacks
                .borrow_mut()
                .push(Callback::TakeTurn(turn_info));
            self.orders.clone()
        }

        fn end(&mut self, end_info: EndInfo) {
            self.callbacks.borrow_mut().push(Callback::End(end_info));
        }
    }

    #[test]
    fn given_input_when_set_up_then_according_to_input() {
        let input = a_start_turn_input();
        let mut client = Client::new(input, |_| {});

        let config = client.set_up();

        assert_matches!(
            config,
            Ok(Config {
                load_time: 3000,
                turn_time: 1000,
                rows: 20,
                cols: 30,
                turns: 500,
                view_radius2: 55,
                attack_radius2: 5,
                food_gathering_radius2: 1,
                player_seed: 42
            })
        );
    }

    #[test]
    fn given_invalid_set_up_input_when_set_up_then_return_error() {
        let input = a_start_turn_input().take(2);
        let mut client = Client::new(input, |_| {});

        let result = client.set_up();

        assert_matches!(result, Err(_));
    }

    #[test]
    fn given_input_when_run_then_according_to_input() {
        let callbacks = RefCell::new(vec![]);
        let save_output = |o: &str| callbacks.borrow_mut().push(Callback::Output(o.to_string()));

        let mut turn_taker = TestTurnTaker {
            callbacks: &callbacks,
            orders: vec![
                Order::new(12, 34, Direction::N),
                Order::new(56, 78, Direction::W),
            ],
        };
        let input = a_start_turn_input()
            .chain(a_normal_turn_input("turn 1"))
            .chain(a_normal_turn_input("turn 2"))
            .chain(a_end_turn_input());
        let mut client = Client::new(input, save_output);
        let _ = client.set_up();

        client.run(&mut turn_taker).unwrap();

        let calls = callbacks.borrow();
        let mut calls = calls.iter();
        assert_matches!(calls.next(), Some(Callback::Output(s)) if s == "go");
        assert_matches!(calls.next(), Some(Callback::TakeTurn(_)));
        assert_matches!(calls.next(), Some(Callback::Output(s)) if s == "o 12 34 N");
        assert_matches!(calls.next(), Some(Callback::Output(s)) if s == "o 56 78 W");
        assert_matches!(calls.next(), Some(Callback::Output(s)) if s == "go");
        assert_matches!(calls.next(), Some(Callback::TakeTurn(_)));
        assert_matches!(calls.next(), Some(Callback::Output(s)) if s == "o 12 34 N");
        assert_matches!(calls.next(), Some(Callback::Output(s)) if s == "o 56 78 W");
        assert_matches!(calls.next(), Some(Callback::Output(s)) if s == "go");
        assert_matches!(calls.next(), Some(Callback::End(_)));
    }

    #[test]
    fn given_invalid_turn_input_when_run_then_return_error() {
        let callbacks = RefCell::new(vec![]);
        let mut turn_taker = TestTurnTaker {
            callbacks: &callbacks,
            orders: vec![],
        };
        let input = a_start_turn_input().chain(a_normal_turn_input("turn 1").take(2));
        let mut client = Client::new(input, |_| {});
        let _ = client.set_up();

        let result = client.run(&mut turn_taker);

        assert_matches!(result, Err(_));
    }

    #[test]
    fn given_invalid_end_input_when_run_then_return_error() {
        let callbacks = RefCell::new(vec![]);
        let mut turn_taker = TestTurnTaker {
            callbacks: &callbacks,
            orders: vec![],
        };
        let input = a_start_turn_input().chain(a_end_turn_input().take(2));
        let mut client = Client::new(input, |_| {});
        let _ = client.set_up();

        let result = client.run(&mut turn_taker);

        assert_matches!(result, Err(_));
    }

    #[test]
    fn given_input_after_end_input_when_run_then_ignore_input_after_end_input() {
        let callbacks = RefCell::new(vec![]);
        let mut turn_taker = TestTurnTaker {
            callbacks: &callbacks,
            orders: vec![],
        };
        let input = a_start_turn_input()
            .chain(a_end_turn_input())
            .chain(a_normal_turn_input("turn 1"));
        let mut client = Client::new(input, |_| {});
        let _ = client.set_up();

        client.run(&mut turn_taker).unwrap();

        let take_turn_called = callbacks.borrow().iter().any(|c| match c {
            Callback::TakeTurn(_) => true,
            _ => false,
        });
        assert_eq!(take_turn_called, false);
    }

    #[test]
    fn given_invalid_input_when_run_then_ignore_invalid_input() {
        let callbacks = RefCell::new(vec![]);
        let mut turn_taker = TestTurnTaker {
            callbacks: &callbacks,
            orders: vec![],
        };
        let input = iter::once("INVALID INPUT")
            .chain(a_start_turn_input())
            .chain(iter::once("INVALID INPUT"))
            .chain(a_end_turn_input());
        let mut client = Client::new(input, |_| {});
        let _ = client.set_up();

        client.run(&mut turn_taker).unwrap();

        let calls = callbacks.borrow();
        let mut calls = calls.iter();
        assert_matches!(calls.next(), Some(Callback::End(_)));
    }
}
