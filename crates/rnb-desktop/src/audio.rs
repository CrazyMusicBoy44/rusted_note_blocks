pub(crate) fn test_audio() -> tinyaudio::OutputDevice {
    let params = tinyaudio::OutputDeviceParameters {
        channels_count: 2,
        sample_rate: 48000,
        channel_sample_count: 4096,
    };

    let mut phase = 0.0_f32;
    let freq = 440.0;
    let volume = 0.1;

    tinyaudio::run_output_device(params, move |samples: &mut [f32]| {
        for frame in samples.chunks_exact_mut(params.channels_count) {
            let sample = if phase < 0.5 { volume } else { -volume };
            frame.fill(sample);
            phase = (phase + freq / params.sample_rate as f32) % 1.0;
        }
    }).expect("i expect something here but it was error")
}
