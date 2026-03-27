use crate::display::draw;
use crate::train::Train;
use std::{
    collections::HashMap,
    thread,
    time::{Duration, Instant},
};

pub fn switch(train: &mut Train) {
        if train.from > train.to {
            train.currLane -= 1;
        } else {
            train.currLane += 1;
        }
}

pub fn run(mut trains_map: HashMap<usize, Train>, mut track: Vec<Vec<String>>) {
    let target_fps = 60;
    let frame_duration = Duration::from_millis(1000 / target_fps);

    loop {
        let start_time = Instant::now();

        // --- 1. UPDATE ---
        for (_, train) in &mut trains_map {
            train.timer += 1 * train.speed;

            if train.timer >= train.cooldown {
                if train.position as i32 - train.wagons as i32 >= 0 {
                    track[train.currLane][train.position - train.wagons] = "=".to_string();
                    if track[train.from][train.position - train.wagons] == "#"{
                        track[train.from][train.position - train.wagons] = "=".to_string();
                    }
                }
                if track[train.currLane][train.position] == "x" && train.currLane != train.to{
                    switch(train);
                }
                track[train.currLane][train.position] = "#".to_string();
                train.train_update();
                train.timer = 0;
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
