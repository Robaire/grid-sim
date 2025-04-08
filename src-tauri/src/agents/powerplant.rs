use super::Agent;
use super::AgentType;
use uuid::Uuid;

#[derive(Clone)]
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

impl Agent for PowerPlant {
    fn agent_type(&self) -> AgentType {
        AgentType::PowerPlant
    }

    fn id(&self) -> Uuid {
        self.id
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn step(&mut self) {}
}
