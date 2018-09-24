extern crate clap;
extern crate walkdir;
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use clap::{Arg, App, SubCommand, ArgMatches};
use walkdir::WalkDir;
use std::path::{Path, PathBuf};
use std::io::stdout;
use std::time::SystemTime;
use std::collections::BTreeMap;
use diesel::prelude::*;

embed_migrations!("migrations");

#[derive(Debug)]
struct FileSnap {
    path: PathBuf,
    modified: u64,
    size: u64,
    snapdate: u64
}

#[derive(Debug, PartialEq)]
enum FileStatus {
    Same,
    Changed,
    New,
    Deleted
}

fn print_walk_error(err: walkdir::Error) {
    eprintln!("Got an error");
}

fn run_capture(database: diesel::SqliteConnection, args: &ArgMatches) {
    unimplemented!("Capture not yet implemented")
    /*
    let mut csv_writer = csv::Writer::from_writer(stdout());

    let dir = WalkDir::new(args.value_of("directory").unwrap());

    for entry in dir {
        let entry = entry;

        match entry {
            Ok(entry) => {
                let metadata = entry.metadata().unwrap();

                if metadata.is_file() {
                    let file_snap = FlatSnap {
                        path: entry.path().to_owned(),
                        modified: metadata.modified().unwrap().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
                        size: metadata.len()
                    };

                    csv_writer.serialize(file_snap).expect("An error occurred while writing the row");
                }
            },
            Err(e) => print_walk_error(e)
        }

    }
    */
}

fn calculate_change(newest: &FileSnap, oldest_option: &Option<FileSnap>) -> FileStatus {
    if let Some(oldest) = oldest_option {
        if newest.modified != oldest.modified || newest.size != oldest.size {
            FileStatus::Changed
        } else {
            FileStatus::Same
        }
    } else {
        FileStatus::New
    }
}

fn print_status(show_same: bool, path: &Path, diff: &FileStatus) {
    let diff_str = match diff {
        FileStatus::Same => "S",
        FileStatus::Changed => "C",
        FileStatus::Deleted => "D",
        FileStatus::New => "N"
    };

    if FileStatus::Same != *diff || (FileStatus::Same == *diff && show_same) {
        println!("{} {}", diff_str, path.display());
    }
}

fn run_compare(args: &ArgMatches) {
    unimplemented!("Not implemented")
    /*
    let mut original_snap = BTreeMap::new();
    let show_same = args.is_present("show_same");
    {
        let mut reader = csv::Reader::from_path(args.value_of("listing1").unwrap()).unwrap();

        for result in reader.deserialize() {
            let record: FileSnap = result.unwrap();

            original_snap.insert(record.path, record.state);
        }
    }

    {
        let mut reader = csv::Reader::from_path(args.value_of("listing2").unwrap()).unwrap();

        for result in reader.deserialize() {
            let record: FileSnap = result.unwrap();
            let original_filepub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
} = original_snap.remove(&record.path);

            let diff = calculate_change(&record, &original_file);
            print_status(show_same, &record.path, &diff);
        }
    }

    for (key, _) in original_snap.iter() {
        print_status(show_same, key, &FileStatus::Deleted);
    }
    */
}

fn establish_connection(database_url: &str) -> diesel::SqliteConnection {
    SqliteConnection::establish(database_url).expect("Unable to open database")
}

fn main() {
    let matches = App::new("Directory Capture")
        .version("0.0.1")
        .author("Matt McClellan <mcclellan.mj@gmail.com>")
        .arg( Arg::with_name("database")
            .short("d")
            .help("Database")
            .value_name("FILE")
            .takes_value(true)
            .required(true))
        .subcommand(SubCommand::with_name("capture")
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
            .arg(Arg::with_name("show_same")
                .help("Show files that did not change")
                .short("s"))
            .about("Compare two previous outputs from this for differences"))
        .get_matches();

    let mut database = establish_connection(
        matches.value_of("database").unwrap());

    embedded_migrations::run_with_output(&database, &mut stdout());

    match matches.subcommand() {
        ("capture", Some(m)) => run_capture(database, m),
        ("compare", Some(m)) => run_compare(m),
        (x, _) => panic!("Subcommand [{}] is not implemented", x)
    }
}
