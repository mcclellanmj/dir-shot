extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Directory Capture")
        .version("0.0.1")
        .author("Matt McClellan <mcclellan.mj@gmail.com>")
        .arg(Arg::with_name("directory")
            .help("Directory to capture")
            .required(true)
            .index(1))
        .get_matches();

    println!("Hello, world! {}", matches.value_of("directory").unwrap());
}
