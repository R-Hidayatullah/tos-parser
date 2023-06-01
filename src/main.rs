use std::path::Path;
use crate::xsm::xsm_parser::xsmparse;

mod xac;
mod xsm;

fn main() {
    println!("This is XSM to JSON file converter for Tree of Savior!");
    let file_location = std::env::args().nth(1).expect("No xsm file location given!");
    let filepath=xsmparse(file_location.as_str());
    let mut output_file = std::fs::File::create(Path::new(&file_location).file_stem().unwrap().to_str().unwrap().to_owned()+".json").unwrap();
    println!("{:#?}",filepath.header);
    println!("{:#?}",filepath.metadata);
    serde_json::to_writer_pretty(&mut output_file, &filepath).unwrap();
}
