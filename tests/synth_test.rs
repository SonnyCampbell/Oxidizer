use std::sync::mpsc::{Sender, Receiver, channel};

use oxidizer::synthesizer::{Synthesizer, SynthEvent};

#[test]
fn do_thing(){
    let (_, synth_receiver): (Sender<SynthEvent>, Receiver<SynthEvent>) = channel();

    let mut synth = Synthesizer::new(synth_receiver);

    let sample = synth.get_synth_sample();

    assert_eq!(sample, 0.0, "Initialized synth should return an empty sample");
}