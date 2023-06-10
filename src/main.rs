mod xac;
mod xsm;

use crate::xac::xac_parser::xacparse;
use bevy::prelude::*;
use bevy::render::mesh;
use bevy::render::mesh::PrimitiveTopology;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_flycam::PlayerPlugin)
        .add_startup_system(setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let modeldata =
        xacparse("/home/ridwan/IdeaProjects/tos-parser/bg_hi/barrack3/barrack_model.xac");

    let vertices = [
        ([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([1.0, 2.0, 1.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([2.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
    ];

    let indices = mesh::Indices::U32(vec![0, 2, 1, 0, 3, 2]);

    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    for (position, normal, uv) in vertices.iter() {
        positions.push(*position);
        normals.push(*normal);
        uvs.push(*uv);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(indices));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    let mut pos = Vec::new();
    let mut norm = Vec::new();

    let mut infl = Vec::new();
    for mesh in modeldata.mesh {
        print!("{:?} ", mesh.num_indices);

        for vert in mesh.vertices_attribute {
            for position in vert.mesh_position {
                pos.push([position.x / 10.0, position.y / 10.0, position.z / 10.0]);
            }
            for normal in vert.mesh_normal {
                norm.push([normal.x / 10.0, normal.y / 10.0, normal.z / 10.0]);
            }
            for influ in vert.mesh_influence_range_indices {
                infl.push(influ as u32);
            }
        }
    }

    let indic = mesh::Indices::U32(infl);

    let mut barrack = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(indic));
    barrack.insert_attribute(Mesh::ATTRIBUTE_POSITION, pos);
    barrack.insert_attribute(Mesh::ATTRIBUTE_NORMAL, norm);
    // barrack.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvset);

    //    let text=image::open("/home/ridwan/IdeaProjects/tos-parser/bg_hi/barrack3/barrack_model.dds").unwrap().as_rgb8();

    commands.spawn(PbrBundle {
        mesh: meshes.add(barrack),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
}
