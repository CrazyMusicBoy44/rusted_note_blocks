use rnb_core::Song;

use crate::AudioSession;

pub trait RNBPlatform {
    type AudioSession: AudioSession;

    fn create_audio_session(&self, song: &Song) -> Self::AudioSession;
    fn pick_file(&self) -> Option<(String, Vec<u8>)>;
}
