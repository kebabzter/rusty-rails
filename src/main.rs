use std::collections::HashMap;

mod display;
mod input;
mod train;
mod engine;

use display::{draw, print_start};
use input::create_trains;
use train::Train;

use crate::engine::run;

fn main() {
    print_start();

    const ROW: usize = 60;
    const COL: usize = 7;
    let mut track: Vec<Vec<String>> = vec![vec!["=".to_string(); ROW]; COL];

    println!("Please enter how many trains will run in this simulation:");

    let train_map: HashMap<usize, Train> = create_trains();
    run(train_map, track);
}
