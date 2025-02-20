use std::fs;
use std::path::PathBuf;

fn main() {
    let target_dir = PathBuf::from("target/release");
    let libs_dir = PathBuf::from("libs");

    fs::create_dir_all(&libs_dir).unwrap();

    let extensions = if cfg!(target_os = "windows") {
        vec!["dll"]
    } else if cfg!(target_os = "macos") {
        vec!["dylib"]
    } else {
        vec!["so"]
    };

    for entry in target_dir.read_dir().unwrap() {
        if let Ok(entry) = entry {
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