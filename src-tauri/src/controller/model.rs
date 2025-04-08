use crate::grid::Grid;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub fn save_model(grid: State<Mutex<Grid>>, path: String) {
    // TODO: Implement
}

#[tauri::command]
pub fn load_model(grid: State<Mutex<Grid>>, path: String) {
    // TODO: Implement
}

#[tauri::command]
pub fn run_model(grid: State<Mutex<Grid>>, steps: u32) {
    let mut grid = grid.lock().unwrap();

    for _ in 0..steps {
        grid.step();
    }
}
