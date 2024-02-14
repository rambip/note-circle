use bevy::prelude::*;
use bevy::audio::AudioPlugin;
use bevy::audio::AddAudioSource;

#[derive(Component)]
struct Playing;

#[derive(Resource)]
struct BaseNote(usize);

use std::f32::consts::PI;

const BASE_FREQUENCY: f32 = 55.0;

const N_OCTAVES : usize = 2;

mod sound;
use sound::{Synth, create_samples};

mod keyboard;
use keyboard::keyboard_input_system;

mod circle;
use circle::{create_circle, draw_notes, create_note_names};



#[derive(Component, Debug, Clone)]
struct NotePosition(usize);

#[derive(Component, Clone, Copy)]
struct Angle(f32);


#[derive(Event)]
struct UpdateNoteMapping;

impl NotePosition {
    fn new(number: usize, octave: usize) -> Self {
        NotePosition(octave * 12 + number)
    }

    fn height(&self) -> usize {
        self.0 / 12
    }

    fn oclock(&self) -> usize {
        self.0 % 12
    }

    fn to_freq(&self, base_note: usize) -> f32 {
        let i = self.0 + base_note;
        BASE_FREQUENCY * 2.0f32.powf(i as f32 / 12.0)
    }

    fn name(&self, base_note: usize) -> &'static str {
        let i = (self.0 + base_note) % 12;
        NOTE_NAMES[i]
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AudioPlugin {
            global_volume: GlobalVolume::new(0.2),
            ..default()
        }))
        .add_audio_source::<Synth>()
        .add_event::<UpdateNoteMapping>()
        .add_systems(Startup, setup)
        .add_systems(PostStartup, create_circle)

        .add_systems(Update, create_note_names.run_if(on_event::<UpdateNoteMapping>()))
        .add_systems(Update, create_samples.run_if(on_event::<UpdateNoteMapping>()))


        .add_systems(Update, keyboard_input_system)
        .add_systems(Update, play_notes)
        .add_systems(Update, draw_notes)
        .run();
}

fn setup(
    mut commands: Commands,
    mut change_mapping: EventWriter<UpdateNoteMapping>,
    ) {
    commands.spawn(Camera2dBundle::default());

    // number of half tones from A0
    commands.insert_resource(BaseNote(27));

    for height in 0..N_OCTAVES {
        for i in 0..12 {
            let note_pos = NotePosition::new(i, height);

            let angle = Angle(PI/2. - 2. * PI * (i as f32) / 12.);
            commands.spawn((note_pos, angle));
        }
    }

    change_mapping.send(UpdateNoteMapping);

}

pub static NOTE_NAMES: [&'static str; 12] = 
    ["la", "la#", "si", "do", "do#", "re", "re#", "mi", "fa", "fa#", "sol", "sol#"];




fn play_notes(
              new_notes: Query<&AudioSink, With<Playing>>,
              dead_notes: Query<&AudioSink, Without<Playing>>
              ) {
    for sink in &new_notes {
        sink.play()
    }
    for sink in &dead_notes {
        sink.pause()
    }

}

