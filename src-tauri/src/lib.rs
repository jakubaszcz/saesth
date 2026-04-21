use std::fs::File;
use std::sync::{Mutex, OnceLock};
use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Source};
use tauri::Manager;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct SoundData {
    pub id: String,
    pub play: bool,
    pub path: String,
    pub volume: f32
}

pub struct SoundStream {
    pub handle: Option<MixerDeviceSink>,
    pub data: SoundData
}

pub type SoundList = Vec<SoundStream>;

static SOUND_LIST: OnceLock<Mutex<SoundList>> = OnceLock::new();
fn init_sounds() {
    let mut list = Vec::new();
    list.push(SoundStream {
        handle: None,
        data: SoundData {
            id: "rain".to_string(),
            play: false,
            volume: 0.5,
            path: "sounds/rain.mp3".to_string(),
        }
    });
    list.push(SoundStream {
        handle: None,
        data: SoundData {
            id: "thunder".to_string(),
            play: false,
            volume: 0.5,
            path: "sounds/thunder.mp3".to_string(),
        }
    });

    SOUND_LIST.get_or_init(|| Mutex::new(list));
}

#[tauri::command]
fn get_sounds() -> Vec<SoundData> {
    let list = SOUND_LIST.get().unwrap().lock().unwrap();
    list.iter().map(|v| v.data.clone()).collect()
}

#[tauri::command]
fn change_volume(id: String, volume: f32) -> Vec<SoundData> {
    let mut list = SOUND_LIST.get().unwrap().lock().unwrap();

    if let Some(sound) = list.iter_mut().find(|s| s.data.id == id) {
        sound.data.volume = volume;
    }
    list.iter()
        .map(|v| v.data.clone())
        .collect()
}

#[tauri::command]
fn toggle_play(id: String) -> Vec<SoundData> {
    let mut list = SOUND_LIST.get().unwrap().lock().unwrap();

    if let Some(sound) = list.iter_mut().find(|s| s.data.id == id) {
        if sound.data.play {
            if let Some(handle) = sound.handle.take() {
                drop(handle);
            }
            sound.data.play = false;
        } else {
            let handle = DeviceSinkBuilder::open_default_sink()
                .expect("failed to open default audio device");

            let file = File::open(&sound.data.path)
                .expect("failed to open audio file");

            let source = Decoder::try_from(file)
                .expect("failed to decode audio file")
                .amplify(sound.data.volume)
                .repeat_infinite();

            handle.mixer().add(source);

            sound.handle = Some(handle);
            sound.data.play = true;
        }
    }

    list.iter()
        .map(|v| v.data.clone())
        .collect()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    init_sounds();

    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            // Focus on opening
            window.set_focus().unwrap();

            // Hide the window title bar
            window.set_decorations(false).unwrap();

            // Window position
            {
                let window_size = window.outer_size().unwrap();

                let current_screen = window.current_monitor().unwrap().unwrap();
                let screen_size = current_screen.size();

                let position_x = (screen_size.width - window_size.width) / 2;
                let position_y = (screen_size.height - window_size.height) / 2;

                window.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                    x: position_x as i32,
                    y: position_y as i32,
                })).unwrap();
            }

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_sounds,
            toggle_play,
            change_volume,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}