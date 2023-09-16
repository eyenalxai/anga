use std::{env, process};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = "10")]
    depth: usize,
}

fn main() {
    let args = Args::parse();
    let mut current_dir = env::current_dir().unwrap();
    let mut depth = 0;

    while depth <= args.depth {
        let mut venv_path = current_dir.clone();
        venv_path.push(".venv");

        if venv_path.is_dir() {
            venv_path.push("bin/activate.nu");
            println!("{}", venv_path.display());
            return;
        }

        if !current_dir.pop() {
            break;
        }

        depth += 1;
    }

    process::exit(1);
}
