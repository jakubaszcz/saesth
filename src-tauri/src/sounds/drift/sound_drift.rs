use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;
use rand::RngExt;
use crate::sounds::apply_sound::apply_sound;

pub const DRIFT_STEP_MS: u64 = 50;
const DRIFT_DURATION_MS: u64 = 4500;

pub(crate) fn song_drift(sound: &mut crate::utils::sound_stream::SoundStream) {
    if !sound.data.play {
        return;
    }

    let fade_volume = sound.fade_volume.clone();
    let user_volume = sound.volume.clone();
    let drift_volume = sound.drift_volume.clone();

    let play_flag = sound.play.clone();
    let Some(player) = sound.player.clone() else {
        return;
    };

    let min = 10;
    let max = 60;

    let min_bonus = 0.85;
    let max_bonus = 1.35;

    thread::spawn(move || {
        let mut rng = rand::rng();

        while play_flag.load(Ordering::Relaxed) {
            let time_until_next_effect = rng.random_range(10..100);

            thread::sleep(Duration::from_secs(time_until_next_effect));

            if !play_flag.load(Ordering::Relaxed) {
                return;
            }

            let start_drift = match drift_volume.lock() {
                Ok(value) => *value,
                Err(_) => return,
            };

            let target_drift = rng.random_range(min_bonus..max_bonus);

            let steps = DRIFT_DURATION_MS / DRIFT_STEP_MS;

            for step in 0..=steps {
                if !play_flag.load(Ordering::Relaxed) {
                    return;
                }

                let t = step as f32 / steps as f32;
                let eased = t * t * (3.0 - 2.0 * t);

                let drift = start_drift + (target_drift - start_drift) * eased;

                if let Ok(mut drift_value) = drift_volume.lock() {
                    *drift_value = drift;
                }

                apply_sound(
                    &player,
                    &user_volume,
                    &fade_volume,
                    &drift_volume,
                );


                thread::sleep(Duration::from_millis(DRIFT_STEP_MS));

            }
        }
    });
}