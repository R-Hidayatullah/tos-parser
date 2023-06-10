#![allow(dead_code, unused_variables)]

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::xac::xac_enums::XacChunkType::{
    XacMaterialDefinitionId, XacMaterialTotalId, XacMeshId, XacMetadataId, XacMorphTargetId,
    XacNodeHierarchyId, XacShaderMaterialId, XacSkinningId,
};
use crate::xac::xac_enums::XacVerticesAttributeType::{
    XacColor128Id, XacColor32Id, XacInfluenceRangeId, XacNormalId, XacPositionId, XacTangentId,
    XacUVCoordId,
};
use crate::xac::xac_structs::{
    Xac, XacActorMaterial, XacActorMaterialLayer, XacActorMesh, XacActorMorphTarget, XacActorNode,
    XacBoolProperties, XacChunk, XacColor8, XacFloatProperties, XacHeader, XacIntProperties,
    XacMaterialTotal, XacMatrix44, XacMetadata, XacNodeHierarchy, XacQuaternion, XacShaderMaterial,
    XacSkinning, XacStringProperties, XacSubMesh, XacVec2d, XacVec3d, XacVec4d,
    XacVerticesAttribute,
};

pub fn xacparse(path: &str) -> Xac {
    println!("Path : {}", path);
    let mut xac_file = File::open(path).expect("Cannot open xac file!");
    let mut xac_new = Xac {
        header: XacHeader {
            magic: "".to_string(),
            major_version: 0,
            minor_version: 0,
            big_endian: 0,
            multiply_order: 0,
        },
        mesh: vec![],
        skinning: XacSkinning {
            node_id: 0,
            num_local_bones: 0,
            num_influences: 0,
            for_collision_mesh: 0,
        },
        material_definition: XacActorMaterial {
            ambient_color: XacVec4d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
            diffuse_color: XacVec4d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
            specular_color: XacVec4d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
            emissive_color: XacVec4d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
            shine: 0.0,
            shine_strength: 0.0,
            opacity: 0.0,
            ior: 0.0,
            double_sided: 0,
            wireframe: 0,
            num_layers: 0,
            name: "".to_string(),
            layers: vec![],
        },
        shader_material: vec![],
        metadata: XacMetadata {
            reposition_mask: 0,
            repositioning_node: 0,
            exporter_major_version: 0,
            exporter_minor_version: 0,
            retarget_root_offset: 0.0,
            source_app: "".to_string(),
            original_filename: "".to_string(),
            export_date: "".to_string(),
            actor_name: "".to_string(),
        },
        node_hierarchy: XacNodeHierarchy {
            num_nodes: 0,
            num_root_nodes: 0,
            node_data: vec![],
        },
        morph_target: XacActorMorphTarget {
            num_morph_targets: 0,
            lod_morph_target_id: 0,
            morph_targets: vec![],
        },
        material_total: XacMaterialTotal {
            num_total_materials: 0,
            num_standard_materials: 0,
            num_fx_materials: 0,
        },
        indices: vec![],
        indices_num: 0,
    };

    read_header(&mut xac_file, &mut xac_new);
    read_chunk(&mut xac_file, &mut xac_new);
    xac_new
}

pub fn xac_read_string(file: &mut File) -> String {
    let mut text = String::new();
    let length = file.read_i32::<LittleEndian>().unwrap();
    for _ in 0..length {
        let character = file.read_u8().unwrap();
        text.push(character as char);
    }
    text
}
fn xac_read_color8(file: &mut File) -> XacColor8 {
    let color8 = XacColor8 {
        x: file.read_u8().unwrap(),
        y: file.read_u8().unwrap(),
        z: file.read_u8().unwrap(),
    };
    color8
}

