use std::collections::HashMap;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct SoundData {
    pub play: bool,
    pub path: String
}

pub type SoundMap = HashMap<String, SoundData>;

#[tauri::command]
fn get_sounds() -> SoundMap {
    let mut map = SoundMap::new();
    map.insert(
        "rain".to_string(),
        SoundData {
            play: false,
            path: "assets/sounds/rain.mp3".to_string(),
        },
    );
    map
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_sounds,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}