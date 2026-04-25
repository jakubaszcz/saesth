use std::fs::File;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;

use rand::RngExt;
use rodio::{Decoder, Player};

use crate::sounds::random_sound;
use crate::utils::sound_stream::SoundStream;

pub fn thunder(sound: &mut SoundStream) {
    let play_flag = sound.play.clone();
    let user_volume = sound.volume.clone();
    let drift_volume = sound.drift_volume.clone();
    let fade_volume = sound.fade_volume.clone();

    let effect = sound.effect.clone() else {
        return;
    };

    let Some(handle) = sound.handle.as_ref() else {
        return;
    };

    let mixer = handle.mixer().clone();

    thread::spawn(move || {
        while play_flag.load(Ordering::Relaxed) {
            let wait = rand::rng().random_range(10..100);
            thread::sleep(Duration::from_secs(wait));

            if !play_flag.load(Ordering::Relaxed) {
                return;
            }

            let path = random_sound::random_sound(effect.path.as_str());

            let Ok(file) = File::open(&path) else {
                continue;
            };

            let Ok(source) = Decoder::try_from(file) else {
                continue;
            };

            let player = Player::connect_new(&mixer);

            let bonus = rand::rng().random_range(0.1..0.6);
            let user = *user_volume.lock().unwrap();
            let fade = *fade_volume.lock().unwrap();
            let drift = *drift_volume.lock().unwrap();

            let volume = (user * fade * drift * bonus).clamp(0.0, 1.0);

            player.set_volume(volume);
            player.append(source);
            player.play();

            player.sleep_until_end();
        }
    });
}