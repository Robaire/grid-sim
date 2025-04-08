use super::AgentData;
use crate::grid::Grid;
use crate::grid::TransmissionLine;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub fn get_agents(grid: State<Mutex<Grid>>) -> Vec<AgentData> {
    let grid = grid.lock().unwrap();
    grid.get_agents()
        .iter()
        .map(|agent| AgentData::from_agent(*agent))
        .collect()
}

#[tauri::command]
pub fn add_connection(grid: State<Mutex<Grid>>, from: String, to: String) -> bool {
    let mut grid = grid.lock().unwrap();

    // Try to convert the string IDs to u128, return false if conversion fails
    let from_id = match u128::from_str_radix(&from, 16) {
        Ok(id) => id,
        Err(_) => return false,
    };

    let to_id = match u128::from_str_radix(&to, 16) {
        Ok(id) => id,
        Err(_) => return false,
    };

    grid.add_connection(from_id, to_id, TransmissionLine::new())
}

#[tauri::command]
pub fn get_connections(grid: State<Mutex<Grid>>, id: String) -> Vec<String> {
    let grid = grid.lock().unwrap();

    let id = match u128::from_str_radix(&id, 16) {
        Ok(id) => id,
        Err(_) => return vec![],
    };

    grid.get_connections(id)
        .iter()
        .map(|id| format!("{:02X}", id))
        .collect()
}
