use super::BatteryStorage;
use super::Customer;
use super::PowerPlant;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub enum Agent {
    Customer(Customer),
    PowerPlant(PowerPlant),
    BatteryStorage(BatteryStorage),
}

#[derive(Serialize, Deserialize)]
pub struct AgentData {
    pub id: String,
    pub name: String,
    pub agent_type: String,
}

impl Agent {
    pub fn as_data(&self) -> AgentData {
        AgentData {
            id: self
                .id()
                .as_bytes()
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<String>(),
            name: self.name(),
            agent_type: self.agent_type(),
        }
    }

    pub fn agent_type(&self) -> String {
        match self {
            Agent::Customer(_) => "Customer".to_string(),
            Agent::PowerPlant(_) => "Power Plant".to_string(),
            Agent::BatteryStorage(_) => "Battery Storage".to_string(),
        }
    }

    pub fn id(&self) -> Uuid {
        match self {
            Agent::Customer(agent) => agent.id(),
            Agent::PowerPlant(agent) => agent.id(),
            Agent::BatteryStorage(agent) => agent.id(),
        }
    }

    pub fn name(&self) -> String {
        match self {
            Agent::Customer(agent) => agent.name(),
            Agent::PowerPlant(agent) => agent.name(),
            Agent::BatteryStorage(agent) => agent.name(),
        }
    }

    pub fn set_name(&mut self, name: String) {
        match self {
            Agent::Customer(agent) => agent.set_name(name),
            Agent::PowerPlant(agent) => agent.set_name(name),
            Agent::BatteryStorage(agent) => agent.set_name(name),
        }
    }

    pub fn step(&self) {
        match self {
            Agent::Customer(agent) => agent.step(),
            Agent::PowerPlant(agent) => agent.step(),
            Agent::BatteryStorage(agent) => agent.step(),
        }
    }
}

pub trait Consumer {
    fn draw(&self) -> f32;
}

pub trait Producer {
    fn supply(&self) -> f32;
    fn rate(&self) -> f32;
}
