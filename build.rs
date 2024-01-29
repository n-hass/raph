use std::env;
use std::fs;
use std::io::{self, ErrorKind};
use std::path::Path;

fn main() -> io::Result<()> {
    let home_dir = env::var("HOME").map_err(|e| io::Error::new(ErrorKind::Other, e.to_string()))?;
    let cargo_bin_path = Path::new(&home_dir).join(".cargo/bin");

    let root_dir = env::var("CARGO_MANIFEST_DIR").map_err(|e| io::Error::new(ErrorKind::Other, e.to_string()))?;
    let raph_script_path = Path::new(&root_dir).join("_raph");

    // Copy the script to .cargo/bin
    if cargo_bin_path.exists() && raph_script_path.exists() {
        println!("cargo:warning=Copying _raph to {}", cargo_bin_path.display());
        fs::copy(&raph_script_path, cargo_bin_path.join("_raph"))?;
    }
    
    Ok(())
}