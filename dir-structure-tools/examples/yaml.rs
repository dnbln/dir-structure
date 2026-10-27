//! An example demonstrating the use of [`Yaml`](dir_structure::data_formats::yaml::Yaml) to read and write YAML files.
//!
//! Requires the `derive` and `yaml` features to be enabled.

use std::error::Error;

use dir_structure::prelude::*;
use serde::{Deserialize, Serialize};

mod example_dirs;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    new_key: String,
    number: u32,
    array: Vec<u32>,
}

#[derive(dir_structure::DirStructure)]
struct Dir {
    #[dir_structure(path = "data.yaml", with_newtype = dir_structure_tools::data_formats::yaml::Yaml<Data>)]
    data: Data,
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = example_dirs::get_example_dir_path("yaml_rw");

    let dir = Dir::read(&path)?;

    println!("Read YAML data: {:?}", dir.data);

    let new_dir = Dir {
        data: Data {
            new_key: "new_value".to_owned(),
            number: 42,
            array: vec![1, 2, 3],
        },
    };

    new_dir.write(&path)?;

    println!(
        "Wrote new YAML data to {}",
        path.join("data.yaml").display()
    );

    Ok(())
}
