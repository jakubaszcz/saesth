use std::fs::File;
use std::io::Repeat;
use std::sync::{Arc, Mutex, OnceLock};
use std::sync::atomic::AtomicBool;
use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Player, Source};
use tauri::async_runtime::handle;
use tauri::{menu::{Menu, MenuItem}, tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent}, utils as other_utils, App, AppHandle, Emitter, Manager};
use tauri::image::Image;
use crate::utils::init_tray::init_tray;

mod database;
mod utils;
mod sounds;

static SOUND_LIST: OnceLock<Mutex<utils::sound_stream::SoundList>> = OnceLock::new();


fn init_sounds() {
    let mut list = Vec::new();
    list.push(utils::sound_stream::SoundStream {
        handle: None,
        player: None,
        play: Arc::new(AtomicBool::new(false)),
        data: utils::sound_stream::SoundData {
            id: "rain".to_string(),
            play: false,
            volume: database::database::get_volume("rain"),
            path: "sounds/rain".to_string(),
        }
    });
    list.push(utils::sound_stream::SoundStream {
        handle: None,
        player: None,
        play: Arc::new(AtomicBool::new(false)),
        data: utils::sound_stream::SoundData {
            id: "fire".to_string(),
            play: false,
            volume: database::database::get_volume("fire"),
            path: "sounds/fire".to_string(),
        }
    });


    SOUND_LIST.get_or_init(|| Mutex::new(list));
}

#[tauri::command]
fn get_sounds() -> Vec<utils::sound_stream::SoundData> {
    let list = SOUND_LIST.get().unwrap().lock().unwrap();
    list.iter().map(|v| v.data.clone()).collect()
}

#[tauri::command]
fn change_volume(id: String, volume: f32) -> Vec<utils::sound_stream::SoundData> {
    let mut list = SOUND_LIST.get().unwrap().lock().unwrap();

    if let Some(sound) = list.iter_mut().find(|s| s.data.id == id) {
        sound.data.volume = volume;
        database::database::set_volume(&id, volume);
        if let Some(player) = &sound.player {
            player.lock().unwrap().set_volume(volume);
        }
    }
    list.iter()
        .map(|v| v.data.clone())
        .collect()
}

#[tauri::command]
fn set_settings(id: String, value: String) {
    database::database::set_setting(&*id, &*value);
}

#[tauri::command]
fn get_settings(id: String) -> String {
    database::database::get_setting(id.as_str())
}

#[tauri::command]
fn toggle_play(id: String) -> Vec<utils::sound_stream::SoundData> {
    let mut list = SOUND_LIST.get().unwrap().lock().unwrap();

    if let Some(sound) = list.iter_mut().find(|s| s.data.id == id) {
        if sound.data.play {
            sounds::sound_handler::stop_sound(sound);
        } else {
            sounds::sound_handler::play_sound(&id, sound);
        }
    }

    list.iter()
        .map(|v| v.data.clone())
        .collect()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    database::database::init_db();

    let defaults = ["rain", "fire"];
    for default in defaults {
        database::database::create_if_missing(default);
    }

    database::database::init_database_settings();

    init_sounds();

    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            let handle = app.handle().clone();

            // Tray
            init_tray(app);

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
        .on_window_event(|window, event| {
            if database::database::get_setting("close_to_tray") == "true" {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    window.hide().unwrap();
                }
            }
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_sounds,
            toggle_play,
            change_volume,
            get_settings,
            set_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}