use crate::agents::{Agent, BatteryStorage, Customer, PowerPlant};
use crate::grid::Grid;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

#[derive(Serialize, Deserialize)]
pub struct AgentData {
    pub id: String,
    pub name: String,
    pub agent_type: String,
}

impl AgentData {
    pub fn from_agent(agent: &impl Agent) -> Self {
        AgentData {
            id: agent
                .id()
                .as_bytes()
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<String>(),
            name: agent.name(),
            agent_type: agent.agent_type().to_string(),
        }
    }
}

#[tauri::command]
pub fn add_agent(grid: State<Mutex<Grid>>, agent_type: String) -> bool {
    let mut grid = grid.lock().unwrap();

    match agent_type.as_str() {
        "Customer" => {
            grid.add_agent(Box::new(Customer::new()));
        }
        "Power Plant" => {
            grid.add_agent(Box::new(PowerPlant::new()));
        }
        "Battery Storage" => {
            grid.add_agent(Box::new(BatteryStorage::new()));
        }
        _ => {
            println!("Invalid agent type");
            return false;
        }
    }

    return true;
}

#[tauri::command]
pub fn delete_agent(grid: State<Mutex<Grid>>, id: String) -> bool {
    let mut grid = grid.lock().unwrap();

    let id = match u128::from_str_radix(&id, 16) {
        Ok(id) => id,
        Err(_) => return false,
    };

    grid.delete_agent(id);

    return true;
}

#[tauri::command]
pub fn set_agent_name(grid: State<Mutex<Grid>>, data: AgentData) {
    let mut grid = grid.lock().unwrap();

    let id = match u128::from_str_radix(&data.id, 16) {
        Ok(id) => id,
        Err(_) => return,
    };

    grid.set_agent_name(id, data.name);
}
