//! rm_except: Small cmdline utility to remove all file system entries (i.e. files and folders)
//! except the given ones.

use std::fs;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(name = "rm_except")]
#[command(about = "Remove all entries in the current working directory except the given ones.", long_about = None)]
struct Args {
    /// Verbose mode on
    #[arg(short, long, action)]
    verbose: bool,

    /// Dry-run
    #[arg(short, long, action)]
    dry: bool,

    /// The files or folders not to delete
    keep: Vec<String>,
}

/// Returns a list of all entries in work_dir, optionally skipping hidden files
///
/// # Returns
/// A vector of PathBuf items
fn ls_all(work_dir: &PathBuf, skip_hidden: bool) -> Vec<PathBuf> {
    let mut all_files: Vec<PathBuf> = fs::read_dir(work_dir)
        .unwrap()
        .map(|p| fs::canonicalize(p.unwrap().path()).unwrap())
        .collect();

    if skip_hidden {
        all_files.retain(|e| !e.file_name().unwrap().to_str().unwrap().starts_with("."));
    }

    return all_files;
}

/// True if a provided entry is an entry in a given directory.
fn in_directory(parent_dir: &PathBuf, entry: &PathBuf) -> bool {
    if !parent_dir.is_absolute() || !entry.is_absolute() {
        panic!("Error in in_directory: Non-absolute path passed") // TODO nicer error handling
    }

    let all_files = ls_all(parent_dir, false);
    if !all_files.contains(entry) {
        return false;
    }

    return true;
}

/// Delete all entries in current working directory except some provided ones.
///
/// # Arguments
///
/// * `work_dir` - The working directory to delete in
/// * `keep` - List of entries to keep as absolute paths
/// * `verbose` - Be verbose about it
/// * `dry` - Print actions but do not actually perform any file system modifications
fn rm_except(work_dir: &PathBuf, keep: &Vec<PathBuf>, verbose: bool, dry: bool) {
    if keep.iter().any(|p| !p.is_absolute()) {
        panic!("Error in child_of_parents: Provided entries to keep not absolute")
        // TODO nicer error handling
    }
    if !work_dir.is_absolute() {
        panic!("not absolute");
    }

    let all_files = ls_all(work_dir, true);

    for entry in &all_files {
        if keep.contains(entry) {
            continue;
        }
        if verbose {
            println!("Deleting entry \"{}\"", entry.display());
        }
        if dry {
            continue;
        }
        if entry.is_file() {
            _ = fs::remove_file(entry);
        }
        else if entry.is_dir() {
            _ = fs::remove_dir_all(entry);
        }
        else {
            panic!("Neither file nor directory: {}", entry.display());
        }
    }
}

fn main() {
    let args = Args::parse();

    // Transform to absolute path objects
    let as_paths: Vec<PathBuf> = args
        .keep
        .iter()
        .map(|p| fs::canonicalize(PathBuf::from(p)).unwrap())
        .collect();

    // Check that all given entries are subentries of the cwd.
    for p in &as_paths {
        if !in_directory(&std::env::current_dir().unwrap(), p) {
            panic!(
                "Error: Provided entry {} is not at the current working directory",
                p.display()
            )
        }
    }

    rm_except(
        &std::env::current_dir().unwrap(),
        &as_paths,
        args.verbose,
        args.dry,
    );
}
