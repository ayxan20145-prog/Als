use colored::Colorize;
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    let help = args.contains(&"-h".to_string());
    let show_hidden = args.contains(&"-a".to_string());
    let long_format = args.contains(&"-l".to_string());

    if help {
        println!("-h -> help");
        println!("-a -> show hidden files");
        println!("-l -> long list format");
        return;
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
            if show_hidden == false {
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
        if long_format {
            let meta = match fs::metadata(&path) {
                Ok(e) => e,
                Err(e) => {
                    println!("Error: {e}");
                    continue;
                }
            };

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
            } else if is_executable(path) {
                println!("{}*", name.green());
            } else {
                println!("{}", name);
            }
        }
    }
}
fn is_executable(path: &Path) -> bool {
    match fs::metadata(path) {
        Ok(meta) => {
            let perms = meta.permissions();
            perms.mode() & 0o111 != 0
        }
        Err(_) => false,
    }
}
