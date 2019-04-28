extern crate clap;

use clap::{App, Arg, SubCommand};
use std::fs;
use std::fs::DirBuilder;
use std::process::exit;

fn main() {
    let matches = App::new("jit")
        .version("1.0")
        .author("Romain Lecomte")
        .about("A toy git program.")
        .subcommand(
            SubCommand::with_name("init")
                .about("initialize a jit repository.")
                .arg(
                    Arg::with_name("path")
                        .help("path of the new jit directory.")
                        .index(1),
                ),
        )
        .get_matches();

    exit(if let Some(matches) = matches.subcommand_matches("init") {
        let path = String::from(matches.value_of("path").unwrap_or("."));
        initialize(path)
    } else {
        1
    });
}

fn initialize(to_path: String) -> i32 {
    let p = fs::canonicalize(to_path).unwrap();
    println!("Initialize empty jit repo in {}", p.display());

    let mut builder = DirBuilder::new();
    builder.recursive(true);
    builder.create(p.join(".jit/objects")).unwrap();
    builder.create(p.join(".jit/refs")).unwrap();
    0
}
