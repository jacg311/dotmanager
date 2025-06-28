use std::{collections::{btree_map::Entry, HashMap}, env, fs::{self, File}, io::{self, BufRead, BufReader}, path::PathBuf};

use clap::{Parser, Subcommand};

use crate::rules::Rule;

mod rules;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        dir: Option<PathBuf>,
        //#[arg()]
        //no_git: bool

    },
    Apply {
        dir: Option<PathBuf>,
        #[arg()]
        force: bool,
    },
    CopyApply {
        dir: Option<PathBuf>,
        #[arg()]
        replace_symlinks: bool,
        #[arg()]
        replace_existing: bool,
    }
}

fn main() -> io::Result<()>{
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { dir } => {
            init(dir);
        },
        Commands::Apply { dir } => {
            apply(dir, false);
        }
        Commands::CopyApply { dir } => {
            apply(dir, true);
        }
    }

    let mut rules: HashMap<PathBuf, Vec<Rule>> = HashMap::new();

    let file = File::open("path")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        
    }

    Ok(())
}

fn init(dir: &Option<PathBuf>) {
    let dir = match dir {
        Some(buf) => buf,
        None => env::current_dir().unwrap()
    };
}

fn apply(dir: &Option<PathBuf>, copy_files: bool) {
    
}

fn walk(root: &PathBuf, entries: &mut Vec<PathBuf>) {
    for entry in root.read_dir() {
        
    }
}
