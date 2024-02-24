use bevy::prelude::*;
use bevy::audio::PlaybackMode;

use bevy::audio::Source;
use bevy::utils::Duration;

use std::f32::consts::PI;

use super::{NotePosition, BaseNote};

static SAMPLE_RATE: u32 = 44_100;

pub static SINE_SPECTRUM: [Sinusoid; 3] = [
    Sinusoid {amplitude: 0.25, phase: 0., frequency_multiple: 1.0},
    Sinusoid {amplitude: 0.23, phase: 0., frequency_multiple: 1.03},
    Sinusoid {amplitude: 0.21, phase: 0., frequency_multiple: 1.05},
];

// pub static TRIANGLE_SPECTRUM: [Sinusoid; 5] = [
//     Sinusoid::NULL,
//     Sinusoid {amplitude: 0.5, phase: 0.},
//     Sinusoid {amplitude: 0.25, phase: PI},
//     Sinusoid {amplitude: 0.05, phase: 0.},
//     Sinusoid {amplitude: 0.02, phase: PI},
// ];

// pub static SAX_SPECTRUM: [Sinusoid; 21] = [
//     Sinusoid::NULL,
//     Sinusoid {amplitude:0.42974835498737607, phase: -3.040638298720167},
//     Sinusoid {amplitude:0.5232032128896923, phase: -0.3185561184424239},
//     Sinusoid {amplitude:0.16455222770418948, phase: -2.7127386118570374},
//     Sinusoid {amplitude:0.41940857280870986, phase: -0.032513108159311264},
//     Sinusoid {amplitude:0.2037244815842362, phase: 2.744218818630659},
//     Sinusoid {amplitude:0.13709849851939704, phase: -2.983734880089535},
//     Sinusoid {amplitude:0.2184466780279849, phase: -2.7789351049301025},
//     Sinusoid {amplitude:0.21702421804588365, phase: -0.05129480237414914},
//     Sinusoid {amplitude:0.24383123604430218, phase: -1.0136584546674183},
//     Sinusoid {amplitude:0.17054145208958676, phase: -1.9926780682697318},
//     Sinusoid {amplitude:0.09102340082812721, phase: -1.8590703381419607},
//     Sinusoid {amplitude:0.06549629181234591, phase: -2.4808378267655655},
//     Sinusoid {amplitude:0.09670327471337731, phase: 0.1468344926846707},
//     Sinusoid {amplitude:0.10382325864286551, phase: -0.18503120762040676},
//     Sinusoid {amplitude:0.15342047537704315, phase: -0.8646085739076568},
//     Sinusoid {amplitude:0.13924698203713623, phase: 2.045216000657128},
//     Sinusoid {amplitude:0.09533112782360806, phase: 1.1155465441094896},
//     Sinusoid {amplitude:0.05752474568031794, phase: 1.3575149512854667},
//     Sinusoid {amplitude:0.06032919698209148, phase: -2.222642381089361},
//     Sinusoid {amplitude:0.05716175751735669, phase: -2.9076408446978794},
// ];

// FIXME: les notes ne sont pas encore générées quand cette fonction est appelée
pub fn create_samples(base_note: Res<BaseNote>,
                      mut assets: ResMut<Assets<Synth>>,
                      mut commands: Commands,
                      query: Query<(Entity, &NotePosition)>,
                      ) {

    for (e, note) in &query {
        let sound = AudioSourceBundle {
            source: assets.add(Synth::new(note.note(base_note.0).to_freq(), SINE_SPECTRUM.into())),
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
    frequency_multiple: f32,
}

impl Sinusoid {
    fn generate_signal(&self, current_phase: f32) -> f32 {
        self.amplitude * f32::cos(self.frequency_multiple * current_phase - self.phase)
    }
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
            .map(|coeff| coeff.generate_signal(self.current_phase))
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
