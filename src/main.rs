// Copy files from one folder to another
// Folders can be on the same or different network
// Folders can be on the same or different computer
// Folders can be nested
// Source and Destination should be configurable and read at runtime
extern crate chrono;

use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use std::thread::sleep;
use std::time::Duration;
use chrono::offset::Utc;
use chrono::DateTime;

struct Config {
    source: String,
    destination: String,
}

fn parse_config() -> Config {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        panic!("Usage: cp <source> <destination>\n");
    }
    Config {
        source: args[2].clone(),
        destination: args[3].clone(),
    }
}

fn copy_file(source: &Path, destination: &Path) {
    fs::copy(source, destination).unwrap();
}

fn copy_folder(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).unwrap();
    let walker = WalkDir::new(source).into_iter();
    let mut file_count = 0;
    for entry in walker.skip(1) {
        let entry = entry.unwrap();
        let path = entry.path();
        print!("Copying: {}\n", path.display());
        if path.is_file() {
            copy_file(&path, &destination.join(path.file_name().unwrap()));
            file_count += 1;
        } else if path.is_dir() {
            fs::create_dir_all(&destination.join(path.file_name().unwrap())).unwrap();
        }

    }

    println!("Files Copied {}", file_count);
}

fn copy_files(config: &Config) {
    let source = Path::new(&config.source);
    let destination = Path::new(&config.destination);
    if source.is_file() {
        copy_file(&source, &destination);
    } else if source.is_dir() {
        copy_folder(&source, &destination);
    } else {
        panic!("Source is not a file or directory\n");
    }
}


fn main() {
    let config = parse_config();

    // run the copy_files function every 60 seconds
    loop {
        // print start time hh:mm:ss
        let start = std::time::SystemTime::now();
        let start_utc: DateTime<Utc> = start.into();
        print!("Start: {}\n", start_utc.format("%H:%M:%S"));

        // copy files from source to destination
        copy_files(&config);

        // print elapsed time
        let end = std::time::SystemTime::now();
        let duration = end.duration_since(start).unwrap();
        print!("Elapsed: {} seconds\n", duration.as_secs());

        // print a message to the console
        println!("Copied files\n");

        // wait 15minutes
        sleep(Duration::new(900, 0));
    }    
}
