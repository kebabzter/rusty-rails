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

    const STARTING_COL:usize = (ROW/2) - 3;

    let mut track: Vec<Vec<String>> = vec![vec!["=".to_string(); ROW]; COL];
    for lane in 0..COL {
        for pos in STARTING_COL..(STARTING_COL + 6) {
            track[lane][pos] = "x".to_string();
        }
    }

    println!("Please enter how many trains will run in this simulation:");

    let train_map: HashMap<usize, Train> = create_trains();
    run(train_map, track);
}
