use bevy::asset::AssetMut;
use bevy::prelude::{
    Aabb2d, App, AssetId, Assets, BorderColor, BorderRadius, ButtonInput, Camera2d, Color,
    ColorMaterial, Commands, Component, DefaultPlugins, Entity, FontSize, IntersectsVolume,
    Justify, JustifySelf, KeyCode, Label, Mesh, Mesh2d, MeshMaterial2d, MeshPickingPlugin, Mut,
    Node, On, PointerClick, Query, Rectangle, Res, ResMut, Resource, Startup, Text, TextColor,
    TextFont, TextLayout, Time, Timer, TimerMode, Transform, Triangle2d, UiRect, Update, Val, Vec2,
    Vec3, Window, With, default,
};
use bevy::ui_widgets::Button;
use bevy::window::PrimaryWindow;

const SPRITE_SIZE: f32 = 50.;
const GREEN: Color = Color::srgb(0., 0.2, 0.);
const AMOUNT_OF_ENEMIES_RECTS: u8 = 3;

#[derive(Component)]
struct RectLabel;

#[derive(Resource)]
struct RectsShouldMove(bool);

const VELOCITY: f32 = 120.0;

const PLAYER_SPEED: f32 = 200.0;
const PLAYER_RADIUS: f32 = 25.0;

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerBullet;

#[derive(Component)]
struct EnemyBullet;

#[derive(Component)]
struct EnemiesFireTimer(Timer);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .insert_resource(RectsShouldMove(false))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                rects_movement_system,
                player_movement,
                player_fire_bullet_system,
                check_player_bullets_velocity_update_system,
                check_player_bullets_and_rects_collisions_system,
                check_player_and_rects_collisions_system,
                tick_enemies_fire_bullets_system,
                check_enemies_bullets_velocity_update_system,
                check_enemies_bullets_and_player_collisions_system,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    // Text with one section
    commands.spawn((
        // Accepts a `String` or any type that converts into a `String`, such as `&str`
        Text::new("Plz click on some rect to change it color"),
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
                Text::new("Start"),
                TextFont {
                    font_size: FontSize::Px(32.0),
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        })
        .observe(on_start_btn_click);
    spawn_player_and_rect_enemies(commands, meshes, materials);
}

fn spawn_player_and_rect_enemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for i in 0..AMOUNT_OF_ENEMIES_RECTS {
        commands
            .spawn((
                Mesh2d(meshes.add(Rectangle::new(SPRITE_SIZE, SPRITE_SIZE))),
                MeshMaterial2d(materials.add(GREEN)),
                Transform::from_xyz(i as f32 * 100.0, i as f32 * 100.0, 0.0),
                RectLabel,
                Velocity {
                    x: VELOCITY,
                    y: VELOCITY,
                },
                EnemiesFireTimer(Timer::from_seconds(2.0, TimerMode::Repeating)),
            ))
            .observe(on_rect_click);
    }

    let half_base = PLAYER_RADIUS * 3f32.sqrt() / 2.0;
    commands.spawn((
        Mesh2d(meshes.add(Triangle2d::new(
            Vec2::new(0.0, PLAYER_RADIUS),
            Vec2::new(-half_base, -PLAYER_RADIUS / 2.0),
            Vec2::new(half_base, -PLAYER_RADIUS / 2.0),
        ))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
        Transform::from_xyz(0.0, -100.0, 0.0),
        Player,
    ));
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

fn on_start_btn_click(
    _click: On<PointerClick>,
    mut commands: Commands,
    mut text_query: Query<&mut Text, With<Label>>,
    mut rect_should_move: ResMut<RectsShouldMove>,
    mut rect_enemy_entities: Query<Entity, With<RectLabel>>,
    player_entity_q: Query<Entity, With<Player>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    for rect_enemy_ent in rect_enemy_entities.iter_mut() {
        commands.entity(rect_enemy_ent).despawn();
    }
    if let Ok(player_entity) = player_entity_q.single() {
        commands.entity(player_entity).despawn();
    }

    spawn_player_and_rect_enemies(commands, meshes, materials);

    text_query.single_mut().unwrap().0 = String::from("Btn is clicked");
    rect_should_move.0 = true;
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

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(mut transf) = player_query.single_mut() else {
        return;
    };
    let Ok(window) = window_query.single() else {
        return;
    };

    let mut dir = Vec2::ZERO;
    if keys.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
        dir.y += 1.0;
    }
    if keys.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
        dir.y -= 1.0;
    }
    if keys.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        dir.x -= 1.0;
    }
    if keys.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
        dir.x += 1.0;
    }

    if dir != Vec2::ZERO {
        dir = dir.normalize();
    }

    transf.translation += (dir * PLAYER_SPEED * time.delta_secs()).extend(0.0);

    let max_x = window.width() / 2.0 - PLAYER_RADIUS;
    let max_y = window.height() / 2.0 - PLAYER_RADIUS;
    transf.translation.x = transf.translation.x.clamp(-max_x, max_x);
    transf.translation.y = transf.translation.y.clamp(-max_y, max_y);
}