fn xac_read_vec2d(file: &mut File) -> XacVec2d {
    let vec2d = XacVec2d {
        x: file.read_f32::<LittleEndian>().unwrap(),
        y: file.read_f32::<LittleEndian>().unwrap(),
    };
    vec2d
}
fn xac_read_vec3d(file: &mut File) -> XacVec3d {
    let vec3d = XacVec3d {
        x: file.read_f32::<LittleEndian>().unwrap(),
        y: file.read_f32::<LittleEndian>().unwrap(),
        z: file.read_f32::<LittleEndian>().unwrap(),
    };
    vec3d
}
fn xac_read_vec4d(file: &mut File) -> XacVec4d {
    let vec4d = XacVec4d {
        x: file.read_f32::<LittleEndian>().unwrap(),
        y: file.read_f32::<LittleEndian>().unwrap(),
        z: file.read_f32::<LittleEndian>().unwrap(),
        w: file.read_f32::<LittleEndian>().unwrap(),
    };
    vec4d
}

fn xac_read_quaternion(file: &mut File) -> XacQuaternion {
    let quaternion = XacQuaternion {
        x: file.read_i32::<LittleEndian>().unwrap(),
        y: file.read_i32::<LittleEndian>().unwrap(),
        z: file.read_i32::<LittleEndian>().unwrap(),
        w: file.read_i32::<LittleEndian>().unwrap(),
    };
    quaternion
}

fn xac_read_matrix44(file: &mut File) -> XacMatrix44 {
    let matrix44 = XacMatrix44 {
        col1: xac_read_vec4d(file),
        col2: xac_read_vec4d(file),
        col3: xac_read_vec4d(file),
        pos: xac_read_vec4d(file),
    };
    matrix44
}

fn read_header<'a>(file: &'a mut File, xac: &'a mut Xac) -> &'a mut Xac {
    let mut magic = [0; 4];
    file.read_exact(&mut magic).unwrap();
    xac.header.magic = std::str::from_utf8(&magic).unwrap().to_string();
    if xac.header.magic != "XAC " {
        panic!("Not an XAC file: invalid header magic");
    }
    xac.header.major_version = file.read_u8().unwrap();
    xac.header.minor_version = file.read_u8().unwrap();
    if xac.header.major_version != 1 || xac.header.minor_version != 0 {
        panic!(
            "Unsupported .xac version: expected v1.0, file is {}.{}",
            xac.header.major_version, xac.header.minor_version
        );
    }
    xac.header.big_endian = file.read_u8().unwrap();
    if xac.header.big_endian != 0 {
        panic!("XAC file is encoded in big endian which is not supported by this importer");
    }
    xac.header.multiply_order = file.read_u8().unwrap();
    xac
}

fn read_chunk<'a>(file: &'a mut File, xac: &'a mut Xac) -> &'a mut Xac {
    while file.stream_position().unwrap() < file.metadata().unwrap().len() {
        let chunk = XacChunk {
            chunk_type: file.read_i32::<LittleEndian>().unwrap(),
            length: file.read_i32::<LittleEndian>().unwrap(),
            version: file.read_i32::<LittleEndian>().unwrap(),
        };
        let position = file.stream_position().unwrap();

        if chunk.chunk_type == XacMeshId as i32 {
            read_mesh(file, xac);
        }

        if chunk.chunk_type == XacSkinningId as i32 {
            read_skinning(file, xac);
        }
        if chunk.chunk_type == XacMaterialDefinitionId as i32 {
            read_material_definition(file, xac);
        }
        if chunk.chunk_type == XacShaderMaterialId as i32 {
            read_shader_material(file, xac);
        }

        if chunk.chunk_type == XacMetadataId as i32 {
            read_metadata(file, xac);
        }
        if chunk.chunk_type == XacNodeHierarchyId as i32 {
            read_node_hierarchy(file, xac);
        }
        if chunk.chunk_type == XacMorphTargetId as i32 {
            read_morph_target(file, xac);
        }
        if chunk.chunk_type == XacMaterialTotalId as i32 {
            read_material_total(file, xac);
        }

        file.seek(SeekFrom::Start(position + chunk.length as u64))
            .unwrap();
    }
    xac
}

