pub trait MixerStrategy {
    fn mix(&self, item: &str) -> Box<str>;
}

pub struct Mixer {
    pub mixer_strategy: Box<dyn MixerStrategy>,
}

impl Mixer {
    pub fn mix(&self, item: &str) -> Box<str> {
        return (*self.mixer_strategy).mix(item);
    }
}