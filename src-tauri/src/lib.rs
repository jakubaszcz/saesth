use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct SoundData {
    pub play: bool,
    pub path: String
}

pub type SoundMap = HashMap<String, SoundData>;

static SOUND_MAP: OnceLock<Mutex<SoundMap>> = OnceLock::new();

fn init_sounds() {
    let mut map = SoundMap::new();
    map.insert("rain".to_string(), SoundData {
        play: false,
        path: "assets/sounds/rain.mp3".to_string(),
    });

    SOUND_MAP.get_or_init(|| Mutex::new(map));
}

#[tauri::command]
fn get_sounds() -> SoundMap {
    SOUND_MAP.get().unwrap().lock().unwrap().clone()
}
#[tauri::command]
fn toggle_play(id: String) {
    let mut map = SOUND_MAP.get().unwrap().lock().unwrap();
    if let Some(sound) = map.get_mut(&id) {
        sound.play = !sound.play;
        println!("{:?}", sound.play);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    init_sounds();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_sounds,
            toggle_play,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}