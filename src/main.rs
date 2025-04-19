use bevy::prelude::*;
use bevy::render::settings::*;
use bevy::render::RenderPlugin;

const SPRITE_SIZE: f32 = 50.;
const GREEN: Color = Color::srgb(0., 0.2, 0.);

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
        MeshMaterial2d(materials.add(GREEN)),
        Transform::from_xyz(100.0, 100.0, 0.0),
    )).observe(on_rect_click);
}

fn on_rect_click(
    _click: Trigger<Pointer<Click>>,
    mut mat_query: Query<
        &mut MeshMaterial2d<ColorMaterial>,
    >,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("click on rect happened");

    let elem: Mut<'_, MeshMaterial2d<ColorMaterial>> = mat_query.single_mut();
    let asset_id: AssetId<ColorMaterial> = elem.0.id();
    if materials.get_mut(asset_id).unwrap().color == Color::BLACK {
        materials.get_mut(asset_id).unwrap().color = GREEN;
    }
    else {
        materials.get_mut(asset_id).unwrap().color = Color::BLACK;
    }
}