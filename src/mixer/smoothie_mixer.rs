use crate::mixer::mixer::MixerStrategy;

pub struct SmoothieMixerStrategy;

impl MixerStrategy for SmoothieMixerStrategy {
    fn mix(&self, ingredient: &str) -> Box<str> {
        let smoothie = format!("{} {}", ingredient, "Smoothie");
        return Box::from(smoothie);
    }
}