use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

fn main() {
    let user_path = get_path();
    let start_path = Path::new(&user_path);
    println!("The path entered is {}", start_path.is_dir());
    let total_size = visit_dirs(start_path, &print_entry_size);
    println!("The total size for this directory is, {:#?}", total_size)
}

fn get_path() -> String {
    println!("Welcome to file size checker.\n Please enter the directory path.");

    let mut chosen_path = String::new();

    io::stdin()
        .read_line(&mut chosen_path)
        .expect("Could not readline.");

    let chosen_path: String = chosen_path
        .trim()
        .parse()
        .expect("This should be a path string");

    return chosen_path;
}

fn print_entry_size(entry: &fs::DirEntry) {
    // println!("{}", entry.path().display());
    let metadata = fs::metadata(entry.path());
    println!("{:#?}", metadata.expect("This must be a path.").len());
}

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<u64> {
    let mut size = 0;
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            size += fs::metadata(&path).expect("This should be a path.").len();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(size)
}
