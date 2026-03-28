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
        let from = read!();
        let to = read!();
        let speed = read!();
        let eta = read!();
        let wagons = read!();

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

pub fn premade_train() -> HashMap<usize, Train> {
    let mut train_map: HashMap<usize, Train> = HashMap::new();
    
    train_map.insert(0, Train {
        direction: "l".to_string(),
        from: 0,
        to: 4,
        speed: 1,
        eta: 20,
        wagons: 6,
        position: 0,
        currLane: 0,
        cooldown: 10,
        timer: 0,
    });

    train_map.insert(1, Train {
        direction: "l".to_string(),
        from: 6,
        to: 1,
        speed: 1,
        eta: 20,
        wagons: 6,
        position: 0,
        currLane: 6,
        cooldown: 10,
        timer: 0,
    });

    train_map.insert(2, Train {
        direction: "l".to_string(),
        from: 1,
        to: 5,
        speed: 1,
        eta: 20,
        wagons: 6,
        position: 0,
        currLane: 1,
        cooldown: 10,
        timer: 0,
    });
    return train_map;
}
