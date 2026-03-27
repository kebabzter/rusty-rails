use crate::train::Train;
use std::collections::HashMap;
use text_io::read;

pub fn create_trains() -> HashMap<usize, Train> {
    let mut train_map: HashMap<usize, Train> = HashMap::new();
    let num_trains: usize = read!();

    let mut i: usize = 0;

    println!("Simulation with {} trains", num_trains);

    clearscreen::clear().expect("failed to clear screen");

    println!("Enter in the following format");
    println!(
        "[direction] (l/r) [from](lane number) [to](lane number) [speed](1-3) [eta](1-60) [wagons](1-5)"
    );

    while i < num_trains {
        let dir: String = read!();
        let from: i32 = read!();
        let to: i32 = read!();
        let speed: i32 = read!();
        let eta: i32 = read!();
        let wagons: i32 = read!();

        let temp_train: Train = Train {
            direction: dir,
            from: from,
            to: to,
            speed: speed,
            eta: eta,
            wagons: wagons,
            position: 0,
            currLane: from,
            cooldown: 10,
            timer: 0,
        };

        train_map.insert(i, temp_train);

        i += 1;
    }
    clearscreen::clear().expect("failed to clear screen");
    return train_map;
}