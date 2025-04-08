use std::sync::Mutex;

mod agents;
use agents::Agent;
use agents::Customer;
use agents::PowerPlant;

mod controller;

mod grid;
use grid::Grid;
use grid::TransmissionLine;

fn example_grid() -> Grid {
    let mut grid = Grid::new();

    let mut producer = PowerPlant::new();
    producer.set_name("National Grid".to_string());
    let producer_id = producer.id().as_u128();
    grid.add_agent(Box::new(producer));

    let mut agent = Customer::new();
    agent.set_name("Alice".to_string());
    let agent_id = agent.id().as_u128();
    grid.add_agent(Box::new(agent));
    grid.add_connection(producer_id, agent_id, TransmissionLine::new());

    let mut agent = Customer::new();
    agent.set_name("Bob".to_string());
    let agent_id = agent.id().as_u128();
    grid.add_agent(Box::new(agent));
    grid.add_connection(producer_id, agent_id, TransmissionLine::new());

    let mut agent = Customer::new();
    agent.set_name("Charlie".to_string());
    let agent_id = agent.id().as_u128();
    grid.add_agent(Box::new(agent));
    grid.add_connection(producer_id, agent_id, TransmissionLine::new());

    return grid;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let grid = example_grid();

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            controller::grid::get_agents,
            controller::agent::add_agent,
            controller::agent::delete_agent,
            controller::agent::set_agent_name,
            controller::grid::add_connection,
            controller::grid::get_connections,
            controller::model::save_model,
            controller::model::load_model,
            controller::model::run_model
        ])
        .manage(Mutex::new(grid))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
