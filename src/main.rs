use glob::glob;
use std::env;
use std::fs;
use std::process;

fn main() {
    println!("Cleanup After Apple: cuaa v{}", env!("CARGO_PKG_VERSION"));

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

    if delete_flag == true {
        println!("\ndeleting...");
    }
    else {
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
            },
            Err(e) => println!("{:?}", e),
        }
    }
}

fn usage() {
    println!("Invalid arguments provided...\n");
    println!("USAGE:");
    println!("    ./cuaa [OPTION]\n");
    println!("OPTIONS:");
    println!("    --dry_run     Dry run - display files without deleting");
    println!("    --delete      Delete files\n");
}

