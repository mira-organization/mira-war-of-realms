use std::fs;
use std::env;
use std::path::PathBuf;

fn main() {
    let target_dir = env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("target"))
        .join(env::var("PROFILE").unwrap_or_else(|_| "release".to_string()));

    let libs_dir = PathBuf::from("libs");

    fs::create_dir_all(&libs_dir).unwrap();

    let extensions = if cfg!(target_os = "windows") {
        vec!["dll"]
    } else if cfg!(target_os = "macos") {
        vec!["dylib"]
    } else {
        vec!["so"]
    };

    if let Ok(entries) = target_dir.read_dir() {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if extensions.contains(&ext.to_str().unwrap()) {
                        let dest = libs_dir.join(path.file_name().unwrap());
                        fs::copy(&path, &dest).unwrap();
                        println!("Moved {:?} -> {:?}", path, dest);
                    }
                }
            }
        }
    }
}