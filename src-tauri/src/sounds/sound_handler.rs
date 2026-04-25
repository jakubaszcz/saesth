use std::fs::File;
use std::sync::{Arc, Mutex};
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;
use rodio::{Decoder, DeviceSinkBuilder, Player, Source};
use crate::sounds::apply_sound::apply_sound;
use crate::sounds::drift::sound_drift::song_drift;
use crate::utils::sound_stream::SoundStream;
pub(crate) const FADE_STEPS: u64 = 5;
const FADE_DURATION_MS: u64 = 1500;

pub fn play_sound(id: &str, sound: &mut SoundStream) {
    sound.play.store(true, Ordering::Relaxed);

    let path = format!("{}/{}.mp3", sound.data.path, "default");

    let handle = DeviceSinkBuilder::open_default_sink()
        .expect("failed to open default audio device");

    let player = Arc::new(Mutex::new(
        Player::connect_new(&handle.mixer())
    ));

    let file = File::open(&path)
        .expect("failed to open audio file");
    let source = Decoder::try_from(file)
        .expect("failed to decode audio file")
        .repeat_infinite();

    player.lock().unwrap().append(source);
    player.lock().unwrap().set_volume(0.0);
    player.lock().unwrap().play();

    let fade_volume = sound.fade_volume.clone();
    let user_volume = sound.volume.clone();
    let drift_volume = sound.drift_volume.clone();

    let clone_player = player.clone();
    let play_flag = sound.play.clone();

    {
        *fade_volume.lock().unwrap() = 0.0;
    }

    thread::spawn(move || {
        let steps = FADE_DURATION_MS / FADE_STEPS;

        for step in 0..=steps {

            if !play_flag.load(Ordering::Relaxed) {
                return;
            }

            let t = step as f32 / steps as f32;
            let eased = t * t;

            *fade_volume.lock().unwrap() = eased;

            apply_sound(
                &clone_player,
                &user_volume,
                &fade_volume,
                &drift_volume
            );


            thread::sleep(Duration::from_millis(FADE_STEPS));
        }
    });

    sound.player = Some(player);
    sound.handle = Some(handle);
    sound.data.play = true;

    song_drift(sound);
}
pub fn stop_sound(sound: &mut SoundStream) {
    sound.play.store(false, Ordering::Relaxed);

    let Some(player) = sound.player.take() else {
        return;
    };
    let Some(handle) = sound.handle.take() else {
        return
    };

    let fade_volume = sound.fade_volume.clone();
    let user_volume = sound.volume.clone();
    let drift_volume = sound.drift_volume.clone();

    let fade_player = player.clone();
    let play_flag = sound.play.clone();

    sound.data.play = false;

    thread::spawn(move || {
        let steps = FADE_DURATION_MS / FADE_STEPS;

        for step in (0..=steps).rev() {
            if play_flag.load(Ordering::Relaxed) {
                return;
            }

            let t = step as f32 / steps as f32;
            let eased = t * t;

            *fade_volume.lock().unwrap() = eased;

            apply_sound(
                &fade_player,
                &user_volume,
                &fade_volume,
                &drift_volume,
            );

            thread::sleep(Duration::from_millis(FADE_STEPS));
        }

        drop(player);
        drop(handle);
    });
}
