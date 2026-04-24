use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;
use rand::RngExt;

pub const DRIFT_STEP_MS: u64 = 10;
const DRIFT_DURATION_MS: u64 = 1500;

pub(crate) fn song_drift(sound: &mut crate::utils::sound_stream::SoundStream) {
    if !sound.data.play {
        return;
    }

    let play_flag = sound.play.clone();
    let Some(player) = sound.player.clone() else {
        return;
    };

    let base_volume = sound.data.volume;

    thread::spawn(move || {
        let mut rng = rand::rng();

        while play_flag.load(Ordering::Relaxed) {
            let time_until_next_effect = rng.random_range(5..10);

            thread::sleep(Duration::from_secs(time_until_next_effect));

            println!("drift");

            if !play_flag.load(Ordering::Relaxed) {
                return;
            }

            let current_volume = match player.lock() {
                Ok(player) => player.volume(),
                Err(_) => return,
            };

            let variation = rng.random_range(-0.05..0.05);

            let target_volume = (base_volume + variation).clamp(
                base_volume * 0.85,
                base_volume * 1.15,
            );

            println!("current_volume: {}", current_volume);
            println!("target_volume: {}", target_volume);

            let start_volume = current_volume;
            let steps = DRIFT_DURATION_MS / DRIFT_STEP_MS;

            for step in 0..=steps {
                if !play_flag.load(Ordering::Relaxed) {
                    return;
                }

                let t = step as f32 / steps as f32;
                let eased = t * t * (3.0 - 2.0 * t);

                let volume = start_volume + (target_volume - start_volume) * eased;

                if let Ok(player) = player.lock() {
                    player.set_volume(volume);
                }

                thread::sleep(Duration::from_millis(DRIFT_STEP_MS));
            }

            println!("drift done");
        }
    });
}