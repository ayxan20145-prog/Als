use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let dir = match env::current_dir() {
        Ok(e) => e,
        Err(e) => {
            println!("Error: {e}");
            return;
        }
    };
    let mut list: Vec<(PathBuf, bool)> = Vec::new();
    let entries = match fs::read_dir(&dir) {
        Ok(entries) => entries,
        Err(e) => {
            println!("Error: {e}");
            return;
        }
    };
    if env::args().any(|a| a == "-a") {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                let is_dir = path.is_dir();

                list.push((path, is_dir));
            }
        }

        for (path, is_dir) in &list {
            let name = path.file_name().unwrap_or_default().to_string_lossy();

            if *is_dir {
                println!("{}/", name);
            } else {
                println!("{}", name);
            }
        }
    } else {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                let is_dir = path.is_dir();
                let name = path.file_name().unwrap_or_default().to_string_lossy();

                if name.starts_with('.') {
                    continue;
                }

                list.push((path, is_dir));
            }
        }

        for (path, is_dir) in &list {
            let name = path.file_name().unwrap_or_default().to_string_lossy();

            if *is_dir {
                println!("{}/", name);
            } else {
                println!("{}", name);
            }
        }
    }
}
