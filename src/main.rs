mod block;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy_fps_controller::controller::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::TAU;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FpsControllerPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_systems(Startup, (setup_player, setup_world))
        .add_systems(Update, (cursor_grab_system, cast_ray))
        .run();
}

fn setup_player(mut commands: Commands) {
    let logical_entity = commands
        .spawn((
            Collider::capsule(Vec3::Y * 0.5, Vec3::Y * 1.5, 0.5),
            Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            ActiveEvents::COLLISION_EVENTS,
            Velocity::zero(),
            RigidBody::Dynamic,
            Sleeping::disabled(),
            LockedAxes::ROTATION_LOCKED,
            AdditionalMassProperties::Mass(1.0),
            GravityScale(0.0),
            Ccd { enabled: true }, // Prevent clipping when going fast
            TransformBundle::from_transform(Transform::from_xyz(0.0, 3.0, 0.0)),
            LogicalPlayer,
            FpsControllerInput {
                pitch: -TAU / 12.0,
                yaw: TAU * 5.0 / 8.0,
                ..default()
            },
            FpsController { ..default() },
        ))
        .insert(CameraConfig {
            height_offset: 0.0,
            radius_scale: 0.75,
        })
        .id();

    commands.spawn((
        Camera3dBundle {
            projection: PerspectiveProjection {
                fov: 60.0_f32.to_radians(),
                ..Default::default()
            }
            .into(),
            ..Default::default()
        },
        RenderPlayer { logical_entity },
    ));
}

fn setup_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let texture_handle = asset_server.load("textures/dirt.png");

    // this material renders the texture normally
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    let mut chunk: Vec<Vec<Vec<block::Cube>>> = vec![];

    for x in 0..17 {
        chunk.push(vec![]);
        for y in 0..17 {
            chunk[x].push(vec![]);
            for z in 0..17 {
                let mut enabled_faces: Vec<usize> = vec![];
                if y == 16 {
                    enabled_faces.push(0);
                }
                if x == 16 {
                    enabled_faces.push(2);
                } else if x == 0 {
                    enabled_faces.push(3);
                }
                if z == 16 {
                    enabled_faces.push(4);
                } else if z == 0 {
                    enabled_faces.push(5);
                }
                let cube = block::generate_cube(Some(enabled_faces));
                chunk[x][y].push(cube);
            }
        }
    }

    let chunk_mesh = block::generate_chunk_mesh(chunk);
    let chunk_handle = meshes.add(chunk_mesh.clone());
    commands
        .spawn(PbrBundle {
            mesh: chunk_handle,
            material: material_handle,
            transform: Transform::from_xyz(0.0, -16.0, 0.0),
            ..Default::default()
        })
        .insert(Collider::from_bevy_mesh(&chunk_mesh, &ComputedColliderShape::TriMesh).unwrap());
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

fn cast_ray(
    cam_query: Query<&Transform, With<Camera>>,
    btn: Res<Input<MouseButton>>,
    rapier_context: Res<RapierContext>,
) {
    let cam = cam_query.single();
    let ray_pos = cam.translation;
    let ray_dir = cam.forward();
    let max_toi = 4.0;
    let solid = true;
    let filter = QueryFilter::new();

    if let Some((entity, toi)) = rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter) {
        // The first collider hit has the entity `entity` and it hit after
        // the ray travelled a distance equal to `ray_dir * toi`.
        let hit_point = ray_pos + ray_dir * toi;
        if btn.just_pressed(MouseButton::Left) {
            println!("Entity {:?} hit at point {}", entity, hit_point.ceil());
        }
    }
}
