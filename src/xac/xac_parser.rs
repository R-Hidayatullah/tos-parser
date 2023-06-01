#![allow(dead_code)]

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::xac::xac_enums::XacChunkType::XacMetadataId;
use crate::xac::xac_structs::{
    Xac, XacChunk, XacHeader, XacMatrix44, XacMetadata, XacQuaternion, XacVec3d, XacVec4d,
};

pub fn xacparse(path: &str) -> Xac {
    println!("Path : {}", path);
    let mut xac_file = File::open(path).expect("Cannot open xac file!");
    let mut xac_new = Xac {
        header: XacHeader {
            magic: "".to_string(),
            major_version: 0,
            minor_version: 0,
            big_endian: false,
            multiply_order: 0,
        },
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
fn xac_read_vec3d(file: &mut File) -> XacVec3d {
    let vec3d = XacVec3d {
        x: file.read_f32::<LittleEndian>().unwrap(),
        y: file.read_f32::<LittleEndian>().unwrap(),
        z: file.read_f32::<LittleEndian>().unwrap(),
    };
    vec3d
}
fn xac_read_vec4d(file: &mut File) -> XacVec4d {
    let vec3d = XacVec4d {
        x: file.read_f32::<LittleEndian>().unwrap(),
        y: file.read_f32::<LittleEndian>().unwrap(),
        z: file.read_f32::<LittleEndian>().unwrap(),
        w: file.read_f32::<LittleEndian>().unwrap(),
    };
    vec3d
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
    println!("Read Header");
    let mut magic = [0; 4];
    file.read_exact(&mut magic).unwrap();
    xac.header.magic = std::str::from_utf8(&magic).unwrap().to_string();
    xac.header.major_version = file.read_u8().unwrap();
    xac.header.minor_version = file.read_u8().unwrap();
    xac.header.big_endian = file.read_u8().unwrap() != 0;
    xac.header.multiply_order = file.read_u8().unwrap();
    println!("Header : {:?}", xac.header);
    xac
}
fn read_chunk<'a>(file: &'a mut File, xac: &'a mut Xac) -> &'a mut Xac {
    println!("Read Chunk");
    while file.stream_position().unwrap() < file.metadata().unwrap().len() {
        let chunk = XacChunk {
            chunk_type: file.read_i32::<LittleEndian>().unwrap(),
            length: file.read_i32::<LittleEndian>().unwrap(),
            version: file.read_i32::<LittleEndian>().unwrap(),
        };
        let position = file.stream_position().unwrap();
        if chunk.chunk_type == XacMetadataId as i32 {
            read_metadata(file, xac);
        }
        file.seek(SeekFrom::Start(position + chunk.length as u64))
            .unwrap();
    }
    xac
}

fn read_metadata<'a>(file: &'a mut File, xac: &'a mut Xac) -> &'a mut Xac {
    println!("Read Metadata");
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
    println!("Metadata : {:?}", xac.metadata);
    xac
}
