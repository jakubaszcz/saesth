use rodio::{MixerDeviceSink, Player};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct SoundData {
    pub id: String,
    pub play: bool,
    pub path: String,
    pub volume: f32
}

pub struct SoundStream {
    pub handle: Option<MixerDeviceSink>,
    pub player: Option<Player>,
    pub data: SoundData
}

pub type SoundList = Vec<SoundStream>;