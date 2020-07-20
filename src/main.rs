use std::fs::{create_dir_all, remove_dir_all, File};
use std::io::prelude::*;
use std::path::Path;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let out_dir = Path::new("dist");
    let _site_dir = Path::new("site");

    match remove_dir_all(out_dir) {
        Ok(()) => (),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => (),
        Err(err) => return Err(err).context("Failed to remove output directory"),
    };
    create_dir_all(out_dir).context("Failed to create output directory")?;

    let mut index_file = File::create(out_dir.join("index.html"))?;
    index_file.write_all(b"Hello, world!")?;
    Ok(())
}
