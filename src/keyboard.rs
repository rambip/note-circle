use bevy::prelude::*;

use super::{Playing, NotePosition, BaseNote, UpdateNoteMapping, ChordJustChanged};

static KEYS: [(KeyCode, usize, usize); 24] = [
    (KeyCode::Key1, 0, 1),
    (KeyCode::Key2, 1, 1),
    (KeyCode::Key3, 2, 1),
    (KeyCode::Key4, 3, 1),
    (KeyCode::Key5, 4, 1),
    (KeyCode::Key6, 5, 1),
    (KeyCode::Key7, 6, 1),
    (KeyCode::Key8, 7, 1),
    (KeyCode::Key9, 8, 1),
    (KeyCode::Key0, 9, 1),
    (KeyCode::Minus, 10, 1),
    (KeyCode::Equals, 11, 1),
    (KeyCode::Q, 00, 0),
    (KeyCode::W, 01, 0),
    (KeyCode::E, 02, 0),
    (KeyCode::R, 03, 0),
    (KeyCode::T, 04, 0),
    (KeyCode::Y, 05, 0),
    (KeyCode::U, 06, 0),
    (KeyCode::I, 07, 0),
    (KeyCode::O, 08, 0),
    (KeyCode::P, 09, 0),
    (KeyCode::BracketLeft, 10, 0),
    (KeyCode::BracketRight, 11, 0),
];

pub fn keyboard_input_system(
    mut mapping_changed: EventWriter<UpdateNoteMapping>,
    mut base_note: ResMut<BaseNote>,
    keyboard_input: Res<Input<KeyCode>>, 
    mut query: Query<(&NotePosition, &AudioSink, &mut Playing)>,
    mut chord_changed: ResMut<ChordJustChanged>,
) {

    //chord_changed.0 = false;

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

    for (note, sink, mut playing) in &mut query {
        for (k, oclock, height) in KEYS {
            if note.oclock() == oclock && note.height() == height {
                if keyboard_input.pressed(k) && !playing.0 {
                    chord_changed.0 = true;
                    sink.play()
                }
                if !keyboard_input.pressed(k) && playing.0 {
                    chord_changed.0 = true;
                }
                if keyboard_input.pressed(k) {
                    playing.0 = true;
                }
                if !keyboard_input.pressed(k) {
                    playing.0 = false;
                    sink.pause();
                }
            }
        }
    }
}
