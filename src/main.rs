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

mod string;
use string::{StringState, StringParams};



#[derive(Component, Debug, Clone)]
struct NotePosition(usize);


#[derive(Component, Clone, Copy)]
struct Angle(f32);


#[derive(Event)]
struct UpdateNoteMapping;

#[derive(Event)]
struct ChordChange;

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
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_audio_source::<Synth>()
        .add_event::<UpdateNoteMapping>()
        .add_event::<ChordChange>()
        .add_systems(Startup, setup)
        .add_systems(Startup, init_string)
        .add_systems(PostStartup, create_circle)

        .add_systems(Update, create_note_names.run_if(on_event::<UpdateNoteMapping>()))
        .add_systems(Update, create_samples.run_if(on_event::<UpdateNoteMapping>()))
        .add_systems(Update, change_string.run_if(on_event::<ChordChange>()))


        .add_systems(Update, keyboard_input_system)
        .add_systems(Update, draw_notes)
        .add_systems(Update, draw_string)
        .add_systems(Update, update_string)
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

const N: usize = 150;

fn init_string(
    mut commands: Commands
    ){
    commands.spawn(
        VibratingString {
            params: StringParams {
                length: 500.,
                n_samples: N,
                dt: 0.005,
                c: 50.,
                junctions: vec![],
                spring_coeff: 0.00,
                solid_friction_coeff: 10.,
                liquid_friction_coeff: 0.005,
                steps_per_render: 100,
            },
            state: StringState::new_flat(N)
        }
    );
}

pub static NOTE_NAMES: [&'static str; 12] = 
    ["la", "la#", "si", "do", "do#", "re", "re#", "mi", "fa", "fa#", "sol", "sol#"];


#[derive(Bundle, Debug, Clone)]
struct VibratingString {
    params: StringParams,
    state: StringState,
}

fn change_string(
    mut string: Query<(&mut StringState, &StringParams)>,
) {
    if let Ok((mut s, p)) = string.get_single_mut() {
        *s = StringState::new_sinusoid(p.n_samples);
    }
}

fn update_string(
    mut string: Query<(&mut StringState, &mut StringParams)>,
    notes: Query<&NotePosition, With<Playing>>
) {
    if let Ok((mut s, mut params)) = string.get_single_mut() {
        let mut junctions = Vec::new();
        let mut n = 0usize;
        for p in &notes {
            let r : f32 = BASE_FREQUENCY / p.to_freq(0);
            n = usize::max(n, (N as f32 * r) as usize);
            junctions.push((r*N as f32) as usize)
        }
        params.junctions =  junctions;
        params.n_samples =  n;
        params.length = (n as f32 / N as f32) * 500.;

        if s.n_samples() == n {
            for _ in 0..params.steps_per_render {
                s.step(&params);
            }
        }
        else {
            *s = StringState::new_sinusoid(params.n_samples);
        }
    }
}

fn draw_string(
    gizmos: Gizmos,
    string: Query<(&StringState, &StringParams)>
) {
    if let Ok((s, p)) = string.get_single() {
        s.draw(p, gizmos)
    }
}

