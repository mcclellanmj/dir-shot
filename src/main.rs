extern crate clap;
extern crate walkdir;
extern crate csv;

use clap::{Arg, App, SubCommand, ArgMatches};
use walkdir::WalkDir;
use walkdir::DirEntry;
use std::fs::File;
use std::path::PathBuf;
use std::io::{Read, stdout};
use std::str;
use csv::Writer;
use std::time::SystemTime;

struct FileSnap {
    path: PathBuf,
    modified: u64,
    size: u64
}

fn as_row(file_snap: &FileSnap) -> Vec<String> {
    vec!(file_snap.path.display().to_string(),
         file_snap.modified.to_string(),
         file_snap.size.to_string()
    )
}

fn run_record(args: &ArgMatches) {
    let mut csv_writer = csv::Writer::from_writer(stdout());
    csv_writer.write_record(&["Path", "Modified", "Size"]);

    for entry in WalkDir::new(args.value_of("directory").unwrap()) {
        let entry = entry.unwrap();
        let metadata = entry.metadata().unwrap();

        let file_snap = FileSnap {
            path: entry.path().to_owned(),
            modified: metadata.modified().unwrap().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
            size: metadata.len()
        };

        csv_writer.write_record(as_row(&file_snap)).expect("An error occurred while writing the row");
    }
}

fn main() {
    let matches = App::new("Directory Capture")
        .version("0.0.1")
        .author("Matt McClellan <mcclellan.mj@gmail.com>")
        .subcommand(SubCommand::with_name("record")
            .arg(Arg::with_name("directory")
                .help("Directory to capture")
                .required(true)
                .index(1))
            .about("Record directory capture to stdout"))
        .get_matches();

    match matches.subcommand() {
        ("record", Some(m)) => run_record(m),
        _ => println!("Unknown command")
    }

}
