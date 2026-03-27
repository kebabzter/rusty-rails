#[derive(Debug)]
pub struct Train{
    pub direction: String,
    pub from: i32,
    pub to: i32,
    pub speed: i32,
    pub eta: i32,
    pub wagons : i32,

    pub currLane: i32,
    pub position: i32,

    pub cooldown: i32,
    pub timer: i32
}

impl Train {
    pub fn train_update(&mut self){
        self.position += 1;
    }
}