fn read_mesh<'a>(file: &'a mut File, xac: &'a mut Xac) -> &'a mut Xac {
    let mut mesh = XacActorMesh {
        node_id: 0,
        num_influence_ranges: 0,
        num_vertices: 0,
        num_indices: 0,
        num_submeshes: 0,
        num_attribute_layers: 0,
        collision_mesh: 0,
        vertices_attribute: vec![],
        sub_mesh: vec![],
    };
    mesh.node_id = file.read_i32::<LittleEndian>().unwrap();
    mesh.num_influence_ranges = file.read_i32::<LittleEndian>().unwrap();
    mesh.num_vertices = file.read_i32::<LittleEndian>().unwrap();
    mesh.num_indices = file.read_i32::<LittleEndian>().unwrap();
    mesh.num_submeshes = file.read_i32::<LittleEndian>().unwrap();
    mesh.num_attribute_layers = file.read_i32::<LittleEndian>().unwrap();
    mesh.collision_mesh = file.read_u8().unwrap();
    file.read_u8().unwrap(); //Padding
    file.read_u8().unwrap(); //Padding
    file.read_u8().unwrap(); //Padding

    let list: Vec<XacVec3d> = Vec::new(); //XacPositionId
    let list2: Vec<XacVec3d> = Vec::new();
    let list3: Vec<XacVec4d> = Vec::new();
    let list4: Vec<XacVec4d> = Vec::new();
    let mut list5: Vec<Vec<XacVec2d>> = Vec::new();
    let mut list6: Vec<i32> = Vec::new();
    let mut list7: Vec<Vec<XacVec3d>> = Vec::new();
    let mut list8: Vec<Vec<XacColor8>> = Vec::new();

    for _ in 0..mesh.num_attribute_layers {
        let mut vertices_attribute = XacVerticesAttribute {
            type_id: 0,
            attribute_size: 0,
            keep_originals: 0,
            scale_factor: 0,
            mesh_position: vec![],
            mesh_normal: vec![],
            mesh_unknown_vec4d: vec![],
            mesh_unknown_vec2d: vec![],
            mesh_color8: vec![],
            mesh_influence_range_indices: vec![],
            mesh_color: vec![],
            mesh_data: vec![],
        };
        vertices_attribute.type_id = file.read_i32::<LittleEndian>().unwrap();
        vertices_attribute.attribute_size = file.read_i32::<LittleEndian>().unwrap();
        vertices_attribute.keep_originals = file.read_u8().unwrap();
        vertices_attribute.scale_factor = file.read_u8().unwrap();
        file.read_u8().unwrap(); //Padding
        file.read_u8().unwrap(); //Padding
        if vertices_attribute.type_id == XacPositionId as i32 {
            for _ in 0..mesh.num_vertices {
                vertices_attribute.mesh_position.push(xac_read_vec3d(file))
            }
        }
        if vertices_attribute.type_id == XacNormalId as i32 {
            for _ in 0..mesh.num_vertices {
                vertices_attribute.mesh_normal.push(xac_read_vec3d(file))
            }
        }
        if vertices_attribute.type_id == XacTangentId as i32 {
            let mut list9: Vec<XacVec4d> = Vec::new();
            if list3.is_empty() {
                let temp = &list3;
                list9 = temp.to_vec();
            } else {
                if !list4.is_empty() {
                    for _ in 0..(&vertices_attribute.attribute_size * &mesh.num_vertices) {
                        file.read_u8().unwrap();
                    }
                    let temp = &list4;
                    list9 = temp.to_vec();
                }
            }
            for _ in 0..mesh.num_vertices {
                list9.push(xac_read_vec4d(file));
            }
        }
        if vertices_attribute.type_id == XacUVCoordId as i32 {
            let mut list10: Vec<XacVec2d> = Vec::new();
            for _ in 0..mesh.num_vertices {
                list10.push(xac_read_vec2d(file));
            }
            list5.push(list10);
        }
        if vertices_attribute.type_id == XacColor32Id as i32 {
            let mut list11: Vec<XacColor8> = Vec::new();
            for _ in 0..mesh.num_vertices {
                list11.push(xac_read_color8(file));
            }
            list8.push(list11);
        }
        if vertices_attribute.type_id == XacInfluenceRangeId as i32 {
            let mut something = Vec::new();
            for _ in 0..mesh.num_vertices {
                something.push(file.read_i32::<LittleEndian>().unwrap());
            }
            list6 = something.clone();
            vertices_attribute.mesh_influence_range_indices = something.clone();
        }
        if vertices_attribute.type_id == XacColor128Id as i32 {
            let mut list12: Vec<XacVec3d> = Vec::new();
            for _ in 0..mesh.num_vertices {
                list12.push(xac_read_vec3d(file));
            }
            list7.push(list12);
        }
        mesh.vertices_attribute.push(vertices_attribute);
    }
    let mut num8: usize = 0;
    let mut num9: usize = 0;

    for _ in 0..mesh.num_submeshes {
        let mut submeshes = XacSubMesh {
            num_indices: 0,
            num_vertices: 0,
            material_id: 0,
            num_bones: 0,
            sub_position: vec![],
            sub_normal: vec![],
            sub_tangent: vec![],
            sub_bi_tangent: vec![],
            sub_uv_set: vec![],
            sub_influence_range_indices: vec![],
            sub_color: vec![],
            sub_color8: vec![],
        };
        submeshes.num_indices = file.read_i32::<LittleEndian>().unwrap();
        submeshes.num_vertices = file.read_i32::<LittleEndian>().unwrap();
        submeshes.material_id = file.read_i32::<LittleEndian>().unwrap();
        submeshes.num_bones = file.read_i32::<LittleEndian>().unwrap();

        if !list.is_empty() {
            let temp = &list;
            submeshes.sub_position = temp[num8..submeshes.num_vertices.clone() as usize].to_vec();
        }
        if !list2.is_empty() {
            let temp = &list2;
            submeshes.sub_normal = temp[num8..submeshes.num_vertices.clone() as usize].to_vec();
        }
        if !list3.is_empty() {
            let temp = &list3;
            submeshes.sub_tangent = temp[num8..submeshes.num_vertices.clone() as usize].to_vec();
        }
        if !list4.is_empty() {
            let temp = &list4;
            submeshes.sub_bi_tangent = temp[num8..submeshes.num_vertices.clone() as usize].to_vec();
        }
        submeshes.sub_influence_range_indices =
            list6[num8..submeshes.num_vertices.clone() as usize].to_vec();

        for i in 0..list8.len() {
            let temp = &list8;
            submeshes
                .sub_color8
                .push(temp[i][num8..submeshes.num_vertices.clone() as usize].to_vec());
        }
        for i in 0..list7.len() {
            let temp = &list7;
            submeshes
                .sub_color
                .push(temp[i][num8..submeshes.num_vertices.clone() as usize].to_vec());
        }
        for i in 0..list5.len() {
            let temp = &list5;
            submeshes.sub_uv_set = temp[i][num8..submeshes.num_vertices.clone() as usize].to_vec();
        }

        for _ in 0..submeshes.num_indices {
            let temp = &list8;
            submeshes
                .sub_influence_range_indices
                .push(file.read_i32::<LittleEndian>().unwrap());
            xac.indices_num = xac.indices.len() as i32;
            xac.indices.push(xac.indices_num.clone() as u32);
        }

        for _ in 0..(4 * &submeshes.num_bones) {
            file.read_u8().unwrap();
        }
        num8 = num8 + submeshes.num_vertices.clone() as usize;
        num9 = num9 + submeshes.num_indices.clone() as usize;
        mesh.sub_mesh.push(submeshes);
    }
    xac.mesh.push(mesh);
    xac
}
fn read_skinning<'a>(file: &'a mut File, xac: &'a mut Xac) -> &'a mut Xac {
    xac
}

