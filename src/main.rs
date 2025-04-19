use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::*;

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
        .add_systems(Update, button_system)
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
            font_size: 32.0,
            ..default()
        },
        // Set the justification of the Text
        TextLayout::new_with_justify(JustifyText::Right),
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
                ..default()
            },
            BorderColor(Color::BLACK),
            BorderRadius::MAX,
        ))
        .with_children(|builder| {
            builder.spawn((
                Text::new("Click on me"),
                TextFont {
                    font_size: 32.0,
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
    click: Trigger<Pointer<Click>>,
    mut mat_query: Query<
        (&mut MeshMaterial2d<ColorMaterial>, &RectangleIndexes),
        With<RectangleIndexes>,
    >,
    mut rect_indexes_q: Query<&RectangleIndexes>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("click on rect happened");

    let rect_index: &RectangleIndexes = rect_indexes_q.get_mut(click.target).unwrap();

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

fn button_system(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&mut Text, With<Label>>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                println!("Btn is pressed");
                text_query.get_single_mut().unwrap().0 = String::from("Btn is clicked");
            }
            _ => {}
        }
    }
}
