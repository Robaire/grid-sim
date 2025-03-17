use std::sync::Mutex;
use tauri::State;

mod agents;
use agents::Agent;
use agents::AgentData;
use agents::BatteryStorage;
use agents::Customer;
use agents::PowerPlant;

mod grid;
use grid::Grid;

#[tauri::command]
fn add_agent(grid: State<Mutex<Grid>>, agent_type: String) {
    let mut grid = grid.lock().unwrap();

    match agent_type.as_str() {
        "Customer" => {
            let agent = Customer::new();
            grid.add_agent(Agent::Customer(Customer::new()));
        }
        "Power Plant" => {
            grid.add_agent(Agent::PowerPlant(PowerPlant::new()));
        }
        "Battery Storage" => {
            grid.add_agent(Agent::BatteryStorage(BatteryStorage::new()));
        }
        _ => {
            println!("Invalid agent type");
        }
    }
}

#[tauri::command]
fn delete_agent(grid: State<Mutex<Grid>>, id: String) {
    let mut grid = grid.lock().unwrap();
    let id = u128::from_str_radix(&id, 16).unwrap();
    grid.delete_agent(id);
}

#[tauri::command]
fn set_agent(grid: State<Mutex<Grid>>, data: AgentData) {
    let mut grid = grid.lock().unwrap();
    grid.set_agent(u128::from_str_radix(&data.id, 16).unwrap(), data.name);
}

#[tauri::command]
fn get_agents(grid: State<Mutex<Grid>>) -> Vec<AgentData> {
    let grid = grid.lock().unwrap();
    grid.get_agents()
        .iter()
        .map(|agent| agent.as_data())
        .collect()
}

#[tauri::command]
fn save_model(grid: State<Mutex<Grid>>, path: String) {
    // TODO: Implement
}

#[tauri::command]
fn load_model(grid: State<Mutex<Grid>>, path: String) {
    // TODO: Implement
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut grid = Grid::new();

    let mut agent = Agent::Customer(Customer::new());
    agent.set_name("Bob".to_string());
    grid.add_agent(agent);

    agent = Agent::Customer(Customer::new());
    agent.set_name("Alice".to_string());
    grid.add_agent(agent);

    agent = Agent::Customer(Customer::new());
    agent.set_name("Charlie".to_string());
    grid.add_agent(agent);

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_agents,
            add_agent,
            delete_agent,
            set_agent,
            save_model,
            load_model
        ])
        .manage(Mutex::new(grid))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
