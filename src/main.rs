use clap::{Parser, Subcommand};

mod openai;
mod commands;

#[derive(Parser)]
#[command(name = "CommitSmith", author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Commit {
        #[arg(short, long)]
        diff_file: Option<String>,
    },
    Pr {
        #[arg(short, long)]
        diff_file: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Commit { diff_file } => {
            commands::commit::run(diff_file);
        }
        Commands::Pr { diff_file } => {
            commands::pr::run(diff_file);
        }
    }
}
