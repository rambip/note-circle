use bevy::prelude::*;

use bevy::sprite::MaterialMesh2dBundle;
use bevy::render::render_resource::PrimitiveTopology;

use super::{NotePosition, Playing, BaseNote, Angle, N_OCTAVES};

use std::f32::consts::PI;

const OUTER_CIRCLE_RAD: f32 = 180.;
const NOTE_NAME_CIRCLE_RAD: f32 = 200.;
const INNER_CIRCLE_RAD: f32 = 100.;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct NoteNames;

pub fn create_note_names(
    mut commands: Commands, 
    base_note: Res<BaseNote>,
    positions: Query<(&NotePosition, &Angle)>,
    old_note_names: Query<Entity, With<NoteNames>>
    ) {

    for e in &old_note_names {
        commands.entity(e).despawn();
    }

    let text_style : TextStyle = Default::default();

    for (p, angle) in &positions {
        if p.height() == 0 {
            let name = p.name(base_note.0);

            let note_text = Text2dBundle {
                text: Text::from_section(name, text_style.clone()),
                transform: Transform::from_translation(
                    NOTE_NAME_CIRCLE_RAD * Vec2::from_angle(angle.0).extend(1.)
                ),
                ..default()
            };

            commands.spawn((note_text, NoteNames, Background));
        }
    }
}

pub fn create_circle(
                    mut commands: Commands, 
                    mut meshes: ResMut<Assets<Mesh>>, 
                    mut materials: ResMut<Assets<ColorMaterial>>,
                    positions: Query<(Entity, &NotePosition, &Angle)>,
    ){

    let bg_circle = MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(OUTER_CIRCLE_RAD).into()).into(),
        material: materials.add(ColorMaterial::from(Color::WHITE)),
        transform: Transform::from_translation(Vec3::new(0., 0., -2.)),
        ..default()
    };

    commands.spawn((bg_circle, Background));


    for (e, p, angle) in &positions {

        let range = p.0 as f32 / (12. * N_OCTAVES as f32);
        let rad = INNER_CIRCLE_RAD + (OUTER_CIRCLE_RAD - INNER_CIRCLE_RAD) * range;

        let points = vec![
                rad * Vec2::from_angle(angle.0 - 0.060 * PI).extend(-1.),
                rad * Vec2::from_angle(angle.0 - 0.020 * PI).extend(-1.),
                rad * Vec2::from_angle(angle.0 + 0.020 * PI).extend(-1.),
                rad * Vec2::from_angle(angle.0 + 0.060 * PI).extend(-1.),
        ];

        let line = MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::new(PrimitiveTopology::LineStrip)
                             .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, points)).into(),
            material: materials.add(ColorMaterial::from(Color::GRAY)),
            ..default()
        };

        commands.spawn((line, angle.clone(), Background));

        let circle = MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(8.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(rad * Vec2::from_angle(angle.0).extend(1.)),
            ..default()
        };

        commands.entity(e).insert(circle);
    }

}

pub fn draw_notes(
    mut new_notes: Query<&mut Visibility, (With<NotePosition>, With<Playing>, Without<Background>)>,
    mut dead_notes: Query<&mut Visibility, (With<NotePosition>, Without<Playing>, Without<Background>)>
    ) {
    for mut visible in new_notes.iter_mut() {
        *visible = Visibility::Visible;
    }
    for mut visible in dead_notes.iter_mut() {
        *visible = Visibility::Hidden;
    }
}
