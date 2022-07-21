use glob::glob;
use std::env;
use std::fs;

fn main() {
    println!("Cleanup After Apple: cuaa v{}", env!("CARGO_PKG_VERSION"));
    println!("\ndeleting...");

    let cwd = env::current_dir().unwrap();
    let fpath = cwd.join("**/.DS_Store");

    for entry in glob(&fpath.display().to_string()).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                println!("{:?}", path.display());
                fs::remove_file(path).expect("File delete failed");
            },
            Err(e) => println!("{:?}", e),
        }
    }
}
