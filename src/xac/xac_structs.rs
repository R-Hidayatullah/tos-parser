#![allow(dead_code)]

#[derive(Debug)]
pub(crate) struct XacVec3d {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

#[derive(Debug)]
pub(crate) struct XacVec4d {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
    pub(crate) w: f32,
}

#[derive(Debug)]
pub(crate) struct XacQuaternion {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) z: i32,
    pub(crate) w: i32,
}

#[derive(Debug)]
pub(crate) struct XacMatrix44 {
    pub(crate) col1: XacVec4d,
    pub(crate) col2: XacVec4d,
    pub(crate) col3: XacVec4d,
    pub(crate) pos: XacVec4d,
}

#[derive(Debug)]
pub struct Xac {
    pub(crate) header: XacHeader,
    pub(crate) metadata: XacMetadata,
}

#[derive(Debug)]
pub(crate) struct XacHeader {
    pub(crate) magic: String,
    pub(crate) major_version: u8,
    pub(crate) minor_version: u8,
    pub(crate) big_endian: bool,
    pub(crate) multiply_order: u8,
}

#[derive(Debug)]
pub(crate) struct XacChunk {
    pub(crate) chunk_type: i32,
    pub(crate) length: i32,
    pub(crate) version: i32,
}

#[derive(Debug)]
pub(crate) struct XacMesh {
    pub(crate) node_id: i32,
    pub(crate) num_influence_ranges: i32,
    pub(crate) num_vertices: i32,
    pub(crate) num_indices: i32,
    pub(crate) num_submeshes: i32,
    pub(crate) num_attribute_layers: i32,
    pub(crate) collision_mesh: bool,
}

#[derive(Debug)]
pub(crate) struct XacVerticesAttribute {
    pub(crate) type_id: i32,
    pub(crate) attribute_size: i32,
    pub(crate) keep_originals: bool,
    pub(crate) scale_factor: bool,
}

#[derive(Debug)]
pub(crate) struct XacSubMesh {
    pub(crate) num_indices: i32,
    pub(crate) num_vertices: i32,
    pub(crate) material_id: i32,
    pub(crate) num_bones: i32,
    pub(crate) relative_indices: i32,
    pub(crate) bone_ids: Vec<i32>,
}

pub(crate) struct XacSkinning {
    pub(crate) node_id: i32,
    pub(crate) num_local_bones: i32,
    pub(crate) num_influences: i32,
    pub(crate) for_collision_mesh: bool,
}

#[derive(Debug)]
pub(crate) struct XacInfluenceData {
    pub(crate) weight: f32,
    pub(crate) bone_id: i16,
}

#[derive(Debug)]
pub(crate) struct XacInfluenceRange {
    pub(crate) first_influence_index: i32,
    pub(crate) num_influences: i32,
}

#[derive(Debug)]
pub(crate) struct XacMaterialDefinition {
    pub(crate) ambient_color: XacVec4d,
    pub(crate) diffuse_color: XacVec4d,
    pub(crate) specular_color: XacVec4d,
    pub(crate) emissive_color: XacVec4d,
    pub(crate) shine: f32,
    pub(crate) shine_strength: f32,
    pub(crate) opacity: f32,
    pub(crate) ior: f32,
    pub(crate) double_sided: bool,
    pub(crate) wireframe: bool,
    pub(crate) num_layers: u8,
    pub(crate) name: String,
    pub(crate) layers: Vec<XacLayer>,
}

#[derive(Debug)]
pub(crate) struct XacLayer {
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

#[derive(Debug)]
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

#[derive(Debug)]
pub(crate) struct XacNodeHierarchy {
    pub(crate) num_nodes: i32,
    pub(crate) num_root_nodes: i32,
    pub(crate) node_data: Vec<XacNodeData>,
}

#[derive(Debug)]
pub(crate) struct XacNodeData {
    pub(crate) rotation: XacQuaternion,
    pub(crate) scale_rotation: XacQuaternion,
    pub(crate) position: XacVec3d,
    pub(crate) scale: XacVec3d,
    pub(crate) parent_node_id: i32,
    pub(crate) num_child_nodes: i32,
    pub(crate) include_bounds_calc: bool,
    pub(crate) transform: XacMatrix44,
    pub(crate) importance_factor: f32,
    pub(crate) name: String,
}

#[derive(Debug)]
pub(crate) struct XacMorphTarget {
    pub(crate) num_morph_targets: i32,
    pub(crate) lod_morph_target_id: i32,
    pub(crate) morph_targets: Vec<XacMorphTargetData>,
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub(crate) struct XacPositionOffset {
    pub(crate) x: u16,
    pub(crate) y: u16,
    pub(crate) z: u16,
}

#[derive(Debug)]
pub(crate) struct XacNormalOffset {
    pub(crate) x: u8,
    pub(crate) y: u8,
    pub(crate) z: u8,
}

#[derive(Debug)]
pub(crate) struct XacTangentOffset {}

#[derive(Debug)]
pub(crate) struct XacTransformation {
    pub(crate) node_id: i32,
    pub(crate) rotation: XacQuaternion,
    pub(crate) scale_rotation: XacQuaternion,
    pub(crate) position: XacVec3d,
    pub(crate) scale: XacVec3d,
}

#[derive(Debug)]
pub(crate) struct XacMaterialTotal {
    pub(crate) num_total_materials: i32,
    pub(crate) num_standard_materials: i32,
    pub(crate) num_fx_materials: i32,
}
