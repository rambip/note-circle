use bevy::prelude::*;
use bevy::audio::AudioPlugin;
use bevy::audio::AddAudioSource;

#[derive(Component)]
struct Playing(bool);

#[derive(Resource)]
struct ChordJustChanged(bool);

#[derive(Resource)]
struct BaseNote(usize);

use std::f32::consts::PI;

const BASE_FREQUENCY: f32 = 55.0;

const N_OCTAVES : usize = 2;

const STRING_LENGTH: f32 = 500.;

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

    fn note(&self, base_note: usize) -> Note {
        let i = self.0 + base_note;
        Note(i as f32 / 12.0)
    }

    fn angle(&self) -> Angle {
        Angle(PI/2. - 2. * PI * (self.oclock() as f32) / 12.)
    }

    fn name(&self, base_note: usize) -> &'static str {
        let i = (self.0 + base_note) % 12;
        NOTE_NAMES[i]
    }
}

// Note(r) represents the note with frequency f such that
// 2**(r)*BASE_FREQUENCY = f
#[derive(Copy, Clone, Debug)]
struct Note(f32);

impl Note {
    fn to_freq(self) -> f32 {
        2.0f32.powf(self.0) * BASE_FREQUENCY
    }

    fn color(self) -> Color {
        Color::rgb(
            0.9,
            self.0 / 2.,
            self.0 % 1.0,
        )
    }

    fn relative_length(self) -> f32 {
        2.0f32.powf(-self.0 as f32)
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
        .add_systems(Startup, setup)
        .add_systems(Startup, init_string)
        .add_systems(PostStartup, create_circle)

        .add_systems(Update, create_note_names.run_if(on_event::<UpdateNoteMapping>()))
        .add_systems(Update, create_samples.run_if(on_event::<UpdateNoteMapping>()))
        .add_systems(Update, change_string)


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
    commands.insert_resource(ChordJustChanged(false));

    for height in 0..N_OCTAVES {
        for i in 0..12 {
            let note_pos = NotePosition::new(i, height);

            let angle = note_pos.angle();
            commands.spawn((note_pos, angle, Playing(false)));
        }
    }

    change_mapping.send(UpdateNoteMapping);


    let text_style : TextStyle = TextStyle {
        color: Color::WHITE,
        font_size: 16.,
        font: Default::default(),
    };

    let note_text = Text2dBundle {
        text: Text::from_section("Be sure to be on qwerty, and start typing on keys ...", 
                                 text_style.clone()),
        transform: Transform::from_translation(
            Vec3::new(0., -300., -1.)
        ),
        ..default()
    };

    commands.spawn(note_text);

}

const N: usize = 150;

fn init_string(
    mut commands: Commands
    ){
    commands.spawn(
        VibratingString {
            params: StringParams {
                length: STRING_LENGTH,
                n_samples: N,
                dt: 0.02,
                c: 100.,
                chord: vec![],
                spring_coeff: 10.00,
                solid_friction_coeff: 50.,
                liquid_friction_coeff: 0.015,
                steps_per_render: 10,
                excitation_coeff: 0.05,
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
    mut string: Query<(&mut StringState, &mut StringParams)>,
    notes: Query<(&NotePosition, &Playing)>,
    mut chord_changed: ResMut<ChordJustChanged>,
) {
    if !chord_changed.0 {
        return
    }

    chord_changed.0 = false;

    let (mut s, mut p) = match string.get_single_mut() {
        Ok(a) => a,
        Err(_) => return
    };

    let notes: Vec<_> = notes.iter().filter(|(_, p)| p.0).map(|(x, _)| x).collect();

    // FIXME: base_note = 0 ?
    let r0 = notes.iter().map(|note_position| note_position.note(0))
        .fold(f32::NEG_INFINITY, |x, note| note.relative_length().max(x));

    p.n_samples = (N as f32 * r0) as usize;
    p.length = r0 * 500.;
    p.chord = Vec::new();
    *s = StringState::new_flat(p.n_samples);

    for note_position in notes.iter() {
        let note = note_position.note(0);
        p.chord.push(note);
    }

    for _ in 0..(p.length/p.dt) as usize{
        s.step(&p);
    }
}

fn update_string(
    mut string: Query<(&mut StringState, &mut StringParams)>,
) {
    if let Ok((mut s, params)) = string.get_single_mut() {
        for _ in 0..params.steps_per_render {
            s.step(&params);
        }
    }
}

fn draw_string(
    gizmos: Gizmos,
    string: Query<(&StringState, &StringParams)>,
) {
    if let Ok((s, p)) = string.get_single() {
        s.draw(p, gizmos)
    }
}
