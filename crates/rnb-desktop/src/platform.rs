use rnb_gui::RNBPlatform;

use crate::audio::DesktopAudio;

pub struct DesktopPlatform {}

impl RNBPlatform for DesktopPlatform {
    type AudioSession = DesktopAudio;

    fn create_audio_session(&self, _song: &rnb_core::Song) -> Self::AudioSession {
        Self::AudioSession::new()
    }

    fn pick_file(&self) -> Option<(String, Vec<u8>)> {
        None
    }
}
