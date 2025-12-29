use std::env;
use std::fs;
use std::path::Path;
fn main() {
    // println!("Code Weaver: System Online");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("请提供一个路径作为参数。");
        return;
    }
    let path_str = &args[1];
    let path = Path::new(path_str);
    if path.is_dir() {
        process_dir(path);
    } else if path.is_file() {
        process_file(path);
    } else {
        eprintln!("提供的路径无效: {}", path.display());
    }
}

fn process_dir(path: &Path) {
    let entries = fs::read_dir(path).expect("failed to read directory");
    // for entry in entries.flatten() {
    //     let path = entry.path();
    //     let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    //     if file_name.starts_with('.') || file_name == "target" {
    //         continue; // Skip hidden files and directories
    //     }
    //     if path.is_dir() {
    //         process_dir(&path);
    //     } else if path.is_file() {
    //         process_file(&path);
    //     }
    // }
    let allowed_exts = [
        "rs", "py", "c", "cpp", "java", "go", "js", "ts", "tsx", "css", "html", "md", "txt",
        "json", "yml", "toml", "yaml",
    ];
    entries
        .flatten()
        .filter(|entry| {
            let path = entry.path();
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if file_name.starts_with(".") || file_name == "target" {
                return false;
            }
            if path.is_dir() {
                return true;
            }
            let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
            allowed_exts.contains(&ext)
        })
        .for_each(|entry| {
            let path = entry.path();
            if path.is_file() {
                process_file(&path);
            } else if path.is_dir() {
                process_dir(&path);
            }
        });
}

fn process_file(path: &Path) {
    // let content = fs::read_to_string(path).expect("failed to read file");
    //
    // println!("// {}", path.display());
    // println!("{}\n", content);
    match fs::read_to_string(path) {
        Ok(content) => {
            println!("// {}", path.display());
            println!("{}\n", content);
        }
        Err(e) => {
            eprintln!("无法读取文件 {}: {}", path.display(), e);
        }
    }
}
