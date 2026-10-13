use bevy::prelude::{
    App, AssetId, Assets, BorderColor, BorderRadius, Camera2d, Color, ColorMaterial, Commands,
    Component, DefaultPlugins, FontSize, Justify, JustifySelf, Label, Mesh, Mesh2d, MeshMaterial2d,
    MeshPickingPlugin, Mut, Node, On, PluginGroup, PointerClick, Query, Rectangle, ResMut, Startup,
    Text, TextColor, TextFont, TextLayout, Transform, UiRect, Val, With, default,
};
use bevy::render::RenderPlugin;
use bevy::ui_widgets::{Activate, Button};

#[cfg(target_os = "windows")]
use bevy::render::settings::{Backends, WgpuSettings};

const SPRITE_SIZE: f32 = 50.;
const GREEN: Color = Color::srgb(0., 0.2, 0.);

fn main() {
    #[cfg(target_os = "windows")]
    let render_plugin = RenderPlugin {
        render_creation: WgpuSettings {
            backends: Some(Backends::DX12),
            ..default()
        }
        .into(),
        ..default()
    };
    #[cfg(not(target_os = "windows"))]
    let render_plugin = RenderPlugin::default();

    App::new()
        .add_plugins((DefaultPlugins.set(render_plugin), MeshPickingPlugin))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct RectangleIndexes(i32);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // Text with one section
    commands.spawn((
        // Accepts a `String` or any type that converts into a `String`, such as `&str`
        Text::new("Plz click on some rect"),
        TextFont {
            // This font is loaded and will be used instead of the default font.
            // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: FontSize::Px(32.0),
            ..default()
        },
        // Set the justification of the Text
        TextLayout::justify(Justify::Right),
        // Set the style of the Node itself.
        Node {
            justify_self: JustifySelf::Center,
            ..default()
        },
        Label,
    ));

    commands
        .spawn((
            Button,
            Node {
                top: Val::Px(60.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_self: JustifySelf::Center,
                border_radius: BorderRadius::MAX,
                ..default()
            },
            BorderColor::all(Color::BLACK),
        ))
        .observe(
            |_: On<Activate>, mut text_query: Query<&mut Text, With<Label>>| {
                println!("Btn is pressed");
                text_query.single_mut().unwrap().0 = String::from("Btn is clicked");
            },
        )
        .with_children(|builder| {
            builder.spawn((
                Text::new("Click on me"),
                TextFont {
                    font_size: FontSize::Px(32.0),
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });

    for i in 0..2 {
        commands
            .spawn((
                Mesh2d(meshes.add(Rectangle::new(SPRITE_SIZE, SPRITE_SIZE))),
                MeshMaterial2d(materials.add(GREEN)),
                Transform::from_xyz(i as f32 * 100.0, i as f32 * 100.0, 0.0),
                RectangleIndexes(i),
            ))
            .observe(on_rect_click);
    }
}

fn on_rect_click(
    click: On<PointerClick>,
    mut mat_query: Query<
        (&mut MeshMaterial2d<ColorMaterial>, &RectangleIndexes),
        With<RectangleIndexes>,
    >,
    mut rect_indexes_q: Query<&RectangleIndexes>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("click on rect happened");

    let rect_index: &RectangleIndexes = rect_indexes_q.get_mut(click.entity).unwrap();

    for elem in mat_query.iter_mut() {
        let some_mut: Mut<'_, MeshMaterial2d<ColorMaterial>> = elem.0;
        let asset_id: AssetId<ColorMaterial> = some_mut.0.id();
        let ind: i32 = elem.1.0;
        if ind == rect_index.0 {
            if materials.get_mut(asset_id).unwrap().color == Color::BLACK {
                materials.get_mut(asset_id).unwrap().color = GREEN;
            } else {
                materials.get_mut(asset_id).unwrap().color = Color::BLACK;
            }
        }
    }
}
