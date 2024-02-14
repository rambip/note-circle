use bevy::prelude::*;
use bevy::audio::PlaybackMode;

use bevy::audio::Source;
use bevy::utils::Duration;

use std::f32::consts::PI;

use super::{NotePosition, BaseNote};

static SAMPLE_RATE: u32 = 44_100;

pub static SINE_SPECTRUM: [Sinusoid; 2] = [
    Sinusoid::NULL,
    Sinusoid {amplitude: 1.0, phase: 0.},
];

pub static TRIANGLE_SPECTRUM: [Sinusoid; 5] = [
    Sinusoid::NULL,
    Sinusoid {amplitude: 1.0, phase: 0.},
    Sinusoid {amplitude: 0.5, phase: PI},
    Sinusoid {amplitude: 0.11, phase: 0.},
    Sinusoid {amplitude: 0.05, phase: PI},
];

pub static SAX_SPECTRUM: [Sinusoid; 8] = [
    Sinusoid::NULL,
    Sinusoid {amplitude: 1.0, phase: 0.},
    Sinusoid {amplitude: 0.5, phase: PI},
    Sinusoid {amplitude: 0.1, phase: 0.},
    Sinusoid {amplitude: 0.4, phase: 0.},
    Sinusoid {amplitude: 0.1, phase: 0.},
    Sinusoid {amplitude: 0.25, phase: PI},
    Sinusoid {amplitude: 0.0, phase: PI},
];

// FIXME: les notes ne sont pas encore générées quand cette fonction est appelée
pub fn create_samples(base_note: Res<BaseNote>,
                      mut assets: ResMut<Assets<Synth>>,
                      mut commands: Commands,
                      query: Query<(Entity, &NotePosition)>,
                      ) {

    for (e, note) in &query {
        let sound = AudioSourceBundle {
            source: assets.add(Synth::new(note.to_freq(base_note.0), TRIANGLE_SPECTRUM.into())),
            settings: PlaybackSettings {
                mode: PlaybackMode::Remove,
                ..Default::default()
            }
        };

        commands.entity(e).remove::<AudioSink>();
        commands.entity(e).insert(sound);
    }
}

#[derive(Clone, Copy, Default)]
pub struct Sinusoid {
    amplitude: f32,
    phase: f32,
}

impl Sinusoid {
    fn generate_signal(&self, current_phase: f32, order: usize) -> f32 {
        self.amplitude * f32::cos((order as f32) * current_phase - self.phase)
    }

   const NULL: Sinusoid = Sinusoid {amplitude: 0., phase: 0.};
}

#[derive(Asset, TypePath)]
pub struct Synth {
    frequency: f32,
    spectrum: Vec<Sinusoid>
}

impl Synth {
    pub fn new(frequency: f32, spectrum: Vec<Sinusoid>) -> Self {
        Self {
            frequency,
            spectrum
        }
    }
}

pub struct SynthDecoder {
    // how far along one period the wave is (between 0 and 1)
    current_phase: f32,
    step: f32,
    spectrum: Vec<Sinusoid>
}


impl SynthDecoder {
    fn new(frequency: f32, spectrum: Vec<Sinusoid>) -> Self {
        SynthDecoder {
            current_phase: 0.,
            step: 2.0 * PI * frequency / SAMPLE_RATE as f32,
            spectrum,
        }
    }
}

impl Iterator for SynthDecoder {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        // we loop back round to 2pi to avoid floating point inaccuracies
        self.current_phase = (self.current_phase + self.step)%(2.0 * PI);
        Some(
        self.spectrum
            .iter()
            .enumerate()
            .map(|(k, coeff)| coeff.generate_signal(self.current_phase, k))
            .sum::<f32>()
        )
    }
}
// `Source` is what allows the audio source to be played by bevy.
// This trait provides information on the audio.
impl Source for SynthDecoder {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl Decodable for Synth {
    type DecoderItem = <SynthDecoder as Iterator>::Item;

    type Decoder = SynthDecoder;

    fn decoder(&self) -> Self::Decoder {
        SynthDecoder::new(self.frequency, self.spectrum.clone())
    }
}
