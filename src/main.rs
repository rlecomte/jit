extern crate clap;

use clap::{App, Arg, SubCommand};
use std::process::exit;
use workspace::*;

mod workspace;

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
        .subcommand(SubCommand::with_name("commit").about("commit new content into the jit index."))
        .get_matches();

    exit(if let Some(matches) = matches.subcommand_matches("init") {
        let path = matches.value_of("path").unwrap_or(".");
        let w = Workspace::new(path);
        w.initialize().unwrap()
    } else if let Some(_) = matches.subcommand_matches("commit") {
        let w = Workspace::new(".");
        w.commit().unwrap()
    } else {
        1
    });
}
