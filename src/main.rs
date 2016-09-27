extern crate git2;
#[macro_use] extern crate clap;
extern crate git_info;

use std::env;
use clap::{App, Arg};

use git_info::GitInfo;

fn main() {
    // Parse args
    let args = App::new("git-shell-info")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Easily print git info for your prompt")
        .arg(Arg::with_name("TEMPLATE")
             .help("The template to output")
             .required(true)
             .index(1))
        .get_matches();

    // Unwrap is safe because FORMAT is required, if it does not exist this line will never be reached
    let template = args.value_of("TEMPLATE").unwrap();

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

    let output = match git_info.format(template) {
        Ok(text) => text,
        Err(e) => panic!("Error: {}", e),
    };
    
    // Print it out
    print!("{}", output);
}
