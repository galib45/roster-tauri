use tauri::Manager;

use crate::models::Project;
use std::{fs, path::PathBuf};

const DATA_FILE: &str = "data.json";
fn ensure_data_file(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Couldn't get app_data_dir {e}"))?;
    fs::create_dir_all(&app_data_dir).map_err(|e| format!("Couldn't create app_data_dir {e}"))?;
    let data_file_path = app_data_dir.join(DATA_FILE);
    if !data_file_path.exists() {
        let project = Project::default();
        let content = serde_json::to_string_pretty(&project)
            .map_err(|e| format!("Couldn't deserialize {e}"))?;
        fs::write(&data_file_path, content)
            .map_err(|e| format!("Couldn't write to data_file_path {e}"))?;
    }
    Ok(data_file_path)
}

#[tauri::command]
#[specta::specta]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
#[specta::specta]
pub async fn load_project(app: tauri::AppHandle) -> Result<Project, String> {
    let data_file_path =
        ensure_data_file(&app).map_err(|e| format!("Couldn't ensure data_file {e}"))?;
    dbg!(&data_file_path);
    let content = fs::read_to_string(data_file_path)
        .map_err(|e| format!("Failed to read project file: {}", e))?;
    let project = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse project file: {}", e))?;
    Ok(project)
}
