use std::fs::File;
use tauri::{AppHandle, Manager, path::BaseDirectory};
use rodio::{Decoder, DeviceSinkBuilder, Player};

#[tauri::command]
async fn play_sound(app_handle: AppHandle) -> Result<(), String> {
    let resource_path = app_handle
        .path()
        .resolve("assets/universfield-new-notification-060-494264.mp3", BaseDirectory::Resource)
        .map_err(|e| e.to_string())?;

    let system_sink = DeviceSinkBuilder::open_default_sink()
        .map_err(|e| format!("No Audio device found!: {}", e))?;

    let file = File::open(resource_path).map_err(|e| e.to_string())?;
    let source = Decoder::try_from(file).map_err(|e| e.to_string())?;

    let player = Player::connect_new(system_sink.mixer());
    player.append(source);
    
    player.sleep_until_end();

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![play_sound])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
