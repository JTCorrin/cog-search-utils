mod backup;
mod restore;

use structopt::StructOpt;

#[derive(StructOpt)]
pub struct BackupCommand {
    /// The URL of the cognitive search service
    pub url: String,

    /// The API key required to access the service
    pub api_key: String,

    /// The name of the index to backup
    pub index: String,

    /// Number of concurrent threads to run to perform the backup
    #[structopt(long, default_value = "1")]
    pub workers: u8,

    /// Number of documents to obtain at a time
    #[structopt(long, default_value = "50")]
    pub cadence: u8,

    /// Where to save the backups
    #[structopt(long, default_value = "./")]
    pub file: String,
}

#[derive(StructOpt)]
pub struct RestoreCommand {
    /// The URL of the cognitive search service
    pub url: String,

    /// The API key required to access the service
    pub api_key: String,

    /// The name of the index to restore
    pub index: String,

    /// Where the backup files are located
    #[structopt(long, default_value = "./")]
    pub file: String,
}

#[derive(StructOpt)]
enum Command {
    Backup(BackupCommand),
    Restore(RestoreCommand),
}

#[derive(StructOpt)]
#[structopt(name = "azure_search_backup")]
struct Opt {
    #[structopt(subcommand)]
    command: Command,
}

fn main() {
    let opt = Opt::from_args();

    match opt.command {
        Command::Backup(cmd) => backup::run(cmd),
        Command::Restore(cmd) => restore::run(cmd),
    }
}
