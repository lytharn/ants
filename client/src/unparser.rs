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
    pub pos: Position,
    pub direction: Direction,
}

impl Order {
    pub fn new(row: i32, col: i32, direction: Direction) -> Self {
        Self {
            pos: Position { row, col },
            direction,
        }
    }
}

#[derive(Debug)]
pub struct Unparser<F: Fn(&str)> {
    output: F,
}

impl<F: Fn(&str)> Unparser<F> {
    pub fn new(output: F) -> Self {
        Self { output }
    }

    pub fn output_go(&self) {
        (self.output)("go");
    }

    pub fn output_orders(&self, orders: Vec<Order>) {
        orders.iter().for_each(|o| {
            (self.output)(
                format!(
                    "o {} {} {}",
                    o.pos.row.to_string(),
                    o.pos.col.to_string(),
                    unparse_direction(o.direction),
                )
                .as_str(),
            );
        });
        (self.output)("go");
    }
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
    use std::cell::RefCell;

    #[test]
    fn when_output_go_then_output_go() {
        let outputs = RefCell::new(vec![]);
        let save_output = |o: &str| outputs.borrow_mut().push(o.to_string());
        let unparser = Unparser::new(save_output);

        unparser.output_go();

        let outputs = outputs.borrow();
        assert_eq!(outputs[0].as_str(), "go");
    }

    #[test]
    fn given_orders_when_output_orders_then_output_orders_as_a_str() {
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
        let outputs = RefCell::new(vec![]);
        let save_output = |o: &str| outputs.borrow_mut().push(o.to_string());
        let unparser = Unparser::new(save_output);

        unparser.output_orders(orders);

        let outputs = outputs.borrow();
        assert_eq!(outputs[0].as_str(), "o 0 0 N");
        assert_eq!(outputs[1].as_str(), "o 0 1 E");
        assert_eq!(outputs[2].as_str(), "o 1 0 S");
        assert_eq!(outputs[3].as_str(), "o 42 32 W");
        assert_eq!(outputs[4].as_str(), "go");
    }
}