fn read_material_definition<'a>(file: &'a mut File, xac: &'a mut Xac) -> &'a mut Xac {
    xac.material_definition.ambient_color = xac_read_vec4d(file);
    xac.material_definition.diffuse_color = xac_read_vec4d(file);
    xac.material_definition.specular_color = xac_read_vec4d(file);
    xac.material_definition.emissive_color = xac_read_vec4d(file);
    xac.material_definition.shine = file.read_f32::<LittleEndian>().unwrap();
    xac.material_definition.shine_strength = file.read_f32::<LittleEndian>().unwrap();
    xac.material_definition.opacity = file.read_f32::<LittleEndian>().unwrap();
    xac.material_definition.ior = file.read_f32::<LittleEndian>().unwrap();
    xac.material_definition.double_sided = file.read_u8().unwrap();
    xac.material_definition.wireframe = file.read_u8().unwrap();
    file.read_u8().unwrap(); //Padding
    xac.material_definition.num_layers = file.read_u8().unwrap();
    xac.material_definition.name = xac_read_string(file);

    for _ in 0..xac.material_definition.num_layers {
        let mut layer = XacActorMaterialLayer {
            amount: 0.0,
            u_offset: 0.0,
            v_offset: 0.0,
            u_tiling: 0.0,
            v_tiling: 0.0,
            rotation_in_radian: 0.0,
            material_id: 0,
            map_type: 0,
            texture: "".to_string(),
        };
        layer.amount = file.read_f32::<LittleEndian>().unwrap();
        layer.u_offset = file.read_f32::<LittleEndian>().unwrap();
        layer.v_offset = file.read_f32::<LittleEndian>().unwrap();
        layer.u_tiling = file.read_f32::<LittleEndian>().unwrap();
        layer.v_tiling = file.read_f32::<LittleEndian>().unwrap();
        layer.rotation_in_radian = file.read_f32::<LittleEndian>().unwrap();
        layer.material_id = file.read_i16::<LittleEndian>().unwrap();
        layer.map_type = file.read_u8().unwrap();
        file.read_u8().unwrap(); //Padding
        layer.texture = xac_read_string(file);
        xac.material_definition.layers.push(layer);
    }
    xac
}

