use bevy::prelude::*;
use bevy::render::settings::*;
use bevy::render::RenderPlugin;

const SPRITE_SIZE: f32 = 50.;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(RenderPlugin {
                render_creation: WgpuSettings {
                    backends: Some(Backends::GL),
                    ..default()
                }
                .into(),
                ..default()
            }),
            MeshPickingPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(SPRITE_SIZE, SPRITE_SIZE))),
        MeshMaterial2d(materials.add(Color::srgb(0., 0.2, 0.))),
        Transform::from_xyz(100.0, 100.0, 0.0),
    ));
}
