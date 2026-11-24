// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri_specta::collect_commands;

use crate::commands::{greet, load_project};
use crate::models::{Employee, Organization, Position, Project, Roster, RosterTemplate, Shift};
mod commands;
mod models;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri_specta::Builder::new()
        .typ::<Project>()
        .typ::<Organization>()
        .typ::<Position>()
        .typ::<Employee>()
        .typ::<Shift>()
        .typ::<RosterTemplate>()
        .typ::<Roster>()
        .commands(collect_commands![greet, load_project]);

    #[cfg(debug_assertions)]
    builder
        .export(
            specta_typescript::Typescript::default(),
            "../src/lib/bindings.ts",
        )
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
