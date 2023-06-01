#![allow(dead_code)]
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::xsm::xsm_enums::XsmChunkType::{XsmBoneAnimationId, XsmMetadataId};
use crate::xsm::xsm_structs::{
    Xsm, XsmBoneAnimation, XsmChunk, XsmHeader, XsmMetadata, XsmPosKey, XsmQuaternion16, XsmRotKey,
    XsmScaleKey, XsmScaleRotKey, XsmSubMotion, XsmVec3d,
};

pub fn xsmparse(path: &str) -> Xsm {
    println!("Path : {}", path);
    let mut xsm_file = File::open(path).expect("Cannot open xsm file!");
    let mut xsm_new = Xsm {
        header: XsmHeader {
            magic: "".to_string(),
            major_version: 0,
            minor_version: 0,
            big_endian: false,
        },
        metadata: XsmMetadata {
            unused: 0.0,
            max_acceptable_error: 0.0,
            fps: 0,
            exporter_major_version: 0,
            exporter_minor_version: 0,
            source_app: "".to_string(),
            original_filename: "".to_string(),
            export_date: "".to_string(),
            motion_name: "".to_string(),
        },
        bone_animation: XsmBoneAnimation {
            num_submotion: 0,
            skeletal_submotion: vec![],
        },
    };
    read_header(&mut xsm_file, &mut xsm_new);
    if xsm_new.header.magic == "XSM ".to_string() {
        read_chunk(&mut xsm_file, &mut xsm_new);
    } else {
        panic!("Wrong file header!");
    }
    xsm_new
}
fn xsm_read_string(file: &mut File) -> String {
    let mut text = String::new();
    let length = file.read_i32::<LittleEndian>().unwrap();
    for _ in 0..length {
        let character = file.read_u8().unwrap();
        text.push(character as char);
    }
    text
}

fn xsm_read_quaternion16(file: &mut File) -> XsmQuaternion16 {
    let quat = XsmQuaternion16 {
        x: file.read_i16::<LittleEndian>().unwrap(),
        y: file.read_i16::<LittleEndian>().unwrap(),
        z: file.read_i16::<LittleEndian>().unwrap(),
        w: file.read_i16::<LittleEndian>().unwrap(),
    };
    quat
}

fn xsm_read_vec3d(file: &mut File) -> XsmVec3d {
    let vec3d = XsmVec3d {
        x: file.read_f32::<LittleEndian>().unwrap(),
        y: file.read_f32::<LittleEndian>().unwrap(),
        z: file.read_f32::<LittleEndian>().unwrap(),
    };
    vec3d
}

fn read_header<'a>(file: &'a mut File, xsm: &'a mut Xsm) -> &'a mut Xsm {
    println!("Read Header");
    let mut magic = [0; 4];
    file.read_exact(&mut magic).unwrap();
    xsm.header.magic = std::str::from_utf8(&magic).unwrap().to_string();
    xsm.header.major_version = file.read_u8().unwrap();
    xsm.header.minor_version = file.read_u8().unwrap();
    xsm.header.big_endian = file.read_u8().unwrap() != 0;
    file.read_u8().unwrap(); // Padding
    println!("Header : {:?}", xsm.header);
    xsm
}

fn read_chunk<'a>(file: &'a mut File, xsm: &'a mut Xsm) -> &'a mut Xsm {
    println!("Read Chunk");
    while file.stream_position().unwrap() < file.metadata().unwrap().len() {
        let chunk = XsmChunk {
            chunk_type: file.read_i32::<LittleEndian>().unwrap(),
            length: file.read_i32::<LittleEndian>().unwrap(),
            version: file.read_i32::<LittleEndian>().unwrap(),
        };
        let position = file.stream_position().unwrap();
        if chunk.chunk_type == XsmMetadataId as i32 {
            read_metadata(file, xsm);
        }
        if chunk.chunk_type == XsmBoneAnimationId as i32 {
            read_bone_animation(file, xsm);
        }
        file.seek(SeekFrom::Start(position + chunk.length as u64))
            .unwrap();
    }
    xsm
}

fn read_metadata<'a>(file: &'a mut File, xsm: &'a mut Xsm) -> &'a mut Xsm {
    println!("Read Metadata");
    xsm.metadata.unused = file.read_f32::<LittleEndian>().unwrap();
    xsm.metadata.max_acceptable_error = file.read_f32::<LittleEndian>().unwrap();
    xsm.metadata.fps = file.read_i32::<LittleEndian>().unwrap();
    xsm.metadata.exporter_major_version = file.read_u8().unwrap();
    xsm.metadata.exporter_minor_version = file.read_u8().unwrap();
    file.read_u8().unwrap(); //Padding
    file.read_u8().unwrap(); //Padding
    xsm.metadata.source_app = xsm_read_string(file);
    xsm.metadata.original_filename = xsm_read_string(file);
    xsm.metadata.export_date = xsm_read_string(file);
    xsm.metadata.motion_name = xsm_read_string(file);
    println!("Metadata : {:?}", xsm.metadata);
    xsm
}

fn read_bone_animation<'a>(file: &'a mut File, xsm: &'a mut Xsm) -> &'a mut Xsm {
    println!("Read Bone Animation");
    xsm.bone_animation.num_submotion = file.read_i32::<LittleEndian>().unwrap();
    for _ in 0..xsm.bone_animation.num_submotion {
        xsm.bone_animation.skeletal_submotion.push({
            let mut submotion = XsmSubMotion {
                pose_rot: xsm_read_quaternion16(file),
                bind_pose_rot: xsm_read_quaternion16(file),
                pose_scale_rot: xsm_read_quaternion16(file),
                bind_pose_scale_rot: xsm_read_quaternion16(file),
                pose_pos: xsm_read_vec3d(file),
                pose_scale: xsm_read_vec3d(file),
                bind_pose_pos: xsm_read_vec3d(file),
                bind_pose_scale_pos: xsm_read_vec3d(file),
                num_pos_keys: file.read_i32::<LittleEndian>().unwrap(),
                num_rot_keys: file.read_i32::<LittleEndian>().unwrap(),
                num_scale_keys: file.read_i32::<LittleEndian>().unwrap(),
                num_scale_rot_keys: file.read_i32::<LittleEndian>().unwrap(),
                max_error: file.read_f32::<LittleEndian>().unwrap(),
                node_name: xsm_read_string(file),
                pos_key: vec![],
                rot_key: vec![],
                scale_key: vec![],
                scale_rot_key: vec![],
            };

            for _ in 0..submotion.num_pos_keys {
                submotion.pos_key.push(XsmPosKey {
                    pos: xsm_read_vec3d(file),
                    time: file.read_f32::<LittleEndian>().unwrap(),
                })
            }

            for _ in 0..submotion.num_rot_keys {
                submotion.rot_key.push(XsmRotKey {
                    rot: xsm_read_quaternion16(file),
                    time: file.read_f32::<LittleEndian>().unwrap(),
                })
            }
            for _ in 0..submotion.num_scale_keys {
                submotion.scale_key.push(XsmScaleKey {
                    scale: xsm_read_vec3d(file),
                    time: file.read_f32::<LittleEndian>().unwrap(),
                })
            }

            for _ in 0..submotion.num_scale_rot_keys {
                submotion.scale_rot_key.push(XsmScaleRotKey {
                    rot: xsm_read_quaternion16(file),
                    time: file.read_f32::<LittleEndian>().unwrap(),
                })
            }
            submotion
        });
    }

    xsm
}
