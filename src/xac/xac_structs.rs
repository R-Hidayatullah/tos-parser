#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct XacVec2d {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct XacVec3d {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct XacVec4d {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
    pub(crate) w: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct XacColor8 {
    pub(crate) x: u8,
    pub(crate) y: u8,
    pub(crate) z: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacQuaternion {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) z: i32,
    pub(crate) w: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacMatrix44 {
    pub(crate) col1: XacVec4d,
    pub(crate) col2: XacVec4d,
    pub(crate) col3: XacVec4d,
    pub(crate) pos: XacVec4d,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Xac {
    pub(crate) header: XacHeader,
    pub(crate) metadata: XacMetadata,
    pub(crate) node_hierarchy: XacNodeHierarchy,
    pub(crate) material_total: XacMaterialTotal,
    pub(crate) material_definition: XacActorMaterial,
    pub(crate) mesh: Vec<XacActorMesh>,
    pub(crate) skinning: XacSkinning,
    pub(crate) shader_material: Vec<XacShaderMaterial>,
    pub(crate) morph_target: XacActorMorphTarget,
    pub(crate) indices: Vec<u32>,
    pub(crate) indices_num: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacHeader {
    pub(crate) magic: String,
    pub(crate) major_version: u8,
    pub(crate) minor_version: u8,
    pub(crate) big_endian: u8,
    pub(crate) multiply_order: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacChunk {
    pub(crate) chunk_type: i32,
    pub(crate) length: i32,
    pub(crate) version: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacActorMesh {
    pub(crate) node_id: i32,
    pub(crate) num_influence_ranges: i32,
    pub(crate) num_vertices: i32,
    pub(crate) num_indices: i32,
    pub(crate) num_submeshes: i32,
    pub(crate) num_attribute_layers: i32,
    pub(crate) collision_mesh: u8,
    pub(crate) vertices_attribute: Vec<XacVerticesAttribute>,
    pub(crate) sub_mesh: Vec<XacSubMesh>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacVerticesAttribute {
    pub(crate) type_id: i32,
    pub(crate) attribute_size: i32,
    pub(crate) keep_originals: u8,
    pub(crate) scale_factor: u8,
    pub(crate) mesh_position: Vec<XacVec3d>,
    pub(crate) mesh_normal: Vec<XacVec3d>,
    pub(crate) mesh_unknown_vec4d: Vec<XacVec4d>,
    pub(crate) mesh_unknown_vec2d: Vec<XacVec2d>,
    pub(crate) mesh_color8: Vec<XacColor8>,
    pub(crate) mesh_influence_range_indices: Vec<i32>,
    pub(crate) mesh_color: Vec<XacVec4d>,
    pub(crate) mesh_data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacSubMesh {
    pub(crate) num_indices: i32,
    pub(crate) num_vertices: i32,
    pub(crate) material_id: i32,
    pub(crate) num_bones: i32,
    pub(crate) sub_position: Vec<XacVec3d>,
    pub(crate) sub_normal: Vec<XacVec3d>,
    pub(crate) sub_tangent: Vec<XacVec4d>,
    pub(crate) sub_bi_tangent: Vec<XacVec4d>,
    pub(crate) sub_uv_set: Vec<XacVec2d>,
    pub(crate) sub_influence_range_indices: Vec<i32>,
    pub(crate) sub_color: Vec<Vec<XacVec3d>>,
    pub(crate) sub_color8: Vec<Vec<XacColor8>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacSkinning {
    pub(crate) node_id: i32,
    pub(crate) num_local_bones: i32,
    pub(crate) num_influences: i32,
    pub(crate) for_collision_mesh: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacInfluenceData {
    pub(crate) weight: f32,
    pub(crate) bone_id: i16,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacInfluenceRange {
    pub(crate) first_influence_index: i32,
    pub(crate) num_influences: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacActorMaterial {
    pub(crate) ambient_color: XacVec4d,
    pub(crate) diffuse_color: XacVec4d,
    pub(crate) specular_color: XacVec4d,
    pub(crate) emissive_color: XacVec4d,
    pub(crate) shine: f32,
    pub(crate) shine_strength: f32,
    pub(crate) opacity: f32,
    pub(crate) ior: f32,
    pub(crate) double_sided: u8,
    pub(crate) wireframe: u8,
    pub(crate) num_layers: u8,
    pub(crate) name: String,
    pub(crate) layers: Vec<XacActorMaterialLayer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacActorMaterialLayer {
    pub(crate) amount: f32,
    pub(crate) u_offset: f32,
    pub(crate) v_offset: f32,
    pub(crate) u_tiling: f32,
    pub(crate) v_tiling: f32,
    pub(crate) rotation_in_radian: f32,
    pub(crate) material_id: i16,
    pub(crate) map_type: u8,
    pub(crate) texture: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacMetadata {
    pub(crate) reposition_mask: u32,
    pub(crate) repositioning_node: i32,
    pub(crate) exporter_major_version: u8,
    pub(crate) exporter_minor_version: u8,
    pub(crate) retarget_root_offset: f32,
    pub(crate) source_app: String,
    pub(crate) original_filename: String,
    pub(crate) export_date: String,
    pub(crate) actor_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacNodeHierarchy {
    pub(crate) num_nodes: i32,
    pub(crate) num_root_nodes: i32,
    pub(crate) node_data: Vec<XacActorNode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacActorNode {
    pub(crate) rotation: XacQuaternion,
    pub(crate) scale_rotation: XacQuaternion,
    pub(crate) position: XacVec3d,
    pub(crate) scale: XacVec3d,
    pub(crate) parent_node_id: i32,
    pub(crate) num_child_nodes: i32,
    pub(crate) include_bounds_calc: i32,
    pub(crate) transform: XacMatrix44,
    pub(crate) importance_factor: f32,
    pub(crate) name: String,
    pub(crate) parent: Option<Box<XacActorNode>>,
    pub(crate) children: Vec<XacActorNode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacActorMorphTarget {
    pub(crate) num_morph_targets: i32,
    pub(crate) lod_morph_target_id: i32,
    pub(crate) morph_targets: Vec<XacMorphTargetData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacMorphTargetData {
    pub(crate) range_min: f32,
    pub(crate) range_max: f32,
    pub(crate) lod_level: i32,
    pub(crate) num_deformations: i32,
    pub(crate) num_transformations: i32,
    pub(crate) phoneme_set_bitmask: i32,
    pub(crate) name: String,
    pub(crate) deformation: Vec<XacDeformation>,
    pub(crate) transformation: Vec<XacTransformation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacDeformation {
    pub(crate) node_id: i32,
    pub(crate) min_value: f32,
    pub(crate) max_value: f32,
    pub(crate) num_vertices: i32,
    pub(crate) position_offset: Vec<XacPositionOffset>,
    pub(crate) normal_offset: Vec<XacNormalOffset>,
    pub(crate) tangent_offset: Vec<XacTangentOffset>,
    pub(crate) vertex_indices: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacPositionOffset {
    pub(crate) x: u16,
    pub(crate) y: u16,
    pub(crate) z: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacNormalOffset {
    pub(crate) x: u8,
    pub(crate) y: u8,
    pub(crate) z: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacTangentOffset {}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacTransformation {
    pub(crate) node_id: i32,
    pub(crate) rotation: XacQuaternion,
    pub(crate) scale_rotation: XacQuaternion,
    pub(crate) position: XacVec3d,
    pub(crate) scale: XacVec3d,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacMaterialTotal {
    pub(crate) num_total_materials: i32,
    pub(crate) num_standard_materials: i32,
    pub(crate) num_fx_materials: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacIntProperties {
    pub(crate) name_properties: String,
    pub(crate) value: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacFloatProperties {
    pub(crate) name_properties: String,
    pub(crate) value: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacBoolProperties {
    pub(crate) name_properties: String,
    pub(crate) value: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacStringProperties {
    pub(crate) name_properties: String,
    pub(crate) value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacShaderMaterial {
    pub(crate) num_int: i32,
    pub(crate) num_float: i32,
    pub(crate) num_bool: i32,
    pub(crate) num_string: i32,
    pub(crate) flag: i32,
    pub(crate) name_material: String,
    pub(crate) name_shader: String,
    pub(crate) int_property: Vec<XacIntProperties>,
    pub(crate) float_property: Vec<XacFloatProperties>,
    pub(crate) bool_property: Vec<XacBoolProperties>,
    pub(crate) string_property: Vec<XacStringProperties>,
}
