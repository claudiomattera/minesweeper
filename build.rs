use std::env::var;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use png2wasm4src::build_sprite_modules_tree;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let module = build_sprite_modules_tree("assets/sprites")?;

    let mut cargo_instructions = String::default();
    module.generate_cargo_build_instructions(&mut cargo_instructions)?;
    println!("{}", cargo_instructions);

    let module = module.parse()?;
    println!("Module: {:?}", module);
    let mut output_file = open_output_file()?;
    writeln!(output_file, "{}", module)?;

    Ok(())
}

fn open_output_file() -> Result<File, Box<dyn std::error::Error>> {
    let output_directory = PathBuf::from(var("OUT_DIR")?);
    let output_path = output_directory.join("sprites.rs");
    let output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(output_path)?;
    Ok(output_file)
}