fn read_shader_material<'a>(file: &'a mut File, xac: &'a mut Xac) -> &'a mut Xac {
    let mut shader_material = XacShaderMaterial {
        num_int: 0,
        num_float: 0,
        num_bool: 0,
        num_string: 0,
        flag: 0,
        name_material: "".to_string(),
        name_shader: "".to_string(),
        int_property: vec![],
        float_property: vec![],
        bool_property: vec![],
        string_property: vec![],
    };
    shader_material.num_int = file.read_i32::<LittleEndian>().unwrap();
    shader_material.num_float = file.read_i32::<LittleEndian>().unwrap();
    shader_material.flag = file.read_i32::<LittleEndian>().unwrap();
    shader_material.num_bool = file.read_i32::<LittleEndian>().unwrap();
    file.read_i32::<LittleEndian>().unwrap(); //Padding
    shader_material.num_string = file.read_i32::<LittleEndian>().unwrap();
    shader_material.name_material = xac_read_string(file);
    shader_material.name_shader = xac_read_string(file);
    for _ in 0..shader_material.num_int {
        let mut int_property = XacIntProperties {
            name_properties: "".to_string(),
            value: 0,
        };
        int_property.name_properties = xac_read_string(file);
        int_property.value = file.read_i32::<LittleEndian>().unwrap();
        shader_material.int_property.push(int_property);
    }

    for _ in 0..shader_material.num_float {
        let mut float_property = XacFloatProperties {
            name_properties: "".to_string(),
            value: 0.0,
        };
        float_property.name_properties = xac_read_string(file);
        float_property.value = file.read_f32::<LittleEndian>().unwrap();
        shader_material.float_property.push(float_property);
    }

    for _ in 0..shader_material.num_bool {
        let mut bool_property = XacBoolProperties {
            name_properties: "".to_string(),
            value: 0,
        };
        bool_property.name_properties = xac_read_string(file);
        bool_property.value = file.read_u8().unwrap();
        shader_material.bool_property.push(bool_property);
    }
    let skip = file.read_i32::<LittleEndian>().unwrap(); //Padding
    for _ in 0..skip {
        file.read_u8().unwrap(); //Padding
    }

    for _ in 0..shader_material.num_string {
        let mut string_property = XacStringProperties {
            name_properties: "".to_string(),
            value: "".to_string(),
        };
        string_property.name_properties = xac_read_string(file);
        string_property.value = xac_read_string(file);
        shader_material.string_property.push(string_property);
    }
    xac.shader_material.push(shader_material);
    xac
}

