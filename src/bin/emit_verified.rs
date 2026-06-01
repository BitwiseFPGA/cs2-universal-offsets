// Standalone helper: emit verified_features.{json,hpp} without needing a
// live cs2.exe attach. Useful when only the curated catalogue changes.
#[path = "../output/verified.rs"]
mod verified;

use std::fs;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let out_dir: PathBuf = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("include"));
    fs::create_dir_all(&out_dir)?;

    fs::write(out_dir.join("verified_features.json"), verified::render_json(None))?;
    fs::write(out_dir.join("verified_features.hpp"), verified::render_hpp(None))?;

    println!("emitted verified_features.{{json,hpp}} -> {}", out_dir.display());
    Ok(())
}
