use super::Producer;
use uuid::Uuid;

pub struct PowerPlant {
    id: Uuid,
    name: String,
    supply: f32,
    rate: f32,
}

impl PowerPlant {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "Power Plant".to_string(),
            supply: 100.0,
            rate: 1.0,
        }
    }
}

impl PowerPlant {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn step(&self) {}
}

impl Producer for PowerPlant {
    fn supply(&self) -> f32 {
        self.supply
    }

    fn rate(&self) -> f32 {
        self.rate
    }
}
