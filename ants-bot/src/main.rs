use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let stdin_iter = stdin.lock().lines().map(|l| l.unwrap());
    let mut client = client::Client::new(stdin_iter, |o| println!("{}", o));

    let _config = client.set_up();
    let mut agent = ai::Agent {};
    client.run(&mut agent).unwrap();
}