fn player_fire_bullet_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keys: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(transf) = player_query.single() else {
        return;
    };
    let Ok(_window) = window_query.single() else {
        return;
    };
    if keys.just_pressed(KeyCode::Space) {
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(2.0, 2.0))),
            MeshMaterial2d(materials.add(Color::WHITE)),
            Transform::from_xyz(transf.translation.x, transf.translation.y, 0.0),
            PlayerBullet,
            Velocity {
                x: 0.0,
                y: VELOCITY,
            },
        ));
    }
}

fn check_player_bullets_velocity_update_system(
    mut commands: Commands,
    mut bullets_tr_query: Query<(Entity, &mut Transform, &Velocity), With<PlayerBullet>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let Ok(window) = window_query.single() else {
        return;
    };

    let margin = SPRITE_SIZE / 2.0;
    let max_x = window.width() / 2.0 + margin;
    let max_y = window.height() / 2.0 + margin;

    for (entity, mut transf, velocity) in bullets_tr_query.iter_mut() {
        transf.translation += Vec3::new(0.0, velocity.y, 0.0) * time.delta_secs();

        if transf.translation.x.abs() > max_x || transf.translation.y.abs() > max_y {
            commands.entity(entity).despawn();
        }
    }
}

fn check_player_bullets_and_rects_collisions_system(
    mut commands: Commands,
    mut bullets_tr_query: Query<(Entity, &Transform), With<PlayerBullet>>,
    mut rect_tr_query: Query<(Entity, &Transform), With<RectLabel>>,
) {
    for (bullet_entity, bullet_transf) in bullets_tr_query.iter_mut() {
        for (rect_entity, rect_transf) in rect_tr_query.iter_mut() {
            let bullet = Aabb2d::new(bullet_transf.translation.truncate(), Vec2::splat(1.0));
            let rect = Aabb2d::new(
                rect_transf.translation.truncate(),
                Vec2::splat(SPRITE_SIZE / 2.0),
            );
            if bullet.intersects(&rect) {
                commands.entity(bullet_entity).despawn();
                commands.entity(rect_entity).despawn();
            }
        }
    }
}

fn check_player_and_rects_collisions_system(
    mut commands: Commands,
    palyer_query: Query<(Entity, &Transform), With<Player>>,
    mut rect_tr_query: Query<(Entity, &Transform), With<RectLabel>>,
) {
    if let Ok(player_tuple) = palyer_query.single() {
        let player_entity = player_tuple.0;
        let player_transf = player_tuple.1;
        for (_, rect_transf) in rect_tr_query.iter_mut() {
            let player = Aabb2d::new(player_transf.translation.truncate(), Vec2::splat(1.0));
            let rect = Aabb2d::new(
                rect_transf.translation.truncate(),
                Vec2::splat(SPRITE_SIZE / 2.0),
            );
            if player.intersects(&rect) {
                commands.entity(player_entity).despawn();
            }
        }
    }
}

fn tick_enemies_fire_bullets_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut rect_fire_query: Query<(Entity, &mut EnemiesFireTimer, &Transform), With<RectLabel>>,
) {
    for (_, mut fire_timer, transf) in &mut rect_fire_query {
        if fire_timer.0.tick(time.delta()).just_finished() {
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::new(4.0, 4.0))),
                MeshMaterial2d(materials.add(Color::BLACK)),
                Transform::from_xyz(transf.translation.x, transf.translation.y, 0.0),
                EnemyBullet,
                Velocity {
                    x: 0.0,
                    y: -VELOCITY,
                },
            ));
        }
    }
}

fn check_enemies_bullets_velocity_update_system(
    mut commands: Commands,
    mut bullets_tr_query: Query<(Entity, &mut Transform, &Velocity), With<EnemyBullet>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let Ok(window) = window_query.single() else {
        return;
    };

    let margin = SPRITE_SIZE / 2.0;
    let max_x = window.width() / 2.0 + margin;
    let max_y = window.height() / 2.0 + margin;

    for (entity, mut transf, velocity) in bullets_tr_query.iter_mut() {
        transf.translation += Vec3::new(0.0, velocity.y, 0.0) * time.delta_secs();

        if transf.translation.x.abs() > max_x || transf.translation.y.abs() > max_y {
            commands.entity(entity).despawn();
        }
    }
}

fn check_enemies_bullets_and_player_collisions_system(
    mut commands: Commands,
    mut bullets_tr_query: Query<(Entity, &Transform), With<EnemyBullet>>,
    mut rect_tr_query: Query<(Entity, &Transform), With<Player>>,
) {
    for (bullet_entity, bullet_transf) in bullets_tr_query.iter_mut() {
        for (rect_entity, rect_transf) in rect_tr_query.iter_mut() {
            let bullet = Aabb2d::new(bullet_transf.translation.truncate(), Vec2::splat(1.0));
            let rect = Aabb2d::new(
                rect_transf.translation.truncate(),
                Vec2::splat(SPRITE_SIZE / 2.0),
            );
            if bullet.intersects(&rect) {
                commands.entity(bullet_entity).despawn();
                commands.entity(rect_entity).despawn();
            }
        }
    }
}
