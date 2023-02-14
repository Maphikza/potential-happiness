use std::path::Path;
use std::io;
use std::fs::{self, DirEntry};

fn main() {
    let user_path = get_path();
    let start_path = Path::new(&user_path);
    println!("The path entered is {}", start_path.is_dir());
    visit_dirs(start_path, &print_dir_entry);
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

fn print_dir_entry(entry: &fs::DirEntry) {
    println!("{}", entry.path().display());
}

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}