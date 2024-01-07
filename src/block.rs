use bevy::render::mesh::Indices;
use bevy::prelude::*;

struct Side {
    positions: Vec<[f32;3]>,
    normals: Vec<[f32;3]>,
    uvs: Vec<[f32;2]>,
    indices: Vec<u32>,
    enabled: bool,
}

#[derive(Clone)]
pub struct Cube {
    positions: Vec<[f32;3]>,
    normals: Vec<[f32;3]>,
    uvs: Vec<[f32;2]>,
    indices: Vec<u32>,
}

// function to generate a cube mesh with a disabled face
pub fn generate_cube(enabled_faces: Option<Vec<usize>>) -> Cube {
    let top = Side {
        positions: vec![
            // top (facing towards +y)
            [-0.5, 0.5, -0.5], // vertex with index 0
            [0.5, 0.5, -0.5], // vertex with index 1
            [0.5, 0.5, 0.5], // etc. until 23
            [-0.5, 0.5, 0.5],
        ],
        normals: vec![
            // Normals for the top side (towards +y)
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
        ],
        uvs: vec![
            [0.0, 0.0],
            [0.0, 1.0],
            [1.0, 1.0],
            [1.0, 0.0],
        ],
        indices: vec![
            0,3,1, 1,3,2, // triangles making up the top (+y) facing side.
        ],
        enabled: false,
    };

    let bottom = Side {
        positions: vec![
            // bottom   (-y)
            [-0.5, -0.5, -0.5],
            [0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [-0.5, -0.5, 0.5],
        ],
        normals: vec![
            // Normals for the bottom side (towards -y)
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
        ],
        uvs: vec![
            [0.0, 0.0],
            [0.0, 1.0],
            [1.0, 1.0],
            [1.0, 0.0],
        ],
        indices: vec![
            0,1,3,1,2,3 // bottom(-y)
        ],
        enabled: false,
    };

    let right = Side {
        positions: vec![
            // right    (+x)
            [0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [0.5, 0.5, 0.5], // This vertex is at the same position as vertex with index 2, but they'll have different UV and norma,
                             // 
            [0.5, 0.5, -0.5],
        ],
        normals: vec![
            // Normals for the right side (towards +x)
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0], 
        ],
        uvs: vec![
            [0.0, 0.0],
            [0.0, 1.0],
            [1.0, 1.0],
            [1.0, 0.0],
        ],
        indices: vec![
            0,3,1,1,3,2 // right (+x)
        ],
        enabled: false,
    };
    let left = Side { 
        positions: vec![
            // left     (-x)
            [-0.5, -0.5, -0.5],
            [-0.5, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [-0.5, 0.5, -0.5],
        ],
        normals: vec![
            // Normals for the left side (towards -x)
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
        ],
        uvs: vec![
            [0.0, 0.0],
            [0.0, 1.0],
            [1.0, 1.0],
            [1.0, 0.0],
        ],
        indices: vec![
            0,1,3,1,2,3 // left (-x)
        ],
        enabled: false,
    };
    let back = Side {
        positions: vec![
            // back     (+z)
            [-0.5, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [0.5, 0.5, 0.5],
            [0.5, -0.5, 0.5],
        ],
        normals: vec![
            // Normals for the back side (towards +z)
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
        ],
        uvs: vec![
            [0.0, 0.0],
            [0.0, 1.0],
            [1.0, 1.0],
            [1.0, 0.0],
        ],
        indices: vec![
            0,3,1,1,3,2 // back (+z)
        ],
        enabled: false,
    };
    let front = Side {
        positions: vec![
            // forward  (-z)
            [-0.5, -0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [0.5, 0.5, -0.5],
            [0.5, -0.5, -0.5],
        ],
        normals: vec![
            // Normals for the forward side (towards -z)
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
        ],
        uvs: vec![
            [0.0, 0.0],
            [0.0, 1.0],
            [1.0, 1.0],
            [1.0, 0.0],
        ],
        indices: vec![
            0,1,3,1,2,3, // forward (-z)
        ],
        enabled: false,
    };
   

    let mut cube_faces = vec![
        top, // +y
        bottom, // -y
        right, // +x
        left, // -x
        back, // +z
        front, // -z
    ];

    if let Some(faces) = enabled_faces {
        for face in faces {
            cube_faces[face].enabled = true;
        }
    }
    let mut positions = vec![];
    let mut normals = vec![];
    let mut uvs = vec![];
    let mut indices = vec![];

    for mut face in cube_faces {
        if face.enabled {
            let size = positions.len() as u32;
            positions.append(&mut face.positions);
            normals.append(&mut face.normals);
            uvs.append(&mut face.uvs);
            face.indices = face.indices.iter().map(|x| x+size).collect::<Vec<u32>>();
            indices.append(&mut face.indices);
        } 
    }
    Cube {
        positions: positions.clone(),
        normals: normals.clone(),
        uvs: uvs.clone(),
        indices: indices.clone(),
    }
}

pub fn generate_chunk_mesh(chunk_data:Vec<Vec<Vec<Cube>>>) -> Mesh {
    let mut positions: Vec<[f32;3]> = vec![];
    let mut normals: Vec<[f32;3]>  = vec![];
    let mut uvs: Vec<[f32;2]>  = vec![];
    let mut indices: Vec<u32>  = vec![];
    for x in 0..chunk_data.len() {
        for y in 0..chunk_data[x].len() {
            for z in 0..chunk_data[x][y].len() {
                let mut cube = chunk_data[x][y][z].clone();
                for i in 0..cube.positions.len() {
                    cube.positions[i][0] += x as f32;
                    cube.positions[i][1] += y as f32;
                    cube.positions[i][2] += z as f32;
                }
                let size = positions.len() as u32;
                positions.append(&mut cube.positions);
                normals.append(&mut cube.normals);
                uvs.append(&mut cube.uvs);
                cube.indices = cube.indices.iter().map(|x| x+size).collect::<Vec<u32>>();
                indices.append(&mut cube.indices);
            }
        }
    }
    Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_indices(Some(Indices::U32(indices)))
}
