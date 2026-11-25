use rnb_core::Song;

use crate::AudioSession;

pub trait RNBPlatform {
    fn create_audio_session(&self, song: &Song) -> AudioSession;
    fn pick_file(&self) -> Option<(String, Vec<u8>)>;
}
