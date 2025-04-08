use super::Agent;
use super::AgentType;
use uuid::Uuid;

#[derive(Clone)]
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
            demand: 10.0,
        }
    }
}

impl Agent for Customer {
    fn agent_type(&self) -> AgentType {
        AgentType::Customer
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
        self.demand += 1.0;

        if self.demand > 15.0 {
            self.demand = 10.0;
        }
    }
}
