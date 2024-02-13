use bevy::prelude::*;

use super::{Playing, Note};

static KEYS2: [KeyCode; 12] = [
    KeyCode::Key1, 
    KeyCode::Key2, 
    KeyCode::Key3,
    KeyCode::Key4,
    KeyCode::Key5,
    KeyCode::Key6,
    KeyCode::Key7,
    KeyCode::Key8,
    KeyCode::Key9,
    KeyCode::Key0,
    KeyCode::Minus,
    KeyCode::Equals,
];

static KEYS1: [KeyCode; 12] = [
    KeyCode::Q, 
    KeyCode::W, 
    KeyCode::E,
    KeyCode::R,
    KeyCode::T,
    KeyCode::Y,
    KeyCode::U,
    KeyCode::I,
    KeyCode::O,
    KeyCode::P,
    KeyCode::BracketLeft,
    KeyCode::BracketRight,
];

pub fn keyboard_input_system(mut commands: Commands, keyboard_input: Res<Input<KeyCode>>, query: Query<(Entity, &Note)>) {
    for (e, note) in &query {
        if note.octave == 0 {
            let k = KEYS1[note.number as usize];
            if keyboard_input.pressed(k) {
                commands.entity(e).insert(Playing);
            }
            else {
                commands.entity(e).remove::<Playing>();
            }
        }
        if note.octave == 1 {
            let k = KEYS2[note.number as usize];
            if keyboard_input.pressed(k) {
                commands.entity(e).insert(Playing);
            }
            else {
                commands.entity(e).remove::<Playing>();
            }
        }
    }
}
