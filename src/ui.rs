// ui.rs

//use std::env;

/*
pub fn get_prog_name() -> String {
    let prog_name = env::current_exe()
        .expect("Can't get the exec path")
        .file_name()
        .expect("Can't get the exec name")
        .to_string_lossy()
        .into_owned();
    prog_name
}
*/

pub fn usage() {
    println!("\nInvalid arguments provided...\n");
    println!("USAGE:");
    println!("    ./cuaa [OPTION]\n");
    println!("OPTIONS:");
    println!("    --dry-run     Dry run - display files without deleting");
    println!("    --delete      Delete files\n");
}
