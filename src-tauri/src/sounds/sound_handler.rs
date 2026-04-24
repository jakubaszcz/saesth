use std::fs::File;
use rodio::{Decoder, DeviceSinkBuilder, Player, Source};
use crate::utils::sound_stream::SoundStream;

pub fn play_sound(sound: &mut SoundStream) {
    let handle = DeviceSinkBuilder::open_default_sink()
        .expect("failed to open default audio device");

    let player = Player::connect_new(&handle.mixer());

    let file = File::open(&sound.data.path)
        .expect("failed to open audio file");

    let source = Decoder::try_from(file)
        .expect("failed to decode audio file")
        .repeat_infinite();

    player.append(source);
    player.set_volume(sound.data.volume);

    sound.player = Some(player);
    sound.handle = Some(handle);
    sound.data.play = true;
}

pub fn stop_sound(sound: &mut SoundStream) {
    if let Some(player) = sound.player.take() {
        drop(player);
    }
    if let Some(handle) = sound.handle.take() {
        drop(handle);
    }
    sound.data.play = false;
}
