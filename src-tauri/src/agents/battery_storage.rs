use super::{Consumer, Producer};
use uuid::Uuid;

pub struct BatteryStorage {
    id: Uuid,
    name: String,
    capacity: f32,
    charge: f32,
}

impl BatteryStorage {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "Battery Storage".to_string(),
            capacity: 100.0,
            charge: 50.0,
        }
    }
}

impl BatteryStorage {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn step(&self) {
        // TODO: Implement step
    }
}

impl Consumer for BatteryStorage {
    fn draw(&self) -> f32 {
        0.0
    }
}

impl Producer for BatteryStorage {
    fn supply(&self) -> f32 {
        0.0
    }

    fn rate(&self) -> f32 {
        0.0
    }
}
