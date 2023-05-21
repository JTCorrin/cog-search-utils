mod backup;
mod restore;
mod documents;

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
    pub workers: usize,

    /// Number of documents to obtain at a time
    #[structopt(long, default_value = "50")]
    pub cadence: usize,

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
pub struct DocumentCommand {
    /// The URL of the cognitive search service
    pub url: String,

    /// The API key required to access the service
    pub api_key: String,

    /// The name of the index
    pub index: String,

    /// The key of the document
    pub key: String,

    /// Fields to select
    #[structopt(long, default_value = "*")]
    pub select: String,
}

#[derive(StructOpt)]
enum Command {
    Backup(BackupCommand),
    Restore(RestoreCommand),
    Document(DocumentCommand),
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
        Command::Document(cmd) => documents::run(cmd),
    }
}
