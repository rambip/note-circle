use bevy::prelude::*;

use super::{Playing, NotePosition, BaseNote, UpdateNoteMapping};

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

pub fn keyboard_input_system(
    mut mapping_changed: EventWriter<UpdateNoteMapping>,
    mut commands: Commands, 
    mut base_note: ResMut<BaseNote>,
    keyboard_input: Res<Input<KeyCode>>, 
    query: Query<(Entity, &AudioSink, &NotePosition)>
) {

    if keyboard_input.just_pressed(KeyCode::Right) {
        base_note.0 += 1;
        mapping_changed.send(UpdateNoteMapping);
        return
    }
    if keyboard_input.just_pressed(KeyCode::Left) && base_note.0 > 0 {
        base_note.0 -= 1;
        mapping_changed.send(UpdateNoteMapping);
        return
    }

    for (e, sink, note) in &query {
        if note.height() == 0 {
            let k = KEYS1[note.oclock() as usize];
            if keyboard_input.pressed(k) {
                commands.entity(e).insert(Playing);
                sink.play()
            }
            else {
                commands.entity(e).remove::<Playing>();
                sink.pause()
            }
        }
        if note.height() == 1 {
            let k = KEYS2[note.oclock() as usize];
            if keyboard_input.pressed(k) {
                commands.entity(e).insert(Playing);
                sink.play()
            }
            else {
                commands.entity(e).remove::<Playing>();
                sink.pause()
            }
        }
    }
}
