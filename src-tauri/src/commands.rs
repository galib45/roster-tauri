use uuid::Uuid;

use crate::models::{Organization, Project};

#[tauri::command]
#[specta::specta]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
#[specta::specta]
pub async fn load_project() -> Project {
    Project {
        organizations: vec![Organization {
            id: Uuid::now_v7(),
            name: "UHC, Sreepur, Magura".into(),
            positions: Vec::new(),
            employees: Vec::new(),
            shifts: Vec::new(),
            roster_templates: Vec::new(),
            rosters: Vec::new(),
        }],
    }
}
