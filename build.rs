use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let levels_dir = Path::new(&manifest_dir).join("levels");

    println!("cargo:rerun-if-changed={}", levels_dir.display());

    let mut level_nums: Vec<u32> = fs::read_dir(&levels_dir)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", levels_dir.display()))
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir())
        .filter_map(|entry| entry.file_name().to_str()?.parse::<u32>().ok())
        .collect();
    level_nums.sort_unstable();

    if level_nums.is_empty() {
        panic!(
            "no numbered level directories found in {}",
            levels_dir.display()
        );
    }

    let mut code = String::from("pub const LEVEL_DATA: &[(&[u8], &[u8])] = &[\n");

    for n in &level_nums {
        let level_dir = levels_dir.join(n.to_string());
        let floor_path = level_dir.join("floor.png");
        let tiles_path = level_dir.join("tiles.png");

        if !floor_path.exists() {
            panic!("missing {}", floor_path.display());
        }
        if !tiles_path.exists() {
            panic!("missing {}", tiles_path.display());
        }

        println!("cargo:rerun-if-changed={}", floor_path.display());
        println!("cargo:rerun-if-changed={}", tiles_path.display());

        let floor_lit = floor_path.display().to_string().replace('\\', "\\\\");
        let tiles_lit = tiles_path.display().to_string().replace('\\', "\\\\");

        code.push_str(&format!(
            "    (include_bytes!(\"{floor_lit}\"), include_bytes!(\"{tiles_lit}\")),\n"
        ));
    }

    code.push_str("];\n");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("level_data.rs");
    fs::write(&dest_path, code)
        .unwrap_or_else(|e| panic!("failed to write {}: {e}", dest_path.display()));
}
