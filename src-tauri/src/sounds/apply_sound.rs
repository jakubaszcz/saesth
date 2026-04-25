use std::sync::{Arc, Mutex};
use rodio::Player;

pub fn apply_sound(
    player: &Arc<Mutex<Player>>,
    user_volume: &Arc<Mutex<f32>>,
    fade_volume: &Arc<Mutex<f32>>,
    drift_volume: &Arc<Mutex<f32>>,
) {

    let user_volume = user_volume.lock().unwrap();
    let fade_volume = fade_volume.lock().unwrap();
    let drift_volume = drift_volume.lock().unwrap();

    let final_volume = (*user_volume * *fade_volume * *drift_volume).clamp(0.0, 1.0);

    if let Ok(player) = player.lock() {
        player.set_volume(final_volume);
    }

}