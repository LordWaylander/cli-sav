use clap::{Parser, Subcommand};

mod scan;
mod save;
mod restore;

/// CLI-tools to save some files or folders ine a disk or a directory


#[derive(Subcommand)]
enum CommandList {
    /// Scan the disk
    Scan,
   
    Save(crate::save::Args),
    /// restore disk, folder or files
    Restore(crate::restore::Args)
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli{
    #[command(subcommand)]
    command: CommandList
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        CommandList::Scan => scan::main(),
        CommandList::Save(options) => save::main(options),
        CommandList::Restore(options) => restore::main(options),
    }
}