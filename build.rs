use std::{env, fs::{self, create_dir_all, read_dir}, io, path::{Path, PathBuf}};

/// Returns the `target` directory for the current build.
pub fn get_target_dir() -> PathBuf {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR must be set"));

    // climb up three levels: `.../build/<crate>/out` → `target/debug` or `target/release`
    out_dir
        .parent().expect("OUT_DIR parent missing")
        .parent().expect("OUT_DIR grandparent missing")
        .parent().expect("OUT_DIR great-grandparent missing")
        .to_path_buf()
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    create_dir_all(&dst)?;
    
    for entry in read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
            println!("cargo:rerun-if-changed={}", entry.path().display());
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
            println!("cargo:rerun-if-changed={}", entry.path().display());
        }
    }
    Ok(())
}

fn main() {

    let target_dir = get_target_dir();
    let output_path = target_dir.join("data");

    let source_dir = "data";

    // fs::copy(source_dir, output_path).unwrap();
    copy_dir_all(source_dir, output_path).unwrap();

    println!("cargo:rerun-if-changed={}", source_dir);
}
