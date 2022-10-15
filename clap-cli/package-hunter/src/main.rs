use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use logger::DummyLogger;

mod logger;

#[derive(Parser, Debug)]
#[clap(author = "Author Name", version, about)]
/// A Very simple Package Hunter
struct Arguments {
    // #[clap(forbid_empty_values = true, validator=validate_package_name)]
    // package_name: String,
    #[clap(default_value_t=usize::MAX, short, long)]
    /// maximum depth for sub-directories searched
    max_depth: usize,

    #[clap(short, long, parse(from_occurrences))]
    verbosity: usize,

    #[clap(subcommand)]
    cmd: SubCommand,
}

#[derive(Parser, Debug)]
#[clap(author = "Author Name", version, about)]
/// A Very simple Package Hunter
struct ArgumentsOld {
    #[clap(forbid_empty_values = true, validator=validate_package_name)]
    package_name: String,

    #[clap(default_value_t=usize::MAX, short, long)]
    /// maximum depth for sub-directories searched
    max_depth: usize,

    #[clap(short, long, parse(from_occurrences))]
    verbosity: usize,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    Count {
        #[clap(forbid_empty_values = true, validator=validate_package_name)]
        /// Name of package to search
        package_name: String,
    },
    Projects {
        #[clap(short, long, default_value_t = String::from("."),
            forbid_empty_values = true, validator = validate_package_name)]
        /// directory to start exploring from
        start_path: String,
        #[clap(short, long, multiple_values = true, value_delimiter = ':')]
        /// paths to exclude when searching
        exclude: Vec<String>,
    },
}

/// count the number of sub-directories
///
/// count the number of sub-directories with the specified name
fn count(name: &str, max_depth: usize, logger: &DummyLogger) -> std::io::Result<usize> {
    let mut count = 0;
    // queue to store next dirs to explore
    let mut queue = VecDeque::new();
    // start with current dir
    queue.push_back((PathBuf::from("."), 0));
    loop {
        if queue.is_empty() {
            break;
        }
        let (path, current_depth) = queue.pop_back().unwrap();
        logger.debug(format!("path: {:?}, depth: {:?}", path, current_depth));
        if current_depth > max_depth {
            continue;
        }
        for dir_result in fs::read_dir(path)? {
            let dir = dir_result?;
            if dir.file_type()?.is_dir() {
                if dir.file_name() == name {
                    logger.log(format!("match found at {:?}", dir.path()));
                    count += 1;
                } else {
                    queue.push_back((dir.path(), current_depth + 1));
                }
            }
        }
    }
    Ok(count)
}

fn validate_package_name(name: &str) -> Result<(), String> {
    if name.trim().len() != name.len() {
        Err(String::from(
            "package name cannot have leading or trailing spaces",
        ))
    } else {
        Ok(())
    }
}

#[allow(dead_code)]
fn main_old() {
    let args = ArgumentsOld::parse();
    println!("{:?}", args);
    let logger = logger::DummyLogger::new(args.verbosity);
    match count(&args.package_name, args.max_depth, &logger) {
        Ok(c) => println!("{} users found for {}", c, args.package_name),
        Err(e) => println!("error in procesing : {}", e),
    };
}

#[allow(unused_variables)]
fn main() {
    let args = Arguments::parse();
    let logger = logger::DummyLogger::new(args.verbosity);
    match args.cmd {
        SubCommand::Count { package_name } => match count(&package_name, args.max_depth, &logger) {
            Ok(c) => println!("{} users found for {}", c, package_name),
            Err(e) => println!("error in procesing : {}", e),
        },
        SubCommand::Projects {
            start_path,
            exclude,
        } => println!("{} {:?}", start_path, exclude),
    }
}
