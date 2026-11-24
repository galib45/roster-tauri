use crate::models::Project;

#[tauri::command]
#[specta::specta]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
#[specta::specta]
pub async fn load_project() -> Project {
    Project {
        organizations: Vec::new(),
    }
}
