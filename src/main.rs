use glob::glob;
use std::env;
use std::fs;
use std::process;

use crossterm::style::Color;

mod tui_gen;

fn main() {

    println!();
    println!("Cleanup After Apple");
    let buffer = format!(
        "{} v{}",
        tui_gen::get_prog_name(),
        env!("CARGO_PKG_VERSION")
    );
    tui_gen::print_color_bold(&buffer, Color::DarkYellow);

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage();
        process::exit(1);
    }

    let cmd = &args[1];
    let mut delete_flag: bool = false;
    if cmd == "--delete" {
        delete_flag = true;
    }

    if delete_flag {
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
                if flag {
                    fs::remove_file(path).expect("File delete failed");
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

fn usage() {
    println!("\n\n    Invalid arguments provided...\n");
    println!("    USAGE:");
    println!("        ./cuaa [OPTION]\n");
    println!("    OPTIONS:");
    println!("        --dry-run     Dry run - display files without deleting");
    println!("        --delete      Delete files\n");
}
