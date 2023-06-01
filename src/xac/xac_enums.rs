#![allow(dead_code)]
pub(crate) enum XacChunkType {
    XacMeshId = 1,
    XacSkinningId = 2,
    XacMaterialDefinitionId = 3,
    XacShaderMaterialId = 5,
    XacMetadataId = 7,
    XacNodeHierarchyId = 11,
    XacMorphTargetId = 12,
    XacMaterialTotalId = 13,
}

pub(crate) enum XacVerticesAttributeType {
    XacPositionId = 0,
    XacNormalId = 1,
    XacTangentId = 2,
    XacUVCoordId = 3,
    XacColor32Id = 4,
    XacInfluenceRangeId = 5,
    XacColor128Id = 6,
}

pub(crate) enum XacPhonemeType {
    XacNeutralId = 0x1,
    XacMBPXId = 0x2,
    XacAAAOOWId = 0x4,
    XacIHAEAHEYAYHId = 0x8,
    XacAWId = 0x10,
    XacNNGCHJDHDGTKZZHTHSSHId = 0x20,
    XacIYEHYId = 0x40,
    XacUWUHOYId = 0x80,
    XacFVId = 0x100,
    XacLELId = 0x200,
    XacWId = 0x400,
    XacRERId = 0x800,
}
