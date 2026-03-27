use std::collections::HashMap;

mod train;
mod display;
mod input;

use train::Train;
use input::create_trains;
use display::{draw, print_start};


fn main() {

    draw();

    // print_start();
    // println!("Please enter how many trains will run in this simulation:");

    // let train_map: HashMap<usize, Train> = create_trains();

    // println!("{:#?}", train_map);
}


