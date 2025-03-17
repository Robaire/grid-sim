use super::Consumer;
use uuid::Uuid;

pub struct Customer {
    id: Uuid,
    name: String,
    demand: f32,
}

impl Customer {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "Customer".to_string(),
            demand: 100.0,
        }
    }
}

impl Customer {
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

impl Consumer for Customer {
    fn draw(&self) -> f32 {
        self.demand
    }
}
