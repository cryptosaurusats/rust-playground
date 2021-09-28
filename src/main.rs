use crate::mixer::mixer::Mixer;
use crate::mixer::juice_mixer::JuiceMixerStrategy;
use crate::mixer::smoothie_mixer::SmoothieMixerStrategy;

mod mixer;

fn main() {
    let my_fruit = "Apple";
    let juice_mixer = Mixer{mixer_strategy: Box::new(JuiceMixerStrategy)};
    let smoothie_mixer = Mixer{mixer_strategy: Box::new(SmoothieMixerStrategy)};
    println!("{}", juice_mixer.mix(&my_fruit));
    println!("{}", smoothie_mixer.mix(&my_fruit));
}