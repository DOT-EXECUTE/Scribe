use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    sub_commands: Subcommands,
}

#[derive(Subcommand)]
enum Subcommands {
    New(NewArgs),
}

// Command Flags
#[derive(Args)]
struct NewArgs {
    name: String,
    #[arg(short = 'o', long = "output")]
    output: Option<PathBuf>,
}

// === SUBCOMMANDS ===

mod new {
    use crate::{Cli, NewArgs, Subcommands};
    use clap::Parser;
    use dirs::home_dir;
    use std::{
        fs,
        fs::{DirBuilder, File},
        io,
        path::{Path, PathBuf},
    };

    fn get_args_new(cli: Cli) -> NewArgs {
        match cli.sub_commands {
            Subcommands::New(args) => args,
        }
    }

    pub fn new_note() {
        let default_directory = home_dir()
            .expect("Could not find home directory")
            .join(".scribe/notes");

        let cli = Cli::parse();
        let arguments = get_args_new(cli);

        if let Err(e) = check_path_no_file(
            arguments.output.as_deref().unwrap_or(&default_directory),
            &arguments,
        ) {
            return eprint!("ERROR: {}", e);
        }

        let output_path: &PathBuf = if let Some(ref path) = arguments.output {
            path
        } else {
            &default_directory
        };

        if !default_directory.exists() && output_path == &default_directory {
            DirBuilder::new()
                .recursive(true)
                .create(&default_directory)
                .unwrap();
        }

        let _file = file_write(output_path, &arguments);
    }

    fn file_write(path: &Path, arguments: &NewArgs) -> Result<File, io::Error> {
        let mut file: PathBuf = path.join(&arguments.name);
        let mut highest_number: u16 = 1; // Junk value to init the var

        for entry in fs::read_dir(path)? {
            let entry = entry.unwrap();
            let entry_name = entry.file_name();
            let entry_str = entry_name.to_string_lossy();
            let stem = entry_str.trim_end_matches(".txt");

            if !stem.starts_with(arguments.name.as_str()) {
                continue;
            }

            let current_number = stem
                .trim_start_matches(arguments.name.as_str())
                .trim()
                .parse::<u16>()
                .unwrap_or(1);

            highest_number = highest_number.max(current_number);

            file = path.join(format!("{}{}", arguments.name, highest_number + 1));
        }

        // update the path to include the name and extension of the note
        File::create(file.with_extension("txt"))
    }

    fn check_path_no_file<'a>(path: &Path, arguments: &NewArgs) -> Result<(), &'a str> {
        // Early return if no arguments
        let Some(ref _output) = arguments.output else {
            return Ok(());
        };

        for path_ancestor in path.ancestors() {
            if path_ancestor.is_dir() {
                continue;
            } else {
                return Err("Path includes a non directory");
            }
        }

        Ok(())
    }
}

mod open_note {}

mod edit_note {}

mod delete_note {}

fn main() {
    let cli = Cli::parse();

    match &cli.sub_commands {
        Subcommands::New(NewArgs) => new::new_note(),
    }
}
