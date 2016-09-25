extern crate git2;
#[macro_use] extern crate clap;
#[macro_use] extern crate quick_error;

mod git;

use std::env;
use clap::{App, Arg};

use git::GitInfo;

// Replaces the tags with info from git
fn replace(git: git::GitInfo, format_string: String) -> String {
    format_string.replace("%b",
                          git.branch_current().unwrap().as_str())
}

fn main() {
    // Parse args
    let args = App::new("git-shell-info")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Easily print git info for your prompt")
        .arg(Arg::with_name("FORMAT")
             .help("The output format")
             .required(true)
             .index(1))
        .get_matches();

    // Unwrap is safe because FORMAT is required, if it does not exist this line will never be reached
    let format = String::from(args.value_of("FORMAT").unwrap());

    // Get the current working directory
    let cwd = match env::current_dir() {
        Ok(path) => path,
        Err(e) => panic!("Error: {}", e),
    };

    // Make a GitInfo instance
    let git_info = match GitInfo::new(cwd) {
        Ok(gi) => gi,
        Err(e) => panic!("Error: {}", e),
    };

    // Replace the format string
    let output_string = replace(git_info, format);

    // Print it out
    print!("{}", output_string);
}
