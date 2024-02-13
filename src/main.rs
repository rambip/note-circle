use bevy::prelude::*;
use bevy::audio::AudioPlugin;
use bevy::audio::AddAudioSource;

use bevy::sprite::MaterialMesh2dBundle;

#[derive(Component)]
struct Chord(u64);

#[derive(Component)]
struct Playing;



mod sound;
use sound::{Synth, SAX_SPECTRUM};

mod keyboard;
use keyboard::keyboard_input_system;

use std::f32::consts::PI;

#[derive(Component, Debug, Clone)]
struct Note { octave: u8, number: u8}

impl Note {
    fn name(&self) -> &str {
        NOTE_NAMES[self.number as usize]
    }

    fn to_freq(&self) -> f32 {
        return 220.0 * 2.0f32.powf(self.octave as f32 + self.number as f32 / 12.0)
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AudioPlugin {
            global_volume: GlobalVolume::new(0.2),
            ..default()
        }))
        .add_audio_source::<Synth>()
        .add_systems(Startup, (create_notes, setup))
        .add_systems(Update, keyboard_input_system)
        .add_systems(Update, play_notes)
        .add_systems(Update, draw_notes)
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(120.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::WHITE)),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    });

}

static NOTE_NAMES: [&'static str; 12] = ["do", "do#", "re", "re#", "mi", "fa", "fa#", "sol", "sol#", "la", "la#", "si"];





fn create_notes(mut assets: ResMut<Assets<Synth>>, mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>){
    let text_style : TextStyle = Default::default();

    for i in 0..12 {
        let note = Note {octave: 0, number: i};
        let angle = PI/2. - 2. * PI * (note.number as f32) / 12.;

        commands.spawn(Text2dBundle {
            text: Text::from_section(NOTE_NAMES[i as usize], text_style.clone()),
            transform: Transform::from_translation(140. * Vec2::from_angle(angle).extend(0.)),
            ..default()
        });

        for octave in 0..=1 {
            let rad = 100. + 10. * octave as f32;

            let note = Note {octave, number: i};

            let sound = AudioSourceBundle {
                source: assets.add(Synth::new(note.to_freq(), SAX_SPECTRUM.into())),
                ..Default::default()
            };


            let circle = MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(10.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                transform: Transform::from_translation(rad * Vec2::from_angle(angle).extend(1.)),
                ..default()
            };

            commands.spawn((note, Chord(0), sound, circle));
        }
    }
}


fn play_notes(mut commands: Commands, 
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

fn draw_notes(mut gizmos: Gizmos, 
              mut new_notes: Query<(&Note, &mut Visibility), With<Playing>>,
              mut dead_notes: Query<(&Note, &mut Visibility), Without<Playing>>
              ) {
    for (note, mut visible) in new_notes.iter_mut() {
        *visible = Visibility::Visible;
        // let angle = PI/2. - 2. * PI * (note.number as f32) / 12.;
        // let pos = 120. * Vec2::new(f32::cos(angle), f32::sin(angle));
        // gizmos.circle_2d(pos, 20., Color::RED);
    }
    for (note, mut visible) in dead_notes.iter_mut() {
        *visible = Visibility::Hidden;
        // let angle = PI/2. - 2. * PI * (note.number as f32) / 12.;
        // let pos = 120. * Vec2::new(f32::cos(angle), f32::sin(angle));
        // gizmos.circle_2d(pos, 20., Color::RED);
    }
}

