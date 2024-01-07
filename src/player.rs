use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_setup)
            .add_systems(Update, (cursor_grab_system, player_look, player_move));
    }
}

#[derive(Resource)]
struct PlayerConfig {
    sensitivity: f32,
    movement_speed: f32,
}

#[derive(Component)]
struct PlayerMarker;

#[derive(Component)]
struct CameraMarker;

fn player_setup(mut commands: Commands) {
    // Config
    commands.insert_resource(PlayerConfig { sensitivity: 0.1, movement_speed: 5.0 });

    // Player
    commands
        .spawn(SpatialBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        })
        .insert(PlayerMarker)
        .with_children(|parent| {
            parent
                .spawn(Camera3dBundle {
                    transform: Transform::from_xyz(0., 4., 0.0),
                    projection: PerspectiveProjection {
                        fov: 60.0_f32.to_radians(),
                        ..Default::default()
                    }.into(),
                    ..Default::default()
                })
                .insert(CameraMarker);
        });
}

fn cursor_grab_system(
    mut windows: Query<&mut Window>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    for mut window in &mut windows {
        if btn.just_pressed(MouseButton::Left) {
            // if you want to use the cursor, but not let it leave the window,
            // use `Confined` mode:
            window.cursor.grab_mode = CursorGrabMode::Confined;

            // also hide the cursor
            window.cursor.visible = false;
        }
        if key.just_pressed(KeyCode::Escape) {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    }
}

fn player_look(
    mut player_query: Query<&mut Transform, With<PlayerMarker>>,
    mut cam_query: Query<&mut Transform, (With<CameraMarker>, Without<PlayerMarker>)>,
    windows: Query<&Window>,
    config: Res<PlayerConfig>,
    mut mouse_event: EventReader<MouseMotion>,
    time: Res<Time>,
) {
    let mut player_transform = player_query.single_mut();
    let mut cam_transform = cam_query.single_mut();

    for window in &windows {
        if !window.cursor.visible {
            for event in mouse_event.read() {
                // Rotate by mouse movement
                player_transform
                    .rotate_y(-event.delta.x * config.sensitivity * time.delta_seconds());
                cam_transform.rotate_x(-event.delta.y * config.sensitivity * time.delta_seconds());
                let mut euler = cam_transform.rotation.to_euler(EulerRot::XYZ);
                euler.0 = euler.0.clamp(-85_f32.to_radians(), 85_f32.to_radians());
                cam_transform.rotation = Quat::from_euler(EulerRot::XYZ, euler.0, euler.1, euler.2);
            }
        }
    }
}

fn player_move(
    mut player_query: Query<&mut Transform, With<PlayerMarker>>,
    keys: Res<Input<KeyCode>>,
    config: Res<PlayerConfig>,
    time: Res<Time>,
) {
    let mut player_transform = player_query.single_mut();
    let spd = config.movement_speed;
    if keys.pressed(KeyCode::W) {
        player_transform.translation = player_transform.translation - player_transform.local_z() * spd * time.delta_seconds();
    }
    if keys.pressed(KeyCode::S) {
        player_transform.translation = player_transform.translation + player_transform.local_z() * spd * time.delta_seconds();
    }
    if keys.pressed(KeyCode::A) {
        player_transform.translation = player_transform.translation - player_transform.local_x() * spd * time.delta_seconds();
    }
    if keys.pressed(KeyCode::D) {
        player_transform.translation = player_transform.translation + player_transform.local_x() * spd * time.delta_seconds();
    }
    if keys.pressed(KeyCode::Q) {
        player_transform.translation = player_transform.translation - player_transform.local_y() * spd * time.delta_seconds();
    }
    if keys.pressed(KeyCode::E) {
        player_transform.translation = player_transform.translation + player_transform.local_y() * spd * time.delta_seconds();
    }
}
