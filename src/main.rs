use glob::glob;
use std::env;
use std::fs;
use std::io;
use std::process;

use crossterm::style::Color;

mod tui_gen;

fn main() -> io::Result<()> {
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

    let deleted_count = process_files(delete_flag)?;
    println!("file count: {}", deleted_count);

    Ok(())
}

fn process_files(delete_flag: bool) -> io::Result<usize> {
    let mut count = 0;
    let cwd = env::current_dir()?;
    let fpath = cwd.join("**/.DS_Store");

    for entry in glob(&fpath.display().to_string()).expect("Failed to read glob pattern") {
        count += 1;
        match entry {
            Ok(path) => {
                println!("{:?}", path.display());
                if delete_flag {
                    fs::remove_file(path)?;
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }

    Ok(count)
}

fn usage() {
    println!("\n\n    Invalid arguments provided...\n");
    println!("    USAGE:");
    println!("        ./cuaa [OPTION]\n");
    println!("    OPTIONS:");
    println!("        --dry-run     Dry run - display files without deleting");
    println!("        --delete      Delete files\n");
}
