use crate::display::draw;
use crate::train::Train;
use std::{
    collections::HashMap,
    thread,
    time::{Duration, Instant},
};

// pub fn switch(mut train:Train, mut track: Vec<Vec<String>>){
//     if(track[train.position + 1][train.currLane] == "="){
//         train.currLane = train.to;
//     }
// }

pub fn run(mut trains_map: HashMap<usize, Train>, mut track: Vec<Vec<String>>) {
    let target_fps = 60;
    let frame_duration = Duration::from_millis(1000 / target_fps);

    loop {
        let start_time = Instant::now();

        // --- 1. UPDATE ---
        for (_, train) in &mut trains_map {
            train.timer += 1 * train.speed;

            if train.timer >= train.cooldown {
                if train.position - train.wagons >= 0 {
                    track[train.currLane as usize][(train.position - train.wagons) as usize] = "=".to_string();
                }
                track[train.currLane as usize][train.position as usize] = "#".to_string();
                train.train_update();
                train.timer = 0
            }
        }

        // --- 2. DRAW ---
        draw(&track);

        // --- 3. WAIT ---
        let elapsed = start_time.elapsed();
        if elapsed < frame_duration {
            thread::sleep(frame_duration - elapsed);
        }
    }
}
