use lazy_static::lazy_static;
use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;
use std::sync:: Mutex;

// This was a fun little test to improve my Rust programming skills. I have learned quite a bit from this little experiment.
// This function will print out the total size of the directory path you provide.

lazy_static! {
    static ref A: Mutex<u64> = Mutex::new(0);
}


fn main() {
    let user_path = get_path();
    let start_path = Path::new(&user_path);
    let _total_size = visit_dirs(start_path, &print_entry_size);
    let answer = A.lock().unwrap();
    println!("The total size for this directory is, {:#?}MB", answer.to_le()/1048576)
}

fn get_path() -> String {
    println!("Welcome to file size checker.\nPlease enter the directory path.");

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
    let mut mutex_guard = match A.try_lock() {
        Ok(guard) =>guard,
        Err(e) => {
            eprintln!("Failed to acquire lock: {:?}", e);
            return;
        }
    };
    *mutex_guard += &metadata.expect("This must be a path.").len();
    // println!("{:#?}", mutex_guard);
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
