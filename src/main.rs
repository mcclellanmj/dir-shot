extern crate clap;
extern crate walkdir;
extern crate csv;

#[macro_use]
extern crate serde_derive;

use clap::{Arg, App, SubCommand, ArgMatches};
use walkdir::WalkDir;
use std::path::PathBuf;
use std::io::stdout;
use std::time::SystemTime;
use std::collections::BTreeMap;
use std::borrow::Cow;

#[derive(Debug, Serialize, Deserialize)]
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

    for entry in WalkDir::new(args.value_of("directory").unwrap()) {
        let entry = entry.unwrap();
        let metadata = entry.metadata().unwrap();

        let file_snap = FileSnap {
            path: entry.path().to_owned(),
            modified: metadata.modified().unwrap().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
            size: metadata.len()
        };

        csv_writer.serialize(file_snap).expect("An error occurred while writing the row");
    }
}

fn run_compare(args: &ArgMatches) {
    let mut original_snap = BTreeMap::new();
    let mut reader = csv::Reader::from_path(args.value_of("listing1").unwrap()).unwrap();

    for result in reader.deserialize() {
        let record: FileSnap = result.unwrap();

        original_snap.insert(record.path.clone(), record);
    }
    unimplemented!("Not yet implemented");
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
        .subcommand(SubCommand::with_name("compare")
            .arg(Arg::with_name("listing1")
                .help("Older listing of files")
                .required(true)
                .index(1))
            .arg(Arg::with_name("listing2")
                .help("Newer listing of files")
                .required(true)
                .index(2))
            .about("Compare two previous outputs from this for differences")
        )
        .get_matches();

    match matches.subcommand() {
        ("record", Some(m)) => run_record(m),
        ("compare", Some(m)) => run_compare(m),
        _ => panic!("Subcommand not implemented")
    }

}
