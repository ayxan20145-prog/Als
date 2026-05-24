use colored::Colorize;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    if env::args().any(|h| h == "-h") {
        println!("-a -> show hidden files");
        println!("-l -> long list format");
    }

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
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            let is_dir = path.is_dir();
            if !env::args().any(|a| a == "-a") {
                let name = path.file_name().unwrap_or_default().to_string_lossy();
                if name.starts_with('.') {
                    continue;
                }
            }
            list.push((path, is_dir));
        }
    }
    for (path, is_dir) in &list {
        let name = path.file_name().unwrap_or_default().to_string_lossy();
        if env::args().any(|l| l == "-l") {
            let meta = fs::metadata(&path).unwrap();

            if *is_dir {
                println!(
                    "{:?} {} {:?} {}/",
                    meta.permissions(),
                    meta.len(),
                    meta.modified(),
                    name.blue()
                );
            } else {
                println!(
                    "{:?} {} {:?} {}",
                    meta.permissions(),
                    meta.len(),
                    meta.modified(),
                    name
                );
            }
        } else {
            if *is_dir {
                println!("{}/", name.blue());
            } else {
                println!("{}", name);
            }
        }
    }
}
