pub trait AudioSession {
    fn next_buffer(&mut self, samples: &mut [f32]);
    fn is_done(&self) -> bool;
}