fn read_metadata<'a>(file: &'a mut File, xac: &'a mut Xac) -> &'a mut Xac {
    xac.metadata.reposition_mask = file.read_u32::<LittleEndian>().unwrap();
    xac.metadata.repositioning_node = file.read_i32::<LittleEndian>().unwrap();
    xac.metadata.exporter_major_version = file.read_u8().unwrap();
    xac.metadata.exporter_minor_version = file.read_u8().unwrap();
    file.read_u8().unwrap(); //Padding
    file.read_u8().unwrap(); //Padding
    xac.metadata.retarget_root_offset = file.read_f32::<LittleEndian>().unwrap();
    xac.metadata.source_app = xac_read_string(file);
    xac.metadata.original_filename = xac_read_string(file);
    xac.metadata.export_date = xac_read_string(file);
    xac.metadata.actor_name = xac_read_string(file);
    xac
}

fn read_node_hierarchy<'a>(file: &'a mut File, xac: &'a mut Xac) -> &'a mut Xac {
    xac.node_hierarchy.num_nodes = file.read_i32::<LittleEndian>().unwrap();
    if xac.node_hierarchy.num_nodes <= 0 {
        panic!("Invalid number of nodes");
    }
    xac.node_hierarchy.num_root_nodes = file.read_i32::<LittleEndian>().unwrap();
    for _ in 0..xac.node_hierarchy.num_nodes {
        let mut xac_node_data = XacActorNode {
            rotation: XacQuaternion {
                x: 0,
                y: 0,
                z: 0,
                w: 0,
            },
            scale_rotation: XacQuaternion {
                x: 0,
                y: 0,
                z: 0,
                w: 0,
            },
            position: XacVec3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            scale: XacVec3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            parent_node_id: 0,
            num_child_nodes: 0,
            include_bounds_calc: 0,
            transform: XacMatrix44 {
                col1: XacVec4d {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    w: 0.0,
                },
                col2: XacVec4d {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    w: 0.0,
                },
                col3: XacVec4d {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    w: 0.0,
                },
                pos: XacVec4d {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    w: 0.0,
                },
            },
            importance_factor: 0.0,
            name: "".to_string(),
            parent: None,
            children: vec![],
        };
        xac_node_data.rotation = xac_read_quaternion(file);
        xac_node_data.scale_rotation = xac_read_quaternion(file);
        xac_node_data.position = xac_read_vec3d(file);
        xac_node_data.scale = xac_read_vec3d(file);
        file.read_f32::<LittleEndian>().unwrap(); //Padding
        file.read_f32::<LittleEndian>().unwrap(); //Padding
        file.read_f32::<LittleEndian>().unwrap(); //Padding
        file.read_i32::<LittleEndian>().unwrap(); //Padding
        file.read_i32::<LittleEndian>().unwrap(); //Padding
        xac_node_data.parent_node_id = file.read_i32::<LittleEndian>().unwrap();
        xac_node_data.num_child_nodes = file.read_i32::<LittleEndian>().unwrap();
        xac_node_data.include_bounds_calc = file.read_i32::<LittleEndian>().unwrap();
        xac_node_data.transform = xac_read_matrix44(file);
        xac_node_data.importance_factor = file.read_f32::<LittleEndian>().unwrap();
        xac_node_data.name = xac_read_string(file);
        /*
        if xac.node_hierarchy.node_data.len() != xac.node_hierarchy.num_nodes {
            panic!("numRootNodes does not match number of nodes with parent ID -1");
        }

         */
        if xac_node_data.parent_node_id == -1 {
        } else {
        }
        xac.node_hierarchy.node_data.push(xac_node_data);
    }
    xac
}
fn read_morph_target<'a>(file: &'a mut File, xac: &'a mut Xac) -> &'a mut Xac {
    xac
}
fn read_material_total<'a>(file: &'a mut File, xac: &'a mut Xac) -> &'a mut Xac {
    xac.material_total.num_total_materials = file.read_i32::<LittleEndian>().unwrap();
    xac.material_total.num_standard_materials = file.read_i32::<LittleEndian>().unwrap();
    xac.material_total.num_fx_materials = file.read_i32::<LittleEndian>().unwrap();
    xac
}
