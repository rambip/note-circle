use bevy::prelude::*;

use bevy::sprite::MaterialMesh2dBundle;
use bevy::render::render_resource::PrimitiveTopology;

use super::{NotePosition, Playing, BaseNote, Angle, N_OCTAVES};

use std::f32::consts::PI;

const OUTER_CIRCLE_RAD: f32 = 180.;
const NOTE_NAME_CIRCLE_RAD: f32 = 200.;
const INNER_CIRCLE_RAD: f32 = 100.;
const OFFSET: Vec3 = Vec3::new(-300., 0., -1.);

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct NoteNames;


fn polar2(angle: f32, radius: f32) -> Vec2 {
    radius * Vec2::from_angle(angle)
}

fn polar3(angle: f32, radius: f32, z: f32) -> Vec3 {
    polar2(angle, radius).extend(z)
}

pub fn create_note_names(
    mut commands: Commands, 
    base_note: Res<BaseNote>,
    positions: Query<(&NotePosition, &Angle)>,
    old_note_names: Query<Entity, With<NoteNames>>
    ) {

    for e in &old_note_names {
        commands.entity(e).despawn();
    }

    let text_style : TextStyle = TextStyle {
        color: Color::WHITE,
        font_size: 18.,
        font: Default::default(),
    };

    for (p, angle) in &positions {
        if p.height() == 0 {
            let name = p.name(base_note.0);

            let note_text = Text2dBundle {
                text: Text::from_section(name, text_style.clone()),
                transform: Transform::from_translation(
                    polar3(angle.0, NOTE_NAME_CIRCLE_RAD, -1.) + OFFSET
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
        transform: Transform::from_translation(OFFSET),
        ..default()
    };

    commands.spawn((bg_circle, Background));


    for (e, p, angle) in &positions {

        let range = p.0 as f32 / (12. * N_OCTAVES as f32);
        let rad = INNER_CIRCLE_RAD + (OUTER_CIRCLE_RAD - INNER_CIRCLE_RAD) * range;

        let points = vec![
                OFFSET + polar3(angle.0 - 0.060 * PI, rad, -1.),
                OFFSET + polar3(angle.0 - 0.020 * PI, rad, -1.),
                OFFSET + polar3(angle.0 + 0.020 * PI, rad, -1.),
                OFFSET + polar3(angle.0 + 0.060 * PI, rad, -1.),
        ];

        let line = MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::new(PrimitiveTopology::LineStrip)
                             .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, points)).into(),
            material: materials.add(ColorMaterial::from(Color::GRAY)),
            ..default()
        };

        commands.spawn((line, angle.clone(), Background));

        let color = p.note(0).color();
        let circle = MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(8.).into()).into(),
            material: materials.add(color.into()),
            transform: Transform::from_translation(polar3(angle.0, rad, 0.5) + OFFSET),
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

