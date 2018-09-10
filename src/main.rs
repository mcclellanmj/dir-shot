extern crate clap;
extern crate walkdir;
extern crate crossbeam_channel;
extern crate num_cpus;

use clap::{Arg, App};
use walkdir::WalkDir;
use crossbeam_channel as channel;
use std::thread;
use walkdir::DirEntry;
use std::thread::JoinHandle;
use std::fs::File;
use std::io::Read;
use std::str;
use std::time::SystemTime;

fn main() {
    let matches = App::new("Directory Capture")
        .version("0.0.1")
        .author("Matt McClellan <mcclellan.mj@gmail.com>")
        .arg(Arg::with_name("directory")
            .help("Directory to capture")
            .required(true)
            .index(1))
        .get_matches();

    let (file_sender, file_receiver): (channel::Sender<DirEntry>, channel::Receiver<DirEntry>) = channel::bounded(10000);

    let threads = (0..num_cpus::get() - 1).map(move |x| {
        let local_receiver = file_receiver.clone();

        let thread_builder = thread::Builder::new()
            .name(format!("Thread {}", x))
            .spawn(move || {
                loop {
                    if let Some(entry) = local_receiver.recv() {
                        let metadata = entry.metadata().unwrap();

                        println!("{}: [{}] - Modified: {:?}, Size: {}", thread::current().name().unwrap(),
                                 entry.path().display(),
                                 metadata.modified().unwrap().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
                                 metadata.len());
                    } else {
                        break;
                    }
                }
            });

        return thread_builder.unwrap();
    }).collect::<Vec<JoinHandle<()>>>();

    for entry in WalkDir::new(matches.value_of("directory").unwrap()) {
        let entry = entry.unwrap();
        file_sender.send(entry);
    }
}
