use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand, command};

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
    },
    Apply {
        dir: Option<PathBuf>,
        #[arg()]
        force: bool,
        #[arg()]
        dry_run: bool,
        #[arg()]
        copy_files: bool,
    },
    Collect {
        dir: Option<PathBuf>,
        #[arg()]
        force: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { dir } => init(dir),
        Commands::Apply {
            dir,
            force,
            dry_run,
            copy_files,
        } => apply(dir, *force, *dry_run, *copy_files),
        Commands::Collect { dir, force } => collect(dir, *force),
    }
}

fn init(dir: &Option<PathBuf>) {
    let cwd = &env::current_dir().unwrap();
    let dir = dir.as_ref().unwrap_or(cwd);

    // create basic config file, may replace with include_str() later
    let config_file = dir.join(".dotdef");

    if config_file.exists() {
        println!(
            "The file {} already exists. This looks like a dotfile repository already!",
            config_file.display()
        );
        return;
    }

    let _ = match File::create_new(config_file) {
        Ok(mut file) => file.write_all(b"~/.config"),
        Err(err) => {
            eprintln!("Couldn't create default config: {err}",);
            Ok(())
        }
    };
}

fn apply(dir: &Option<PathBuf>, overwrite_files: bool, dry_run: bool, copy_files: bool) {
    let cwd = env::current_dir().unwrap();
    let dir = match dir {
        Some(buf) => buf.as_path(),
        None => cwd.as_path(),
    };
    let rules = parse_rules(dir);
    for ele in rules {}
}

fn collect(dir: &Option<PathBuf>, force: bool) {}

fn parse_rules(path: &Path) -> Vec<Rule> {
    if !path.exists() {
        return Vec::new();
    }

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|line| match line {
            Ok(line) => Rule::parse(line),
            Err(_) => None,
        })
        .collect::<Vec<Rule>>()
}
