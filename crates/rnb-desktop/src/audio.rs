pub fn test_audio() -> tinyaudio::OutputDevice {
    let params = OUTPUT_PARAMS;
    let mut phase = 0.0_f32;
    let freq = 440.0;
    let volume = 0.1;

    tinyaudio::run_output_device(params, move |samples: &mut [f32]| {
        for frame in samples.chunks_exact_mut(params.channels_count) {
            let sample = if phase < 0.5 { volume } else { -volume };
            frame.fill(sample);
            phase = (phase + freq / params.sample_rate as f32) % 1.0;
        }
    })
    .expect("i expect something here but it was error")
}

struct DesktopAudio {
    device: tinyaudio::OutputDevice,
}

impl rnb_gui::AudioSession for DesktopAudio {
    fn next_buffer(&mut self, samples: &mut [f32]) {
        let params = OUTPUT_PARAMS;
        let mut phase = 0.0_f32;
        let freq = 440.0;
        let volume = 0.1;

        for frame in samples.chunks_exact_mut(params.channels_count) {
            let sample = if phase < 0.5 { volume } else { -volume };
            frame.fill(sample);
            phase = (phase + freq / params.sample_rate as f32) % 1.0;
        }
    }

    fn is_done(&self) -> bool {
        false
    }
}

const OUTPUT_PARAMS: tinyaudio::OutputDeviceParameters = tinyaudio::OutputDeviceParameters {
    channels_count: 2,
    sample_rate: 48000,
    channel_sample_count: 4096,
};
