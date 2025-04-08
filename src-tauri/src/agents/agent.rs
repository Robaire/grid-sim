use std::fmt;
use uuid::Uuid;

pub enum AgentType {
    Customer,
    PowerPlant,
    BatteryStorage,
}

impl fmt::Display for AgentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgentType::Customer => write!(f, "Customer"),
            AgentType::PowerPlant => write!(f, "Power Plant"),
            AgentType::BatteryStorage => write!(f, "Battery Storage"),
        }
    }
}

pub trait Agent {
    fn agent_type(&self) -> AgentType;
    fn id(&self) -> Uuid;
    fn name(&self) -> String;
    fn set_name(&mut self, name: String);
    fn step(&mut self);
}

impl Agent for Box<dyn Agent + Send + Sync> {
    fn agent_type(&self) -> AgentType {
        self.as_ref().agent_type()
    }

    fn id(&self) -> Uuid {
        self.as_ref().id()
    }

    fn name(&self) -> String {
        self.as_ref().name()
    }

    fn set_name(&mut self, name: String) {
        self.as_mut().set_name(name);
    }

    fn step(&mut self) {
        self.as_mut().step();
    }
}

/*
#[derive(Clone)]
pub enum Agent {
    Customer(Customer),
    PowerPlant(PowerPlant),
    BatteryStorage(BatteryStorage),
}

impl Agent {
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

    pub fn step(
        &mut self,
        prev_in: Vec<EdgeReference<TransmissionLine>>,
        new_in: Vec<EdgeReference<TransmissionLine>>,
        prev_out: Vec<EdgeReference<TransmissionLine>>,
        new_out: Vec<EdgeReference<TransmissionLine>>,
    ) {
        match self {
            Agent::Customer(agent) => agent.step(),
            Agent::PowerPlant(agent) => agent.step(),
            Agent::BatteryStorage(agent) => agent.step(),
        }
    }
}
*/
