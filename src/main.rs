extern crate clap;
extern crate walkdir;
extern crate rusqlite;
extern crate time;
extern crate cast;

use rusqlite::Connection;
use clap::{Arg, App, SubCommand, ArgMatches};
use walkdir::WalkDir;
use std::path::{Path, PathBuf};
use std::io::stdout;
use std::time::SystemTime;
use std::collections::BTreeMap;
use std::time::UNIX_EPOCH;
use cast::i64;

const CREATE_SQL: &'static str = include_str!("sql/create.sql");
const DIFF_SQL: &'static str = include_str!("sql/diff.sql");
const LATEST_SQL: &'static str = include_str!("sql/latest_record_dates.sql");

#[derive(Debug)]
struct FileSnap {
    path: String,
    modified: i64,
    size: i64,
    record_date: i64
}

fn print_walk_error(err: walkdir::Error) {
    eprintln!("Got an error on file {}", err.path().unwrap().display());
}

fn run_capture(connection: &mut Connection, args: &ArgMatches) {
    let dir = WalkDir::new(args.value_of("directory").unwrap());
    let start =
        i64(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs())
            .expect("Unable to cast from u64 to i64, date too large");
    let tx = connection.transaction().unwrap();

    for entry in dir {
        match entry {
            Ok(entry) => {
                let metadata = entry.metadata().unwrap();

                if metadata.is_file() {
                    let file_snap = FileSnap {
                        path: entry.path().display().to_string(),
                        modified: i64(metadata.modified().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs()).expect("Modified date too large to convert from u64 to i64"),
                        size: i64(metadata.len()).expect("Filesize too large to convert from unsigned to signed i64"),
                        record_date: start
                    };

                    tx.execute("INSERT INTO file_snaps (path, modified, size, record_date)\
                        VALUES (?, ?, ?, ?)",
                        &[&file_snap.path, &file_snap.modified, &file_snap.size, &file_snap.record_date]).unwrap();
                }
            },
            Err(e) => print_walk_error(e)
        }
    }

    tx.commit().unwrap();
}

fn find_dates(connection: &Connection) -> (i64, i64) {
    let mut statement = connection.prepare(LATEST_SQL).unwrap();
    let mut results = statement.query(&[]).unwrap();

    let first = results.next()
        .expect("Expected to have 2 dates in database but had 0.")
        .expect("Failed while executing query to find latest dates.")
        .get(0);

    let second = results.next()
        .expect("Expected to have 2 dates in database but only had 1.")
        .expect("Failed while executing query to find latest dates.")
        .get(0);

    (first, second)
}

fn run_compare(connection: &Connection, args: &ArgMatches) {
    let (first_date, second_date) = find_dates(connection);

    println!("Finding difference between dates [{}] and [{}]", first_date, second_date);

    let mut statement = connection.prepare(DIFF_SQL).unwrap();
    let mut results = statement.query(&[&first_date, &second_date]).unwrap();

    while let Some(result_row) = results.next() {
        let row = result_row.unwrap();
        let status: String = row.get(0);
        let file_name: String = row.get(1);
        println!("Status [{}], file [{}]", status, file_name);
    }
}

fn create_tables(connection: &Connection) {
    // FIXME: This needs to return an error instead of panicking
    let table_query : Result<(), rusqlite::Error> = connection.execute_batch(CREATE_SQL);

    match table_query {
        Ok(_) => println!("Table creation ok"),
        Err(e) => {
            println!("Get error {:?}", e);
            panic!("Failed to create table")
        }
    }
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
            .about("Compare two previous outputs from this for differences"))
        .subcommand( SubCommand::with_name("list-captures") )
            .about("Prints a list of all the dates captured")
        .get_matches();

    let mut connection =
        Connection::open(matches.value_of("database").unwrap()).unwrap();

    create_tables(&connection);

    match matches.subcommand() {
        ("capture", Some(m)) => run_capture(&mut connection, m),
        ("compare", Some(m)) => run_compare(&connection, m),
        (x, _) => panic!("Subcommand [{}] is not implemented", x)
    }

    connection.close().unwrap();
}
