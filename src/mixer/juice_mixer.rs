use crate::mixer::mixer::MixerStrategy;

pub struct JuiceMixerStrategy;

impl MixerStrategy for JuiceMixerStrategy{
    fn mix(&self, fruit: &str) -> Box<str> {
        let juice = format!("{} {}", fruit, "Juice");
        return Box::from(juice);
    }
}