use rand::Rng;

#[derive(Debug)]
pub struct Config {
    pub load_time: i32,
    pub turn_time: i32,
    pub width: i32,
    pub height: i32,
    pub turns: i32,
    pub view_radius2: i32,
    pub attack_radius2: i32,
    pub food_gathering_radius2: i32,
    pub player_seed: i64,
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PlayerEntity {
    pub id: i32,
    pub pos: Position,
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
#[derive(Debug)]
pub struct TurnInfo {
    pub water: Vec<Position>, // Sent once
    pub food: Vec<Position>,
    pub ant_hill: Vec<PlayerEntity>,
    pub ant: Vec<PlayerEntity>,
    pub dead_ant: Vec<PlayerEntity>,
}
#[derive(Debug)]
pub struct EndInfo {
    pub scores: Vec<i32>,
    pub turn_info: TurnInfo,
}
#[derive(Debug, Copy, Clone)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Debug, Copy, Clone)]
pub struct Order {
    pub pos: Position,
    pub direction: Direction,
}

impl Order {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        Self {
            pos: Position { x, y },
            direction,
        }
    }
}

pub trait TurnTaker {
    fn take_turn(&mut self, turn_info: TurnInfo) -> Vec<Order>;
    fn end(&mut self, end_info: EndInfo);
}

pub struct Agent {}

impl TurnTaker for Agent {
    fn take_turn(&mut self, turn_info: TurnInfo) -> Vec<Order> {
        let mut rng = rand::thread_rng();
        turn_info
            .ant
            .iter()
            .filter(|a| a.id == 0)
            .map(|a| {
                let direction = match rng.gen_range(0, 3) {
                    0 => Direction::N,
                    1 => Direction::E,
                    2 => Direction::S,
                    _ => Direction::W,
                };
                Order {
                    pos: a.pos,
                    direction,
                }
            })
            .collect()
    }

    fn end(&mut self, _end_info: EndInfo) {}
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
        let turn_info = TurnInfo {
            water: vec![],
            food: vec![],
            ant_hill: vec![],
            ant: vec![
                PlayerEntity {
                    id: 0,
                    pos: Position { x: 2, y: 1 },
                },
                PlayerEntity {
                    id: 1,
                    pos: Position { x: 5, y: 4 },
                },
                PlayerEntity {
                    id: 0,
                    pos: Position { x: 7, y: 6 },
                },
            ],
            dead_ant: vec![],
        };

        let mut orders = agent.take_turn(turn_info).into_iter();
        assert_matches!(
            orders.next(),
            Some(Order {
                pos: Position { x: 2, y: 1 },
                ..
            })
        );
    }
}
