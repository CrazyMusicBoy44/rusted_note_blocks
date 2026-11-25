pub(crate) struct Mixer {

}

#[derive(Default)]
struct SampleCache {
    samples: HashMap<InstrumentIndex, !>,
}

impl SampleCache {
    fn get_sample_from_note(
        &mut self,
        note: &Note,
        song: &Song,
    ) -> Option<()> {
        self.get_sample(note.instrument_index, song)
    }

    fn get_sample(
        &mut self,
        instrument_index: InstrumentIndex,
        song: &Song,
    ) -> Option<()> {
        if let Some(sample) = self.samples.get(&instrument_index) {
            return Some(sample.clone());
        }
        if let Some(instrument) = song.instruments.get_instrument(instrument_index) {
            let sample = ();
            self.samples.insert(instrument_index, sample.clone());
            return Some(sample);
        }
        None
    }
}
