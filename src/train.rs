#[derive(Debug)]
pub struct Train{
    pub direction: String,
    pub from: usize,
    pub to: usize,
    pub speed: usize,
    pub eta: usize,
    pub wagons : usize,

    pub currLane: usize,
    pub position: usize,

    pub cooldown: usize,
    pub timer: usize,
}

impl Train {
    pub fn train_update(&mut self){
        self.position += 1;
    }
}
