use bevy::asset::AssetMut;
use bevy::prelude::{
    App, AssetId, Assets, BorderColor, BorderRadius, Camera2d, Color, ColorMaterial, Commands,
    Component, DefaultPlugins, FontSize, Justify, JustifySelf, Label, Mesh, Mesh2d, MeshMaterial2d,
    MeshPickingPlugin, Mut, Node, On, PluginGroup, PointerClick, Query, Rectangle, Res, ResMut,
    Resource, Startup, Text, TextColor, TextFont, TextLayout, Time, Transform, UiRect, Update, Val,
    Vec3, Window, With, default,
};
use bevy::render::RenderPlugin;
use bevy::ui_widgets::Button;
use bevy::window::PrimaryWindow;

#[cfg(target_os = "windows")]
use bevy::render::settings::{Backends, WgpuSettings};

const SPRITE_SIZE: f32 = 50.;
const GREEN: Color = Color::srgb(0., 0.2, 0.);

#[derive(Component)]
struct RectLabel;

#[derive(Resource)]
struct RectsShouldMove(bool);

const VELOCITY: f32 = 120.0;

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

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
        .insert_resource(RectsShouldMove(false))
        .add_systems(Startup, setup)
        .add_systems(Update, rects_movement_system)
        .run();
}

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
        .with_children(|builder| {
            builder.spawn((
                Text::new("Click on me"),
                TextFont {
                    font_size: FontSize::Px(32.0),
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        })
        .observe(on_btn_click);

    for i in 0..2 {
        commands
            .spawn((
                Mesh2d(meshes.add(Rectangle::new(SPRITE_SIZE, SPRITE_SIZE))),
                MeshMaterial2d(materials.add(GREEN)),
                Transform::from_xyz(i as f32 * 100.0, i as f32 * 100.0, 0.0),
                RectLabel,
                Velocity { x: 0.0, y: 0.0 },
            ))
            .observe(on_rect_click);
    }
}

fn on_rect_click(
    click: On<PointerClick>,
    mut mat_query: Query<&mut MeshMaterial2d<ColorMaterial>, With<RectLabel>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("click on rect happened");

    let rect_mat: Mut<'_, MeshMaterial2d<_>> = mat_query.get_mut(click.entity).unwrap();
    let asset_id: AssetId<ColorMaterial> = rect_mat.0.id();
    let mut asset_mat: AssetMut<'_, ColorMaterial> = materials.get_mut(asset_id).unwrap();
    if asset_mat.color == Color::BLACK {
        asset_mat.color = GREEN;
    } else {
        asset_mat.color = Color::BLACK;
    }
}

fn on_btn_click(
    _click: On<PointerClick>,
    mut text_query: Query<&mut Text, With<Label>>,
    mut rect_should_move: ResMut<RectsShouldMove>,
    mut rect_vel_query: Query<&mut Velocity, With<RectLabel>>,
) {
    text_query.single_mut().unwrap().0 = String::from("Btn is clicked");
    rect_should_move.0 = true;
    for mut velocity in rect_vel_query.iter_mut() {
        velocity.x = VELOCITY;
        velocity.y = VELOCITY;
    }
}

fn rects_movement_system(
    time: Res<Time>,
    mut rect_tr_query: Query<(&mut Transform, &mut Velocity), With<RectLabel>>,
    rect_should_move: Res<RectsShouldMove>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if rect_should_move.0
        && let Ok(window) = window_query.single()
    {
        let width = window.width();
        let height = window.height();

        let half = SPRITE_SIZE / 2.0;
        let min_x = -width / 2.0 + half;
        let max_x = width / 2.0 - half;
        let min_y = -height / 2.0 + half;
        let max_y = height / 2.0 - half;

        for (mut transf, mut velocity) in rect_tr_query.iter_mut() {
            transf.translation += Vec3::new(velocity.x, velocity.y, 0.0) * time.delta_secs();

            if transf.translation.x <= min_x {
                transf.translation.x = min_x;
                velocity.x = velocity.x.abs();
            } else if transf.translation.x >= max_x {
                transf.translation.x = max_x;
                velocity.x = -velocity.x.abs();
            }

            if transf.translation.y <= min_y {
                transf.translation.y = min_y;
                velocity.y = velocity.y.abs();
            } else if transf.translation.y >= max_y {
                transf.translation.y = max_y;
                velocity.y = -velocity.y.abs();
            }
        }
    }
}
