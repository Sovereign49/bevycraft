mod player;
mod block;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials : ResMut<Assets<StandardMaterial>>,
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
                }
                else if x == 0 {
                    enabled_faces.push(3);
                }
                if z == 16 {
                    enabled_faces.push(4);
                }
                else if z == 0 {
                    enabled_faces.push(5);
                }
                let cube = block::generate_cube(Some(enabled_faces));
                chunk[x][y].push(cube); 
            }
        }
    }
    let chunk_handle = meshes.add(block::generate_chunk_mesh(chunk));
    commands.spawn(PbrBundle {
                        mesh: chunk_handle,
                        material: material_handle,
                        transform: Transform::from_xyz(0.0,-16.0,0.0),
                        ..Default::default()
                    });


}

