use std::path::Path;
use std::fs;

use clap::Parser;


#[derive(Parser)]
struct Args {
    /// Verbose mode on
    #[arg(short, long, action)]
    verbose: bool,

    /// Dry-run
    #[arg(short, long, action)]
    dry: bool,

    /// The file or folder not to delete
    keep: String,
}

fn rm_except(keep: std::path::PathBuf, verbose: bool, dry: bool) {
    let base_path = keep.parent().unwrap();

    let delete_candidates = fs::read_dir(base_path).unwrap();
    for p in delete_candidates {
        let delete_this = p.unwrap().path();

        if keep.to_str() == delete_this.to_str() {
            if verbose {
                println!("Keeping {}", keep.display());
                continue;
            }
        }

        if verbose || dry {
            println!("Deleting {}", delete_this.display());
        }
        if !dry {
            if delete_this.is_file() {
                fs::remove_file(delete_this).unwrap();
            }
            else if delete_this.is_dir() {
                fs::remove_dir_all(delete_this).unwrap();
            }
            else {
                println!("Could not determine type of entry {}, skipping deletion", delete_this.display());
            }
        }
    }
}

fn main() {
    let args = Args::parse();

    let item_to_keep = fs::canonicalize(Path::new(&args.keep));
    match item_to_keep {
        Ok(p) => rm_except(p, args.verbose, args.dry),
        Err(e) => println!("Error: {e:?}"),
    }

}
