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
        println!("Found dir: {}", path.display());
    } else if path.is_file() {
        process_file(path);
    } else {
        eprintln!("提供的路径无效: {}", path.display());
    }
}

fn process_file(path: &Path) {
    let content = fs::read_to_string(path).expect("read the file");

    println!("// {}", path.display());
    println!("{}\n", content);
}
