use super::Agent;
use super::AgentType;
use uuid::Uuid;

#[derive(Clone)]
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

impl Agent for BatteryStorage {
    fn agent_type(&self) -> AgentType {
        AgentType::BatteryStorage
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

    fn step(&mut self) {
        // TODO: Implement step
    }
}
