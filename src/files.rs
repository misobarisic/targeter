use std::path::{Path, PathBuf};

use scan_dir::ScanDir;

pub fn get_all_cargo_dirs<P: AsRef<Path>>(input: P) -> Vec<PathBuf> {
    ScanDir::dirs()
        .walk(&input, |iter| {
            iter.filter(|&(_, ref name)| name == "target").map(|(ref entry, _)| entry.path()).collect::<Vec<PathBuf>>()
        })
        .unwrap_or_else(|e| panic!("Error processing a file {:?}", e))
        .iter()
        .cloned()
        .filter(|file| {
            let mut pathbuf = file.clone();
            pathbuf.set_file_name("Cargo.toml");
            Path::new(&pathbuf).exists()
        })
        .collect()
}
