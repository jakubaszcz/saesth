use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use rodio::{MixerDeviceSink, Player};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct SoundData {
    pub id: String,
    pub play: bool,
    pub path: String,
    pub volume: f32
}

pub struct SoundStream {

    pub effect: SoundEffect,

    pub handle: Option<MixerDeviceSink>,
    pub player: Option<Arc<Mutex<Player>>>,
    pub play: Arc<AtomicBool>,
    pub volume: Arc<Mutex<f32>>,
    pub fade_volume: Arc<Mutex<f32>>,
    pub drift_volume: Arc<Mutex<f32>>,
    pub data: SoundData
}

#[derive(Clone)]
pub struct SoundEffect {
    pub player: Option<Arc<Mutex<Player>>>,
    pub path: String
}

pub type SoundList = Vec<SoundStream>;