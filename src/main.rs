use glob::glob;
use colored::Colorize;
use std::env;
use std::fs;
use std::process;

mod ui;

fn main() {
    println!();
    println!("Cleanup After Apple");
    println!("{} v{}", ui::get_prog_name().yellow().bold(), env!("CARGO_PKG_VERSION"));

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        ui::usage();
        process::exit(1);
    }

    let cmd = &args[1];
    let mut delete_flag: bool = false;
    if cmd == "--delete" {
        delete_flag = true;
    }

    if delete_flag == true {
        println!("\ndeleting...");
    } else {
        println!("\ndry run...");
    }

    process_files(delete_flag);
}

fn process_files(flag: bool) {
    let cwd = env::current_dir().unwrap();
    let fpath = cwd.join("**/.DS_Store");

    for entry in glob(&fpath.display().to_string()).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                println!("{:?}", path.display());
                if flag == true {
                    fs::remove_file(path).expect("File delete failed");
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
